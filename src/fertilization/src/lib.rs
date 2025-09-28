use genetic::{GenotypeInitializer, Phenotype, Population, crossover};
use log::{debug, info, warn};
use rand::prelude::*;

use std::{iter, sync::Arc};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[derive(Debug)]
pub enum UserEvent {
    SizeChanged(u32),
    State(Box<State>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
}

fn check_collision_recs(r1: Rectangle, r2: Rectangle) -> bool {
    r1.x < r2.x + r2.width
        && r1.x + r1.width > r2.x
        && r1.y < r2.y + r2.height
        && r1.y + r1.height > r2.y
}

fn vector2_distance(v1: Vector2, v2: Vector2) -> f32 {
    ((v1.x - v2.x).powi(2) + (v1.y - v2.y).powi(2)).sqrt()
}

const POPULATION_SIZE: usize = 1000;
const MUTATION_RATE: f64 = 0.3;
const ROCKET_WIDTH: f32 = 2.0;
const ROCKET_HEIGHT: f32 = 20.0;
const TITLE: &str = "Fertilization";
const SPEED: f32 = 4.0;
// const FONT_SIZE: f32 = 42.0; // Text is now printed to console

#[derive(Debug)]
pub struct Board {
    walla: Rectangle,
    wallb: Rectangle,
    target: Rectangle,
    screen_width: f32,
    screen_height: f32,
    path_len: i32,
}

impl Board {
    fn new(screen_width: f32, screen_height: f32, path_len: i32) -> Self {
        Self {
            walla: Rectangle {
                x: screen_width / 3.0,
                y: screen_height / 3.0,
                width: 20.0,
                height: screen_height / 3.0 * 2.0,
            },
            wallb: Rectangle {
                x: screen_width / 3.0 * 2.0,
                y: 0.0,
                width: 20.0,
                height: screen_height / 3.0 * 2.0,
            },
            target: Rectangle {
                x: screen_width - 15.0,
                y: screen_height / 2.0 - 50.0,
                width: 15.0,
                height: 100.0,
            },
            screen_width,
            screen_height,
            path_len,
        }
    }
}

#[derive(Debug)]
pub struct SpermEvolver {
    index: usize,
    calc_fitness: f64,

    pos: Rectangle,
    step: usize,
    winner: bool,
    dead: bool,
    collision_count: usize,
    steps_to_goal: usize,
}

impl SpermEvolver {
    // `update` is now decoupled from drawing
    pub fn update(&mut self, board: &Board, genotypes: &[Angle]) {
        if !self.winner && !self.dead {
            let angle = &genotypes[self.step];
            let movement = Vector2 {
                x: angle.0.to_radians().cos() * SPEED,
                y: angle.0.to_radians().sin() * SPEED,
            };
            self.pos.x += movement.x;
            self.pos.y += movement.y;

            if check_collision_recs(board.walla, self.pos)
                || check_collision_recs(board.wallb, self.pos)
            {
                self.pos.x -= movement.x;
                self.pos.y -= movement.y;
                self.collision_count += 1;
            }
            if self.pos.x < 0.0
                || self.pos.y < 0.0
                || self.pos.x > board.screen_width
                || self.pos.y > board.screen_height
            {
                self.dead = true;
            }
            if check_collision_recs(self.pos, board.target) {
                self.winner = true;
                self.steps_to_goal = self.step;
            }
            self.step += 1;
        }
    }

    pub fn get_draw_info(&self, genotype: &Angle) -> Option<DrawInfo> {
        if !self.dead {
            Some(DrawInfo {
                rect: self.pos,
                rotation: (genotype.0 + 90.0),
                color: if self.winner {
                    Color::GREEN
                } else {
                    Color::RED
                },
            })
        } else {
            None
        }
    }

    pub fn get_winner_path_draw_info(screen_height: f32, genotype: &[Angle]) -> Vec<DrawInfo> {
        let mut path = Vec::new();
        let mut pos = Rectangle::new(
            20.0,
            screen_height / 2.0 - 50.0,
            ROCKET_WIDTH,
            ROCKET_HEIGHT,
        );
        for angle in genotype {
            let movement = Vector2 {
                x: angle.0.to_radians().cos() * SPEED,
                y: angle.0.to_radians().sin() * SPEED,
            };
            pos.x += movement.x;
            pos.y += movement.y;
            path.push(DrawInfo {
                rect: pos,
                rotation: (angle.0 + 90.0),
                color: Color::GREEN,
            });
        }
        path
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DrawInfo {
    rect: Rectangle,
    rotation: f32,
    color: Color,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Angle(f32);

impl Phenotype for SpermEvolver {
    type Gene = Angle;
    type FitnessParam = Board;

    fn new(index: usize) -> Self {
        Self {
            index,
            calc_fitness: 0.0,
            pos: Rectangle::new(0.0, 0.0, ROCKET_WIDTH, ROCKET_HEIGHT),
            step: 0,
            winner: false,
            dead: false,
            collision_count: 0,
            steps_to_goal: 0,
        }
    }

    fn fitness(&mut self, _: &[Angle], board: &Board) {
        let distance = vector2_distance(
            Vector2 {
                x: board.target.x,
                y: board.target.y,
            },
            Vector2 {
                x: self.pos.x,
                y: self.pos.y,
            },
        ) as f64;

        if self.dead {
            self.calc_fitness = 1.0 / (distance.powf(8.0) / self.step as f64);
        } else if self.steps_to_goal > 0 {
            self.calc_fitness = 1.0 / (distance / self.steps_to_goal as f64);
        } else {
            self.calc_fitness = 1.0 / (distance.powf(4.0) / self.step as f64);
        }
    }

    fn mutate(genotype: &mut [Angle], rng: &mut ThreadRng) {
        for gene in genotype.iter_mut() {
            if rng.random_bool(MUTATION_RATE) {
                let delta_angle: f32 = rng.random_range(-10.0..=10.0);
                gene.0 = (gene.0 + delta_angle).rem_euclid(360.0);
            }
        }
    }

    fn crossover(
        parent1: &[Angle],
        parent2: &[Angle],
        child1: &mut [Angle],
        child2: &mut [Angle],
        size: usize,
        rng: &mut ThreadRng,
    ) {
        crossover::double_split(parent1, parent2, child1, child2, size, rng);
    }

    fn get_fitness(&self) -> f64 {
        self.calc_fitness
    }
    fn index(&self) -> usize {
        self.index
    }

    fn reset(&mut self) {
        self.step = 0;
        self.winner = false;
        self.dead = false;
        self.collision_count = 0;
        self.steps_to_goal = 0;
    }
}

impl GenotypeInitializer for Angle {
    fn initial_genotypes(genotype: &mut [Self], rng: &mut ThreadRng) {
        let mut angle: i32 = rng.random_range(0..=360);
        for gene in genotype.iter_mut() {
            let delta_angle: i32 = rng.random_range(-10..=10);
            angle += delta_angle;
            *gene = Angle(angle as f32);
        }
    }
}

// A single vertex on our 2D square model
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

// The unit square vertices. We draw this shape for every object.
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

// Per-instance data sent to the GPU
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model: [[f32; 4]; 4],
    color: [f32; 4],
}

impl InstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        1 => Float32x4, 2 => Float32x4, 3 => Float32x4, 4 => Float32x4, // Model Matrix
        5 => Float32x4, // Color
    ];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

fn create_transformation_matrix(rect: Rectangle, rotation_deg: f32) -> [[f32; 4]; 4] {
    let (s, c) = rotation_deg.to_radians().sin_cos();
    let w = rect.width;
    let h = rect.height;

    // The final model matrix is M = T(final_pos) * R(angle) * S(w, h) * T(-0.5, -0.5)
    // where T is translation, R is rotation, and S is scale.

    // Center of the final rectangle on screen
    let final_x = rect.x + w / 2.0;
    let final_y = rect.y + h / 2.0;

    // Rotation * Scale part of the matrix
    let m11 = c * w; // M[0][0]
    let m12 = -s * h; // M[0][1]
    let m21 = s * w; // M[1][0]
    let m22 = c * h; // M[1][1]

    // The final translation part of the matrix is calculated by applying the
    // rotation and scale to the initial centering translation (-0.5, -0.5)
    // and then adding the final screen position.
    let tx = final_x + (m11 * -0.5) + (m12 * -0.5);
    let ty = final_y + (m21 * -0.5) + (m22 * -0.5);

    // Return as a column-major matrix, which WGSL expects
    [
        // Column 0
        [m11, m21, 0.0, 0.0],
        // Column 1
        [m12, m22, 0.0, 0.0],
        // Column 2
        [0.0, 0.0, 1.0, 0.0],
        // Column 3 (Translation)
        [tx, ty, 0.0, 1.0],
    ]
}
fn create_orthographic_projection(width: f32, height: f32) -> [[f32; 4]; 4] {
    // Y-down coordinate system (0,0 is top-left)
    let (l, r, b, t, n, f) = (0.0, width, height, 0.0, -1.0, 1.0);
    [
        // Column 0
        [2.0 / (r - l), 0.0, 0.0, 0.0],
        // Column 1
        [0.0, 2.0 / (t - b), 0.0, 0.0],
        // Column 2
        [0.0, 0.0, -2.0 / (f - n), 0.0],
        // Column 3 (Translation)
        [
            -(r + l) / (r - l),
            -(t + b) / (t - b),
            -(f + n) / (f - n),
            1.0,
        ],
    ]
}

#[derive(Debug)]
pub struct State {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    projection_buffer: wgpu::Buffer,
    projection_bind_group: wgpu::BindGroup,
    screen_height: f32,
    #[cfg(target_arch = "wasm32")]
    canvas: HtmlCanvasElement,
    // population relevant props
    max_instances: usize,
    board: Board,
    population: Option<Population<SpermEvolver>>,
    loops: i32,
    round: i32,
    winners: i32,
    fast: bool,
    show_winner_path: bool,
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

        // --- Buffers ---
        use wgpu::util::DeviceExt;
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

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceRaw>() * 1) as u64, // TODO: was max instances
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // --- Projection Uniform ---
        let projection_matrix =
            create_orthographic_projection(config.width as f32, config.height as f32);
        let projection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Projection Buffer"),
            contents: bytemuck::cast_slice(&[projection_matrix]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let projection_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("projection_bind_group_layout"),
            });

        let projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &projection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: projection_buffer.as_entire_binding(),
            }],
            label: Some("projection_bind_group"),
        });

        // --- Render Pipeline ---
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&projection_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
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

        let mut state = Self {
            surface,
            window,
            device,
            queue,
            config,
            index_buffer,
            instance_buffer,
            max_instances: 1,
            num_indices,
            projection_bind_group,
            projection_buffer,
            render_pipeline,
            size,
            vertex_buffer,
            #[cfg(target_arch = "wasm32")]
            canvas,
            // population specific props
            board: Board::new(0.0, 0.0, 0),
            population: None,
            fast: false,
            loops: 0,
            round: 0,
            show_winner_path: false,
            screen_height: 0.0,
            winners: 0,
        };
        state.recreate_simulation_state();
        Ok(state)
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        } else if code == KeyCode::KeyW {
            self.show_winner_path = is_pressed;
        } else if code == KeyCode::KeyF {
            self.fast = is_pressed;
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            // Update projection matrix on resize
            let new_projection =
                create_orthographic_projection(self.config.width as f32, self.config.height as f32);
            self.queue.write_buffer(
                &self.projection_buffer,
                0,
                bytemuck::cast_slice(&[new_projection]),
            );
            self.recreate_simulation_state();
        }
    }

    fn recreate_simulation_state(&mut self) {
        let width = self.config.width as f32;
        let height = self.config.height as f32;
        let path_len = (width / 3.0) as i32;

        let max_instances = POPULATION_SIZE + path_len as usize + 10;
        self.max_instances = max_instances;

        self.instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceRaw>() * self.max_instances) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Recreate the board with the new dimensions
        self.board = Board::new(width, height, path_len);

        // Update the stored screen_height (or better, remove it and always use self.config.height)
        self.screen_height = height;

        // Reset the entire population for the new board dimensions
        let mut population = Population::<SpermEvolver>::new(POPULATION_SIZE, path_len as usize);
        population.get_phenotypes_mut().iter_mut().for_each(|p| {
            // Use the new height for initial positioning
            p.pos.y = height / 2.0 - 50.0;
        });
        self.population = Some(population);

        // Reset counters
        self.loops = 0;
        self.round = 0;
    }

    fn evolve_population(&mut self) {
        if let Some(population) = &mut self.population {
            population.evolve(&self.board);
            self.winners = population
                .get_phenotypes()
                .iter()
                .filter(|g| g.winner)
                .count() as i32;
            let start_y = self.config.height as f32 / 2.0 - 50.0;
            population
                .get_phenotypes_mut()
                .iter_mut()
                .for_each(|p| {p.reset(); p.pos.x = 20.0; p.pos.y = start_y });
            self.round = 0;
            self.loops += 1;

            // Print info to console instead of drawing text
            debug!(
                "Generation: {:04}, Winners: {}, Max Fitness: {:.2}",
                self.loops,
                self.winners,
                population.max_fitness()
            );
        }
    }

    fn update(&mut self) {
        if let Some(population) = &mut self.population {
            if self.fast {
                // Run a full generation
                while self.round < (self.board.path_len - 1) {
                    population.for_each_phenotype_mut(|p, genotype| {
                        p.update(&self.board, genotype);
                    });
                    self.round += 1;
                }
                self.evolve_population();
            } else if self.round < self.board.path_len - 1 {
                // Run one step
                population.for_each_phenotype_mut(|p, genotype| {
                    p.update(&self.board, genotype);
                });
                self.round += 1;
            } else {
                // End of round, evolve
                self.evolve_population();
            }
        }
    }

    fn render(&mut self, draw_data: &[DrawInfo]) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Convert DrawInfo to raw instance data for the GPU
        let instance_data = draw_data
            .iter()
            .map(|info| InstanceRaw {
                model: create_transformation_matrix(info.rect, info.rotation),
                color: [info.color.r, info.color.g, info.color.b, info.color.a],
            })
            .collect::<Vec<_>>();

        if instance_data.len() > self.max_instances {
            warn!("Warning: Trying to draw more instances than buffer capacity!");
        }

        self.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&instance_data),
        );

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
                            b: 0.0,
                            a: 1.0,
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
            render_pass.set_bind_group(0, &self.projection_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..instance_data.len() as u32);
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
        self.reset();
        self.window.request_redraw();
    }

    pub fn reset(&mut self) {
        info!("Resetting simulation state.");
    }

    fn get_draw_data(&mut self) -> Vec<DrawInfo> {
        let mut draw_infos = Vec::new();

        // Board
        draw_infos.push(DrawInfo {
            rect: self.board.walla,
            rotation: 0.0,
            color: Color::WHITE,
        });
        draw_infos.push(DrawInfo {
            rect: self.board.wallb,
            rotation: 0.0,
            color: Color::WHITE,
        });
        draw_infos.push(DrawInfo {
            rect: self.board.target,
            rotation: 0.0,
            color: Color::GREEN,
        });

        // Population
        if self.show_winner_path {
            if let Some(population) = &self.population {
                if let Some(winner) = population
                    .get_phenotypes()
                    .iter()
                    .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
                {
                    let genotype = population.get_genotype(winner);
                    draw_infos.extend(SpermEvolver::get_winner_path_draw_info(
                        self.screen_height,
                        genotype,
                    ));
                }
            }
        } else {
            if let Some(population) = &mut self.population {
                population.for_each_phenotype_mut(|p, genotype| {
                    if let Some(info) = p.get_draw_info(&genotype[self.round as usize]) {
                        if p.index() % 10 == 0 || self.fast {
                            // Don't draw all 1000 unless in fast mode
                            draw_infos.push(info);
                        }
                    }
                });
                if self.fast {
                    if let Some(winner) = population
                        .get_phenotypes()
                        .iter()
                        .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
                    {
                        let genotype = population.get_genotype(winner);
                        draw_infos.extend(SpermEvolver::get_winner_path_draw_info(
                            self.screen_height,
                            genotype,
                        ));
                    }
                }
            }
        }
        draw_infos
    }
}

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: winit::event_loop::EventLoopProxy<UserEvent>,
    state: Option<Box<State>>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<UserEvent>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = event_loop.create_proxy();
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
            // #[cfg(target_arch = "wasm32")]
            // _event_closures: Vec::new(),
        }
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;

            //create the event loop
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
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let window_attributes = Window::default_attributes();
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.state = Some(Box::new(
                pollster::block_on(State::new(window)).unwrap(),
            ));
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            #[cfg(target_arch = "wasm32")]
            UserEvent::State(mut state) => {
                let size = state.window.inner_size();
                info!("Resizing surface to {}x{}", size.width, size.height);
                state.resize(size);

                self.state = Some(state);
                if let Some(s) = &self.state {
                    s.window.request_redraw();
                }
            }
            _ => {
                log::warn!("Unhandled User event: {:?}", event);
            }
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
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::RedrawRequested => {
                state.update();
                let draw_data = state.get_draw_data();
                match state.render(&draw_data) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => panic!("GPU out of memory"),
                    Err(e) => eprintln!("{:?}", e),
                }
                state.window.request_redraw();
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
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app).unwrap_throw();
    Ok(())
}
