use std::{iter, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use log::info;
use rand::Rng;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

const RULE: u32 = 90;
const SIZE: u32 = 4;

#[derive(Debug)]
pub enum StartRow {
    Middle,
    Left,
    Right,
    Random,
}

#[derive(Debug)]
pub enum UserEvent {
    RuleChanged(u32),
    SizeChanged(u32),
    InitialChanged(StartRow),
    State(Box<State>),
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    rule: u32,
    width: u32,
    height: u32,
    size: u32,
    current_generation: u32,
}

// This will store the state of our game
#[derive(Debug)]
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    window: Arc<Window>,
    #[cfg(target_arch = "wasm32")]
    canvas: HtmlCanvasElement,

    // Textures to store automaton state
    grid_texture: wgpu::Texture,
    intermediate_texture: wgpu::Texture,
    compute_bind_group_layout: wgpu::BindGroupLayout,
    render_bind_group_layout: wgpu::BindGroupLayout,

    // Compute pipeline
    compute_pipeline: wgpu::ComputePipeline,
    compute_bind_groups: wgpu::BindGroup,

    // Render pipeline
    render_pipeline: wgpu::RenderPipeline,
    render_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,

    // Simulation state
    rule: u32,
    size: u32,
    start_row: StartRow,
    grid_width: u32,
    grid_height: u32,
    current_generation: u32,
}

impl State {
    async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
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
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();
        let grid_width = size.width / SIZE;
        let grid_height = size.height / SIZE;

        info!("canvas size: {}x{}", size.width, size.height);
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

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // --- Create Textures ---
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: grid_width,
                height: grid_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            label: Some("Grid Texture"),
            view_formats: &[],
        };
        let grid_texture = device.create_texture(&texture_desc);

        // Create two views for our ping-ponging technique
        let grid_texture_views = grid_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let intermediate_texture_desc = wgpu::TextureDescriptor {
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST,
            label: Some("Intermediate Grid Texture"),
            ..texture_desc
        };
        let intermediate_texture = device.create_texture(&intermediate_texture_desc);
        let intermediate_texture_view =
            intermediate_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // create the uniform
        let uniforms = Uniforms {
            rule: RULE,
            width: grid_width,
            height: grid_height,
            size: SIZE,
            current_generation: 0,
        };

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: std::mem::size_of::<Uniforms>() as u64,
            mapped_at_creation: false,
        });

        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let compute_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        // Params Uniform
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        // Source Texture (read)
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::ReadOnly,
                            format: wgpu::TextureFormat::R32Uint,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        // Destination Texture (write)
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::R32Uint,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
            });

        let render_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT, // Now used in fragment shader
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        // Texture to display
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Uint,
                        },
                        count: None,
                    },
                ],
                label: Some("render_bind_group_layout"),
            });

        // --- Create Bind Groups ---
        let compute_bind_groups = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&grid_texture_views),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&intermediate_texture_view),
                },
            ],
            label: Some("compute_bind_group_0"),
        });

        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render_bind_group_layout,
            entries: &[
                // Binding 0: Uniforms
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                // Binding 1: Texture
                wgpu::BindGroupEntry {
                    binding: 1, // Changed from 3
                    resource: wgpu::BindingResource::TextureView(&grid_texture_views),
                },
            ],
            label: Some("render_bind_group"),
        });

        // --- Create Pipelines ---
        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Pipeline Layout"),
                bind_group_layouts: &[&compute_bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: Some("compute_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&render_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
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
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // --- Initial Data ---
        let mut initial_data = vec![0u32; (grid_width * grid_height) as usize];
        // Set a single "on" cell in the middle of the first row.
        initial_data[grid_width as usize / 2] = 1;

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &grid_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&initial_data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(grid_width * 4),
                rows_per_image: Some(grid_height),
            },
            wgpu::Extent3d {
                width: grid_width,
                height: grid_height,
                depth_or_array_layers: 1,
            },
        );

        #[cfg(target_arch = "wasm32")]
        let canvas = {
            use wasm_bindgen::JsCast;

            const CANVAS_ID: &str = "shader";

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
            canvas
        };

        info!("end create new state");
        Ok(Self {
            surface,
            window,
            device,
            queue,
            compute_bind_group_layout,
            render_bind_group_layout,
            grid_texture,
            intermediate_texture,
            compute_pipeline,
            compute_bind_groups,
            render_pipeline,
            render_bind_group,
            rule: RULE,
            size: SIZE,
            start_row: StartRow::Middle,
            current_generation: 1,
            grid_width,
            grid_height,
            uniform_buffer,
            #[cfg(target_arch = "wasm32")]
            canvas,
        })
    }

    fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        info!("resize window: {width}x{height}");
        if width > 0 && height > 0 {
            self.resize_grid();
            self.reset();
            self.window.request_redraw();
        }
    }

    fn update(&mut self) {
        if self.current_generation < self.grid_height {
            let uniforms = Uniforms {
                rule: self.rule,
                width: self.grid_width,
                height: self.grid_height,
                size: self.size,
                current_generation: self.current_generation,
            };

            self.queue
                .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Compute Encoder"),
                });

            {
                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Cellular Automaton Compute Pass"),
                    timestamp_writes: None,
                });

                compute_pass.set_pipeline(&self.compute_pipeline);
                compute_pass.set_bind_group(0, &self.compute_bind_groups, &[]);
                let workgroup_count_x = self.grid_width.div_ceil(64);
                compute_pass.dispatch_workgroups(workgroup_count_x, 1, 1);
            }

            // Copy the newly computed row from the intermediate texture back to the main one.
            encoder.copy_texture_to_texture(
                // Source
                wgpu::TexelCopyTextureInfo {
                    texture: &self.intermediate_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: self.current_generation,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                // Destination
                wgpu::TexelCopyTextureInfo {
                    texture: &self.grid_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: self.current_generation,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                // Copy Size
                wgpu::Extent3d {
                    width: self.grid_width,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );

            self.queue.submit(iter::once(encoder.finish()));
            self.current_generation += 1;
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

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
                            b: 0.9,
                            #[cfg(target_arch = "wasm32")]
                            a: 0.0,
                            #[cfg(not(target_arch = "wasm32"))]
                            a: 0.5,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.draw(0..3, 0..1); // Draw a full-screen triangle
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    fn toggle_fullscreen(&mut self) {
        use web_sys::console;

        let document = web_sys::window().unwrap().document().unwrap();
        if document.fullscreen_element().is_none() {
            console::log_1(&"Entering fullscreen".into());
            self.canvas
                .request_fullscreen()
                .expect("Failed to enter fullscreen mode");
        } else {
            console::log_1(&"Exiting fullscreen".into());
            document.exit_fullscreen();
        }
        self.resize_grid();
        self.reset();
        self.window.request_redraw();
    }

    pub fn reset(&mut self) {
        info!("Resetting simulation state.");

        self.current_generation = 1;

        let initial_data = {
            let mut data = vec![0u32; (self.grid_width * self.grid_height) as usize];
            match self.start_row {
                StartRow::Middle => data[self.grid_width as usize / 2] = 1,
                StartRow::Left => data[0] = 1,
                StartRow::Right => data[self.grid_width as usize - 1] = 1,
                StartRow::Random => {
                    let mut rng = rand::rng();
                    for i in 0..self.grid_width {
                        if rng.random_bool(0.5) {
                            data[i as usize] = 1;
                        }
                    }
                }
            };
            data
        };

        let texture_size = wgpu::Extent3d {
            width: self.grid_width,
            height: self.grid_height,
            depth_or_array_layers: 1,
        };

        let data_layout = wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(self.grid_width * 4),
            rows_per_image: Some(self.grid_height),
        };

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.grid_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&initial_data),
            data_layout,
            texture_size,
        );

        // Also clear the intermediate texture
        let clear_data = vec![0u32; (self.grid_width * self.grid_height) as usize];
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.intermediate_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&clear_data),
            data_layout,
            texture_size,
        );
    }

    pub fn resize_grid(&mut self) {
        let size = self.window.inner_size();
        info!("window size: {}x{}", size.width, size.height);
        self.grid_width = size.width / self.size;
        self.grid_height = size.height / self.size;
        if self.grid_width == 0 || self.grid_height == 0 {
            return;
        }

        info!(
            "Resizing grid textures to {}x{}",
            self.grid_width, self.grid_height
        );

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.grid_width,
                height: self.grid_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            label: Some("Grid Texture"),
            view_formats: &[],
        };
        self.grid_texture = self.device.create_texture(&texture_desc);

        let intermediate_texture_desc = wgpu::TextureDescriptor {
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST,
            label: Some("Intermediate Grid Texture"),
            ..texture_desc
        };
        self.intermediate_texture = self.device.create_texture(&intermediate_texture_desc);

        let grid_texture_view = self
            .grid_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let intermediate_texture_view = self
            .intermediate_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.compute_bind_groups = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&grid_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&intermediate_texture_view),
                },
            ],
            label: Some("compute_bind_group"),
        });

        self.render_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&grid_texture_view),
                },
            ],
            label: Some("render_bind_group"),
        });
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
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::{HtmlInputElement, HtmlSelectElement};
            use winit::platform::web::WindowAttributesExtWebSys;

            let proxy = self.proxy.clone();
            let on_input_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlInputElement>() {
                    let value_str = input_element.value();

                    // Parse the value and send our UserEvent through the proxy
                    if let Ok(rule_number) = value_str.parse::<u32>() {
                        proxy
                            .send_event(UserEvent::RuleChanged(rule_number))
                            .unwrap();
                    }
                }
            });

            let proxy = self.proxy.clone();
            let on_size_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlInputElement>() {
                    let value_str = input_element.value();
                    info!("The new size is: '{}'", value_str);
                    // Parse the value and send our UserEvent through the proxy
                    if let Ok(size) = value_str.parse::<u32>() {
                        proxy.send_event(UserEvent::SizeChanged(size)).unwrap();
                    }
                }
            });

            let proxy = self.proxy.clone();
            let on_initial_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlSelectElement>() {
                    let value_str = input_element.value();
                    info!("The new initial is: '{}'", value_str);
                    if let Ok(initial) = value_str.parse::<u32>() {
                        let initial = match initial {
                            1 => StartRow::Middle,
                            2 => StartRow::Right,
                            3 => StartRow::Left,
                            4 => StartRow::Random,
                            _ => unreachable!("unknown initial value: {initial}"),
                        };
                        proxy
                            .send_event(UserEvent::InitialChanged(initial))
                            .unwrap();
                    }
                }
            });

            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            //get the text input
            let text_input = document
                .get_element_by_id("rule")
                .expect("should have an input with id 'rule'");
            let text_input: HtmlInputElement = text_input.dyn_into().map_err(|_| ()).unwrap();
            text_input
                .add_event_listener_with_callback(
                    "input",
                    on_input_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_input_callback);

            //get the text input
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

            //get the initial input
            let initial_input = document
                .get_element_by_id("initial")
                .expect("should have an input with id 'initial'");
            let initial_input: HtmlSelectElement =
                initial_input.dyn_into().map_err(|_| ()).unwrap();
            initial_input
                .add_event_listener_with_callback(
                    "change",
                    on_initial_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_initial_callback);

            //create the event loop
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id("shader").unwrap();
            let canvas: web_sys::HtmlCanvasElement =
                canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

            let attributes = Window::default_attributes().with_canvas(Some(canvas));

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
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let window_attributes = Window::default_attributes();
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.state = Some(Box::new(pollster::block_on(State::new(window)).unwrap()));
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        info!("event: {:?}", event);
        match event {
            UserEvent::RuleChanged(new_rule) => {
                info!("Handling RuleChanged event. New rule: {}", new_rule);
                if let Some(state) = &mut self.state {
                    state.rule = new_rule;
                    state.reset();
                    state.window.request_redraw();
                }
            }
            UserEvent::SizeChanged(new_size) => {
                info!("Handling SizeChanged event. New size: {}", new_size);
                if let Some(state) = &mut self.state {
                    state.size = new_size;
                    state.resize_grid();
                    state.reset();
                    state.window.request_redraw();
                }
            }
            UserEvent::InitialChanged(initial) => {
                info!("Handling SizeChanged event. New initial: {:?}", initial);
                if let Some(state) = &mut self.state {
                    info!("Initial is: {:?}", initial);
                    state.start_row = initial;
                    state.reset();
                    state.window.request_redraw();
                }
            }
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
                if state.current_generation < state.grid_height {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            let size = state.window.inner_size();
                            state.resize(size.width, size.height)
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => log::error!("wgpu error: {:?}", e),
                    }
                    state.window.request_redraw();
                }
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

#[cfg(not(target_arch = "wasm32"))]
pub fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap_throw();

    let event_loop = EventLoop::with_user_event().build().unwrap_throw();
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app).unwrap_throw();
    Ok(())
}
