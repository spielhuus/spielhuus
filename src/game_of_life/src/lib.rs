use std::{iter, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use log::info;

use rand::prelude::*;
use wgpu::util::DeviceExt;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

const CELL_SIZE: f32 = 10.0;
const WORKGROUP_SIZE: u32 = 8;
const BOOL_RAND: f64 = 0.6;
#[cfg(target_arch = "wasm32")]
const CANVAS_ID: &str = "shader";

#[derive(Debug)]
pub enum UserEvent {
    BirthChanged(u32),
    SurviveChanged(u32),
    SizeChanged(u32),
    // CodeChanged(String),
    State(Box<State>),
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    resolution: [f32; 2],
    grid_size: [f32; 2],
    time: f32,
    birth_rule: u32,
    survive_rule: u32, 
    _padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// -1  1                        1,  1
// -1 -1                        1, -1

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
];

const INDICES: &[u16] = &[1, 0, 3, 0, 2, 3];

fn rule_bitmask(rule_num: u32) -> u32 {
    let mut mask = 0u32;
    let mut n = rule_num;
    if n == 0 {
        mask |= 1 << 0;
        return mask;
    }
    while n > 0 {
        let digit = n % 10;
        // Game of Life only considers up to 8 neighbors
        if digit <= 8 {
            mask |= 1 << digit;
        }
        n /= 10;
    }
    mask
}

// This will store the state of our game
#[derive(Debug)]
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    simulation_pipeline: wgpu::ComputePipeline,
    window: Arc<Window>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    uniform_bind_group: [wgpu::BindGroup; 2],
    cell_state_buffer: [wgpu::Buffer; 2],
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    #[cfg(target_arch = "wasm32")]
    canvas: HtmlCanvasElement,
    step: usize,
    cell_size: [f32; 2],
    grid_size: [f32; 2],
    birth_rule: u32,
    survive_rule: u32,
}

impl State {
    async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
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
            .request_device(&wgpu::DeviceDescriptor::default()).await?;

        let surface_caps = surface.get_capabilities(&adapter);

        info!("SurfaceCapabillities: {:#?}", surface_caps);

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
            alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        //storage buffer

        let cell_size = [CELL_SIZE, CELL_SIZE];
        let grid_size = [
            size.width as f32 / cell_size[0],
            size.height as f32 / cell_size[1],
        ];
        let cells = grid_size[0] * grid_size[1];
        println!("screen size: {}x{}", size.width, size.height);
        println!("cell size: {}x{}", cell_size[0], cell_size[1]);
        println!("grid_size: {}x{}", grid_size[0], grid_size[1]);
        println!("cells: {}", cells);

        let mut rng = rand::rng();
        let cell_state_array_a: Vec<u32> = (0..cells as usize)
            .map(|_| if rng.random_bool(BOOL_RAND) { 1 } else { 0 })
            .collect();
        let cell_state_array_b: Vec<u32> = (0..cells as usize)
            .map(|_| if rng.random_bool(BOOL_RAND) { 1 } else { 0 })
            .collect();

        let cell_state_buffer = [
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cell State Storage A"),
                contents: bytemuck::cast_slice(&cell_state_array_a),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cell State Storage B"),
                contents: bytemuck::cast_slice(&cell_state_array_b),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
        ];

        // create the uniform
        let uniforms = Uniforms {
            resolution: [size.width as f32, size.height as f32],
            grid_size: [
                size.width as f32 / cell_size[0],
                size.height as f32 / cell_size[1],
            ],
            time: 0.0,
            birth_rule: rule_bitmask(3),
            survive_rule: rule_bitmask(23),
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
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let uniform_bind_group = [
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group A"),
                layout: &uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: cell_state_buffer[0].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: cell_state_buffer[1].as_entire_binding(),
                    },
                ],
            }),
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group B"),
                layout: &uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: cell_state_buffer[1].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: cell_state_buffer[0].as_entire_binding(),
                    },
                ],
            }),
        ];

        //create the pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
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
                cull_mode: Some(wgpu::Face::Back),
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

        let simulation_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Simulation pipeline"),
                layout: Some(&render_pipeline_layout),
                module: &shader,
                entry_point: Some("compute_main"),
                cache: None,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            });
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        surface.configure(&device, &config);

        #[cfg(target_arch = "wasm32")]
        let canvas = {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
            canvas
        };

        Ok(Self {
            // start_time,
            surface,
            device,
            queue,
            config,
            is_surface_configured: true,
            render_pipeline,
            simulation_pipeline,
            window,
            vertex_buffer,
            uniform_bind_group_layout,
            uniform_buffer,
            uniform_bind_group,
            cell_state_buffer,
            index_buffer,
            num_indices,
            #[cfg(target_arch = "wasm32")]
            canvas,
            step: 0,
            cell_size,
            grid_size,
            birth_rule: rule_bitmask(3),
            survive_rule: rule_bitmask(23),
        })
    }

    fn recreate_simulation(&mut self) {
        let size = self.window.inner_size();
        self.grid_size = [
            (size.width as f32 / self.cell_size[0]).ceil(),
            (size.height as f32 / self.cell_size[1]).ceil(),
        ];
        let cells = (self.grid_size[0] * self.grid_size[1]) as usize;

        info!(
            "Recreating simulation with grid size: {}x{}",
            self.grid_size[0], self.grid_size[1]
        );

        // Re-create the cell data and buffers
        let mut rng = rand::rng();
        let cell_state_array_a: Vec<u32> = (0..cells)
            .map(|_| if rng.random_bool(BOOL_RAND) { 1 } else { 0 })
            .collect();
        let cell_state_array_b: Vec<u32> = (0..cells)
            .map(|_| if rng.random_bool(BOOL_RAND) { 1 } else { 0 })
            .collect();

        self.cell_state_buffer = [
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Cell State Storage A"),
                    contents: bytemuck::cast_slice(&cell_state_array_a),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                }),
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Cell State Storage B"),
                    contents: bytemuck::cast_slice(&cell_state_array_b),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                }),
        ];

        // Re-create the bind groups with the new buffers
        self.uniform_bind_group = [
            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group A"),
                layout: &self.uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.cell_state_buffer[0].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.cell_state_buffer[1].as_entire_binding(),
                    },
                ],
            }),
            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group B"),
                layout: &self.uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.cell_state_buffer[1].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.cell_state_buffer[0].as_entire_binding(),
                    },
                ],
            }),
        ];
    }

    fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        info!("resize window: {width}x{height}");
        if width == 0 || height == 0 {
            self.is_surface_configured = false; // can't render when minimized/zero
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.recreate_simulation();
        self.is_surface_configured = true;
        self.window.request_redraw();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            info!("render: surface not configured");
            return Ok(());
        }

        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Cellular Automaton Compute Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.simulation_pipeline);
            compute_pass.set_bind_group(0, &self.uniform_bind_group[self.step % 2], &[]);
            // let workgroup_count_x = self.grid_width.div_ceil(64);
            //
            let workgroup_count_x =
                (self.grid_size[0] as u32 + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
            let workgroup_count_y =
                (self.grid_size[1] as u32 + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
            compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
        }

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
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let updated_uniforms = Uniforms {
                resolution: [
                    self.window.inner_size().width as f32,
                    self.window.inner_size().height as f32,
                ],
                grid_size: self.grid_size,
                time: 0.0, // TODO
                birth_rule: self.birth_rule,
                survive_rule: self.survive_rule,
                _padding: 0,
            };
            self.queue.write_buffer(
                &self.uniform_buffer,
                0,
                bytemuck::cast_slice(&[updated_uniforms]),
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.uniform_bind_group[self.step % 2], &[]);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            self.step += 1;
        }

        self.queue.submit(iter::once(encoder.finish()));
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
    proxy: winit::event_loop::EventLoopProxy<UserEvent>,
    state: Option<Box<State>>,
    #[cfg(target_arch = "wasm32")]
    _event_closures: Vec<Closure<dyn FnMut(web_sys::Event)>>,
}

#[allow(clippy::new_without_default)]
impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<UserEvent>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = event_loop.create_proxy();
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
            #[cfg(target_arch = "wasm32")]
            _event_closures: Vec::new(),
        }
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            // let canvas = {
            // Use a block to scope variables
            use wasm_bindgen::JsCast;
            use web_sys::HtmlInputElement;
            use winit::platform::web::WindowAttributesExtWebSys;

            let proxy = self.proxy.clone();
            let on_birth_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlInputElement>() {
                    let value_str = input_element.value();
                    if let Ok(birth) = value_str.parse::<u32>() {
                        proxy.send_event(UserEvent::BirthChanged(birth)).unwrap();
                    }
                }
            });

            let proxy = self.proxy.clone();
            let on_survive_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlInputElement>() {
                    let value_str = input_element.value();
                    if let Ok(survive) = value_str.parse::<u32>() {
                        proxy
                            .send_event(UserEvent::SurviveChanged(survive))
                            .unwrap();
                    }
                }
            });

            let proxy = self.proxy.clone();
            let on_size_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlInputElement>() {
                    let value_str = input_element.value();
                    if let Ok(size) = value_str.parse::<u32>() {
                        proxy.send_event(UserEvent::SizeChanged(size)).unwrap();
                    }
                }
            });

            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let birth_input = document
                .get_element_by_id("birth")
                .expect("should have an input with id 'birth'");
            let birth_input: HtmlInputElement = birth_input.dyn_into().map_err(|_| ()).unwrap();
            birth_input
                .add_event_listener_with_callback(
                    "input",
                    on_birth_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_birth_callback);

            let survive_input = document
                .get_element_by_id("survive")
                .expect("should have an input with id 'survive'");
            let survive_input: HtmlInputElement = survive_input.dyn_into().map_err(|_| ()).unwrap();
            survive_input
                .add_event_listener_with_callback(
                    "input",
                    on_survive_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_survive_callback);

            let size_input = document
                .get_element_by_id("size")
                .expect("should have an input with id 'size'");
            let size_input: HtmlInputElement = size_input.dyn_into().map_err(|_| ()).unwrap();
            size_input
                .add_event_listener_with_callback(
                    "input",
                    on_size_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_size_callback);

            // let code_input = document
            //     .get_element_by_id("code")
            //     .expect("should have an input with id 'code'");
            // let code_input: HtmlInputElement = code_input.dyn_into().map_err(|_| ()).unwrap();
            // code_input
            //     .add_event_listener_with_callback(
            //         "input",
            //         on_code_callback.as_ref().unchecked_ref(),
            //     )
            //     .unwrap();
            // self._event_closures.push(on_code_callback);

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id("shader").unwrap();
            let canvas: web_sys::HtmlCanvasElement =
                canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

            let mut attributes = Window::default_attributes().with_canvas(Some(canvas));
            attributes.active = false;

            let window = Arc::new(event_loop.create_window(attributes).unwrap());
            let proxy = self.proxy.clone();
            wasm_bindgen_futures::spawn_local(async move {
                assert!(
                    proxy
                        .send_event(UserEvent::State(Box::new(
                            State::new(window)
                                .await
                                .expect("Unable to create canvas!!!")
                        )))
                        .is_ok()
                )
            });
        };

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state = Some(Box::new(pollster::block_on(State::new(window)).unwrap()));
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: UserEvent) {
        match event {
            UserEvent::BirthChanged(new_birth) => {
                info!("Handling BirthChanged event. New birth: {}", new_birth);
                if let Some(state) = &mut self.state {
                    state.birth_rule = rule_bitmask(new_birth);
                    state.recreate_simulation();
                }
            }
            UserEvent::SurviveChanged(new_survive) => {
                info!(
                    "Handling SurviveChanged event. New survive: {}",
                    new_survive
                );
                if let Some(state) = &mut self.state {
                    state.survive_rule = rule_bitmask(new_survive);
                    state.recreate_simulation();
                }
            },
            UserEvent::SizeChanged(new_size) => {
                info!(
                    "Handling SizeChanged event. New size: {}",
                    new_size
                );
                if let Some(state) = &mut self.state {
                    state.cell_size = [new_size as f32, new_size as f32];
                    state.recreate_simulation();
                }
            },
            // UserEvent::CodeChanged(new_code) => {
            //     info!(
            //         "Handling CodeChanged event. New script: {}",
            //         new_code
            //     );
            //     let result = run_scheme(&new_code);
            //     info!("scheme result: {:?}", result);
            // }

            #[cfg(target_arch = "wasm32")]
            UserEvent::State(mut state) => {
                info!("Handling StateCreated event.");
                {
                    let size = state.window.inner_size();
                    info!("Resizing surface to {}x{}", size.width, size.height);
                    state.resize(size.width, size.height);

                    self.state = Some(state);
                    if let Some(s) = &self.state {
                        info!("set state");
                        s.window.request_redraw();
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            _ => {}
        }
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
                state.render().unwrap();
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
