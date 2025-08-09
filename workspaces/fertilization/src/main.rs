use genetic::{
    // constants::FONT,
    {GenotypeInitializer, Phenotype, Population, crossover},
};
use raylib_egui_rs::color::Color;
use raylib_egui_rs::math::*;
use raylib_egui_rs::raylib;

use rand::prelude::*;
use std::ffi::CString;

pub const FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/Roboto-Regular.ttf"
));

#[cfg(target_arch = "wasm32")]
use std::ffi::{c_int, c_void};

const POPULATION_SIZE: usize = 1000;
const MUTATION_RATE: f64 = 0.3;
const ROCKET_WIDTH: f32 = 2.0;
const ROCKET_HEIGHT: f32 = 20.0;
const TITLE: &str = "Fertilization";
const SPEED: f32 = 4.0;
const FONT_SIZE: f32 = 42.0;

pub struct Board {
    walla: Rectangle,
    wallb: Rectangle,
    target: Rectangle,
    screen_width: i32,
    screen_height: i32,
    path_len: i32,
}

impl Board {
    fn new(screen_width: i32, screen_height: i32, path_len: i32) -> Self {
        Self {
            walla: Rectangle {
                x: (screen_width / 3) as f32,
                y: (screen_height / 3) as f32,
                width: 20.0,
                height: (screen_height / 3 * 2) as f32,
            },
            wallb: Rectangle {
                x: (screen_width / 3 * 2) as f32,
                y: 0.0,
                width: 20.0,
                height: (screen_height / 3 * 2) as f32,
            },
            target: Rectangle {
                x: screen_width as f32 - 15.0,
                y: screen_height as f32 / 2.0 - 50.0,
                width: 15.0,
                height: 100.0,
            },
            screen_width,
            screen_height,
            path_len,
        }
    }
}

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
    pub fn update(&mut self, board: &Board, genotypes: &[Angle]) {
        if !self.winner && !self.dead {
            let angle = &genotypes[self.step];
            let movement = Vector2 {
                x: angle.0.to_radians().cos() * SPEED,
                y: angle.0.to_radians().sin() * SPEED,
            };
            self.pos.x += movement.x;
            self.pos.y += movement.y;

            if raylib::CheckCollisionRecs(board.walla, self.pos)
                || raylib::CheckCollisionRecs(board.wallb, self.pos)
            {
                self.pos.x -= movement.x;
                self.pos.y -= movement.y;
                self.collision_count += 1;
            }
            if self.pos.x < 0.0
                || self.pos.y < 0.0
                || self.pos.x > (board.screen_width as f32)
                || self.pos.y > (board.screen_height as f32)
            {
                self.dead = true;
            }
            if raylib::CheckCollisionRecs(self.pos, board.target) {
                self.winner = true;
                self.steps_to_goal = self.step;
            }
            self.step += 1;
        }
    }
    pub fn draw(&self, genotype: &Angle) {
        if !self.dead {
            raylib::DrawRectanglePro(
                self.pos,
                Vector2 { x: 0.0, y: 0.0 },
                (genotype.0 + 90.0) % 360.0,
                if self.winner {
                    Color::GREEN
                } else {
                    Color::RED
                },
            );
        }
    }
    pub fn draw_winner(&self, genotype: &[Angle]) {
        let mut pos = Rectangle::new(
            20.0,
            (raylib::GetScreenHeight() as f32) / 2.0 - 50.0,
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
            raylib::DrawRectanglePro(
                pos,
                Vector2 { x: 0.0, y: 0.0 },
                (angle.0 + 90.0) % 360.0,
                Color::GREEN,
            );
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Angle(f32);

impl Phenotype for SpermEvolver {
    type Gene = Angle;
    type FitnessParam = Board;

    fn new(index: usize) -> Self {
        Self {
            index,
            calc_fitness: 0.0,
            pos: Rectangle::new(
                20.0,
                (raylib::GetScreenHeight() as f32) / 2.0 - 50.0,
                ROCKET_WIDTH,
                ROCKET_HEIGHT,
            ),
            step: 0,
            winner: false,
            dead: false,
            collision_count: 0,
            steps_to_goal: 0,
        }
    }

    fn fitness(&mut self, _: &[Angle], board: &Board) {
        let distance = raylib::Vector2Distance(
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
        self.pos = Rectangle::new(
            20.0,
            (raylib::GetScreenHeight() as f32) / 2.0 - 50.0,
            ROCKET_WIDTH,
            ROCKET_HEIGHT,
        );
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

struct GameState {
    board: Board,
    population: Population<SpermEvolver>,
    loops: i32,
    round: i32,
    winners: i32,
    font: raylib::Font,
    fast: bool,
}

#[cfg(target_arch = "wasm32")]
type EmArgCallbackFunc = unsafe extern "C" fn(arg: *mut c_void);

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn emscripten_set_main_loop_arg(
        func: EmArgCallbackFunc,
        arg: *mut c_void,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" fn main_loop_wrapper(arg: *mut c_void) {
    let game_state = &mut *(arg as *mut GameState);
    update(game_state);
}

fn main() {
    unsafe {
        raylib::InitWindow(1280, 720, TITLE);
        let screen_width = 1280;
        let screen_height = 720;
        let path_len: i32 = screen_width / 3;

        raylib::SetTargetFPS(60);

        let file_type_c_str = CString::new(".ttf").expect("CString::new failed");

        let font = raylib::LoadFontFromMemory(
            // file_type_c_str.as_ptr(),
            ".ttf", // FONT.as_ptr(),
            FONT,   // FONT.len() as i32,
            192,    // std::ptr::null_mut(),
            // 0,
            None,
        );

        let mut game_state = GameState {
            board: Board::new(screen_width, screen_height, path_len),
            population: Population::<SpermEvolver>::new(POPULATION_SIZE, path_len as usize),
            loops: 0,
            round: 0,
            winners: 0,
            font,
            fast: false,
        };

        // Main game loop
        #[cfg(not(target_arch = "wasm32"))]
        {
            while !raylib::WindowShouldClose() {
                update(&mut game_state);
            }
            raylib::CloseWindow();
        }
        #[cfg(target_arch = "wasm32")]
        {
            let boxed_state = Box::new(game_state);
            let state_ptr = Box::into_raw(boxed_state) as *mut c_void;
            emscripten_set_main_loop_arg(main_loop_wrapper, state_ptr, 0, 1);
        }
    }
}

fn update(state: &mut GameState) {
    let help_text = "(press <F> for toggle fast forward, <W> to show winner.)";

    raylib::BeginDrawing();
    raylib::ClearBackground(Color::BLACK);

    raylib::DrawRectangleRec(state.board.walla, Color::WHITE);
    raylib::DrawRectangleRec(state.board.wallb, Color::WHITE);
    raylib::DrawRectangleRounded(state.board.target, 1.0, 20, Color::GREEN);

    if raylib::IsKeyPressed(raylib::KeyboardKey::KEY_F) {
        println!("change fast");
        state.fast = !state.fast;
    }
    if raylib::IsKeyDown(raylib::KeyboardKey::KEY_W) {
        let winner = state
            .population
            .get_phenotypes()
            .iter()
            .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .unwrap();
        winner.draw_winner(state.population.get_genotype(winner));
    } else if state.fast {
        while state.round < (state.board.path_len - 1) {
            let mut count = 0;
            state.population.for_each_phenotype_mut(|p, genotype| {
                p.update(&state.board, genotype);
                if count % 10 == 0 {
                    p.draw(&genotype[state.round as usize]);
                }
                count += 1;
            });
            let winner = state
                .population
                .get_phenotypes()
                .iter()
                .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
                .unwrap();
            winner.draw_winner(state.population.get_genotype(winner));
            state.round += 1;
        }
        state.population.evolve(&state.board);
        state.winners = state
            .population
            .get_phenotypes()
            .iter()
            .filter(|g| g.winner)
            .count() as i32;
        state
            .population
            .get_phenotypes_mut()
            .iter_mut()
            .for_each(|p| p.reset());
        state.round = 0;
        state.loops += 1;
    } else if state.round < state.board.path_len - 1 {
        state.population.for_each_phenotype_mut(|p, genotype| {
            p.update(&state.board, genotype);
            p.draw(&genotype[state.round as usize]);
        });
        state.round += 1;
    } else {
        state.population.evolve(&state.board);
        state.winners = state
            .population
            .get_phenotypes()
            .iter()
            .filter(|g| g.winner)
            .count() as i32;
        state
            .population
            .get_phenotypes_mut()
            .iter_mut()
            .for_each(|p| p.reset());
        state.round = 0;
        state.loops += 1;
    }

    let info_text = format!(
        "Round: {:04}, winners: {}, fitness: {:.2}",
        state.loops,
        state.winners,
        state.population.max_fitness()
    );

    raylib::DrawTextEx(
        state.font,
        &info_text,
        Vector2 { x: 10.0, y: 10.0 },
        FONT_SIZE,
        2.0,
        Color::WHITE,
    );
    raylib::DrawTextEx(
        state.font,
        help_text,
        Vector2 { x: 10.0, y: 40.0 },
        FONT_SIZE,
        2.0,
        Color::WHITE,
    );
    raylib::EndDrawing();
}
