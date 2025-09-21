use std::{sync::Arc, time::Instant};

use nalgebra::{Point2, Vector2};

use rand::Rng;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use log::info;

use wgpu::util::DeviceExt;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

const EPSILON: f32 = 1e-6;
const SPEED: f32 = 0.002;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuVertex {
    position: [f32; 2],
}

impl GpuVertex {
    fn desc() -> &'static [wgpu::VertexAttribute] {
        &[wgpu::VertexAttribute {
            offset: 0,
            format: wgpu::VertexFormat::Float32x2,
            shader_location: 0,
        }]
    }
}

pub fn generate_sperm_shape_spine(
    num_points: usize,
    length: f32,
    initial_amplitude: f32,
    frequency: f32,
    damping_factor: f32,
) -> Vec<f32> {
    let mut points = Vec::with_capacity(num_points);

    for i in 0..num_points {
        // 1. Calculate the progress along the spine from 0.0 (head) to 1.0 (tail).
        let progress = i as f32 / (num_points - 1) as f32;

        // 2. Calculate the decay factor using an exponential function.
        // It starts at 1.0 (e^0) and smoothly goes towards 0.
        let decay = (-progress * damping_factor).exp();

        // 3. Calculate the sine wave oscillation.
        // We multiply by TAU (2*PI) so `frequency` is the number of full cycles.
        let oscillation = (progress * frequency * std::f32::consts::TAU).sin();

        // 4. Combine everything to get the final y-offset.
        let y_offset = initial_amplitude * decay * oscillation;
        let x_pos = progress * length;

        points.push(y_offset);
    }

    points
}

/// Manually tessellates a worm's spine and thickness into a triangle mesh.
///
/// This function replaces the logic previously handled by `lyon`.
///
/// # Arguments
/// * `visual_points` - The center-line of the worm after wiggling has been applied.
/// * `half_thickness` - A `Vec` containing the half-width of the worm at each point.
///
/// # Returns
/// A tuple containing `(vertices, indices)` ready for `wgpu`.
fn tessellate_worm_manually(
    visual_points: &[Point2<f32>],
    half_thickness: &[f32],
) -> (Vec<GpuVertex>, Vec<u32>) {
    // Handle edge case where the worm is too short to draw.
    if visual_points.len() < 2 {
        return (Vec::new(), Vec::new());
    }

    let mut vertices = Vec::with_capacity(visual_points.len() * 2);
    let mut indices = Vec::with_capacity((visual_points.len() - 1) * 6);

    // --- 1. Generate all the vertices for the outline ---
    for i in 0..visual_points.len() {
        let p = visual_points[i];
        let thickness = half_thickness[i];

        // Calculate the direction vector at this point to find the normal.
        // For smoother normals, we look at the points before and after.
        let dir = if i == 0 {
            // First point: look forward
            (visual_points[i + 1] - p)
                .try_normalize(EPSILON)
                .unwrap_or(Vector2::zeros())
        } else if i == visual_points.len() - 1 {
            // Last point: look backward
            (p - visual_points[i - 1])
                .try_normalize(EPSILON)
                .unwrap_or(Vector2::zeros())
        } else {
            // Middle points: use the average direction for smoothness
            (visual_points[i + 1] - visual_points[i - 1])
                .try_normalize(EPSILON)
                .unwrap_or(Vector2::zeros())
        };

        // The normal is perpendicular to the direction vector.
        let normal = Vector2::new(-dir.y, dir.x);

        // Create top and bottom vertices by moving along the normal.
        let top_vertex = p + normal * thickness;
        let bottom_vertex = p - normal * thickness;

        vertices.push(GpuVertex {
            position: [top_vertex.x, top_vertex.y],
        });
        vertices.push(GpuVertex {
            position: [bottom_vertex.x, bottom_vertex.y],
        });
    }

    // We create a strip of quads, where each quad is two triangles.
    for i in 0..(visual_points.len() - 1) {
        // The vertices are interleaved in the buffer: [top0, bottom0, top1, bottom1, ...]
        let top_left_idx = (i * 2) as u32;
        let bottom_left_idx = (i * 2 + 1) as u32;
        let top_right_idx = ((i + 1) * 2) as u32;
        let bottom_right_idx = ((i + 1) * 2 + 1) as u32;

        // First triangle of the quad
        indices.push(bottom_left_idx);
        indices.push(top_right_idx);
        indices.push(top_left_idx);

        // Second triangle of the quad
        indices.push(bottom_left_idx);
        indices.push(bottom_right_idx);
        indices.push(top_right_idx);
    }

    (vertices, indices)
}

mod worm_logic {

    use super::{EPSILON, Point2, Vector2};

    pub struct Worm {
        pub spine_points: Vec<Point2<f32>>,
        segment_length: f32,
        speed: f32,
        time: f32,
        wiggle_speed: f32,
        wiggle_frequency: f32,
        wiggle_amplitude: f32,
    }

    impl Worm {
        pub fn new() -> Self {
            let num_segments = 20;
            let segment_length = 0.01;
            let start_pos = Point2::new(0.0, 0.0);
            let mut spine_points = Vec::with_capacity(num_segments);
            for i in 0..num_segments {
                spine_points.push(Point2::new(
                    start_pos.x - i as f32 * segment_length,
                    start_pos.y,
                ));
            }

            Self {
                spine_points,
                segment_length,
                speed: 3.0,
                time: 0.0,
                wiggle_speed: 20.0,
                wiggle_frequency: 20.0,
                wiggle_amplitude: 0.1,
            }
        }

        pub fn update(&mut self, dt: f32, target: Point2<f32>) {
            self.time += dt * self.wiggle_speed;
            let head = &mut self.spine_points[0];

            *head = target;

            for i in 1..self.spine_points.len() {
                let leader = self.spine_points[i - 1];
                let follower = &mut self.spine_points[i];
                let dir_to_leader = (leader - *follower)
                    .try_normalize(EPSILON)
                    .unwrap_or(Vector2::zeros());
                *follower = leader - dir_to_leader * self.segment_length;
            }
        }

        pub fn get_render_data(&self) -> (Vec<Point2<f32>>, Vec<f32>) {
            // This is the logic that calculates the wiggled spine
            let mut visual_points = Vec::with_capacity(self.spine_points.len());
            for i in 0..self.spine_points.len() {
                let p1 = self.spine_points[i];
                let dir = if i < self.spine_points.len() - 1 {
                    (self.spine_points[i + 1] - p1)
                        .try_normalize(EPSILON)
                        .unwrap_or(Vector2::zeros())
                } else {
                    (p1 - self.spine_points[i - 1])
                        .try_normalize(EPSILON)
                        .unwrap_or(Vector2::zeros())
                };
                let normal = Vector2::new(-dir.y, dir.x);
                let progress = i as f32 / (self.spine_points.len() - 1) as f32;
                let wiggle_factor = (self.time + i as f32 * (1.0 / self.wiggle_frequency)).sin();
                let current_amplitude = self.wiggle_amplitude * progress.powf(1.5);
                let offset = normal * current_amplitude * wiggle_factor;
                visual_points.push(p1 + offset);
            }

            // This is the logic that calculates the variable thickness
            let half_thickness = crate::generate_sperm_shape_spine(
                visual_points.len(),
                visual_points.len() as f32,
                0.05,
                0.5,
                7.0,
            );

            (visual_points, half_thickness)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    resolution: [f32; 2],
    time: f32,
    _padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

// This will store the state of our game
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    window: Arc<Window>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    #[cfg(not(target_arch = "wasm32"))]
    start_time: std::time::Instant,
    #[cfg(target_arch = "wasm32")]
    start_time: f64,
    #[cfg(target_arch = "wasm32")]
    canvas: HtmlCanvasElement,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    worm: worm_logic::Worm,
    mouse_pos: Point2<f32>,
    last_update: Instant,
    path: Vec<f32>,
    step: usize,
}

impl State {
    async fn new(
        window: Arc<Window>,
        #[cfg(target_arch = "wasm32")] canvas: HtmlCanvasElement,
    ) -> anyhow::Result<State> {
        #[cfg(not(target_arch = "wasm32"))]
        let start_time = std::time::Instant::now();

        #[cfg(target_arch = "wasm32")]
        let start_time = {
            use web_sys::window;
            window().unwrap().performance().unwrap().now()
        };

        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        info!("InstanceDescriptor: {:#?}", instance);

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        info!("AdapterDescriptor: {:#?}", adapter);

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        info!("SurfaceCapabillities: {:#?}", surface_caps);

        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // create the uniform
        let uniforms = Uniforms {
            resolution: [0.0, 0.0],
            time: 0.0,
            _padding: 0,
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GpuVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: GpuVertex::desc(),
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // --- ADD THIS CODE RIGHT BEFORE THE `Self` RETURN BLOCK ---
        let max_vertices = 10000;
        let max_indices = 10000;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (max_vertices * std::mem::size_of::<GpuVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: (max_indices * std::mem::size_of::<u32>()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // fill the random path
        let mut path: Vec<f32> = Vec::new();
        let mut rng = rand::rng();
        let mut old_angle = rng.random_range(0.0..std::f32::consts::TAU);
        for i in 0..1000 {
            path.push(old_angle);
            old_angle += rng.random_range(-std::f32::consts::PI / 2.0..std::f32::consts::PI / 2.0);
        }

        Ok(Self {
            start_time,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
            window,
            uniform_buffer,
            uniform_bind_group,
            #[cfg(target_arch = "wasm32")]
            canvas,

            vertex_buffer,
            index_buffer,
            num_indices: 0,
            worm: worm_logic::Worm::new(),
            mouse_pos: Point2::new(0.0, 0.0),
            last_update: Instant::now(),

            path,
            step: 0,
        })
    }

    fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        let angle = &self.path[self.step];
        let mut movement = Vector2::new(angle.cos() * SPEED, angle.sin() * SPEED);

        let new_pos = self.mouse_pos + movement;
        if new_pos.x < -1.0 || new_pos.x > 1.0 || new_pos.y < -1.0 || new_pos.y > 1.0 {
            let angle = self.path[self.step] - std::f32::consts::PI;
            movement = Vector2::new(angle.cos() * SPEED, angle.sin() * SPEED);
            self.mouse_pos = Point2::new(0.0, 0.0);
        } else {
            self.mouse_pos += movement;
        }

        println!(
            "new step:{} angle: {} pos:{}",
            self.step, self.path[self.step], self.mouse_pos
        );
        self.worm.update(dt, self.mouse_pos);

        self.step = (self.step + 1) % self.path.len();

        let (visual_points, half_thickness) = self.worm.get_render_data();
        let (vertices, indices) = tessellate_worm_manually(&visual_points, &half_thickness);
        self.num_indices = indices.len() as u32;
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        self.queue
            .write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    fn toggle_fullscreen(&self) {
        use web_sys::console;

        // Get the document from the global window object
        let document = web_sys::window().unwrap().document().unwrap();

        // Check if we are currently in fullscreen mode
        if document.fullscreen_element().is_none() {
            // If not, request fullscreen on our canvas
            console::log_1(&"Entering fullscreen".into());
            self.canvas
                .request_fullscreen()
                .expect("Failed to enter fullscreen mode");
        } else {
            // If we are, exit fullscreen
            console::log_1(&"Exiting fullscreen".into());
            document.exit_fullscreen();
        }
    }
}

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "shader";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        #[cfg(target_arch = "wasm32")]
        let canvas = {
            // Use a block to scope variables
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "shader";

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

            window_attributes = window_attributes.with_canvas(Some(canvas.clone()));
            canvas
        };

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(
                        proxy
                            .send_event(
                                State::new(window, canvas)
                                    .await
                                    .expect("Unable to create canvas!!!")
                            )
                            .is_ok()
                    )
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                state.render().unwrap();
                state.window.as_ref().request_redraw();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                #[cfg(target_arch = "wasm32")]
                if code == KeyCode::F11 && key_state.is_pressed() {
                    state.toggle_fullscreen();
                }
                state.handle_key(event_loop, code, key_state.is_pressed());
            }
            WindowEvent::CursorMoved { position, .. } => {
                // let mouse_pos = Point2::new(
                //     (position.x as f32 / state.config.width as f32) * 2.0 - 1.0,
                //     -((position.y as f32 / state.config.height as f32) * 2.0 - 1.0),
                // );
                //
                // let d = mouse_pos - state.mouse_pos;
                // state.mouse_pos += d;
                //
                // state.window.request_redraw();
            }
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    info!("run");
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap_throw();

    let event_loop = EventLoop::with_user_event().build().unwrap_throw();
    let mut app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    event_loop.run_app(&mut app).unwrap_throw();
    Ok(())
}
