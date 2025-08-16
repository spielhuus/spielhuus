use genetic::{Phenotype, Population, crossover};

use rand::{distr::Uniform, prelude::*};

const MUTATION_RATE: f64 = 0.01;

use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use std::ffi::{CStr, CString, c_char, c_int, c_void};

#[allow(static_mut_refs)]
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
    let (text, max_fitness, round) = next_generation();
    unsafe {
        update(
            CString::new(text).expect("cstr").as_ptr(),
            max_fitness,
            round,
        );
    }
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn update(fittest_str: *const c_char, max_fitness: f64, target_len: usize);
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn set_target(text: *const c_char) {
    unsafe {
        let c_str = CStr::from_ptr(text);
        let target = c_str.to_string_lossy().into_owned();
        println!("update target: {}", target);
        GAME_STATE.with(|cell| {
            if let Some(state) = cell.borrow_mut().as_mut() {
                state.set_target(target);
            } else {
                panic!("can not get result");
            }
        })
    }
}

struct GameState<E: Phenotype> {
    target: String,
    population_size: usize,
    generations: usize,
    generation: usize,
    population: Population<E>,
    solved: bool,
}

impl GameState<StringEvolver> {
    fn new(target: String, population_size: usize) -> Self {
        let target_len = target.len();
        Self {
            target,
            population_size,
            generations: 10_000,
            generation: 1,
            population: Population::<StringEvolver>::new(population_size, target_len),
            solved: false,
        }
    }

    fn set_target(&mut self, target: String) {
        self.solved = true;
        let target_len = target.len();
        self.target = target;
        self.generation = 1;
        self.population = Population::<StringEvolver>::new(self.population_size, target_len);
        self.solved = false;
    }
}

thread_local! {
    static GAME_STATE: RefCell<Option<GameState<StringEvolver>>> = RefCell::new(None);
}

pub struct TargetString(pub String);

pub struct StringEvolver {
    index: usize,
    calc_fitness: f64,
}

impl Phenotype for StringEvolver {
    type Gene = u8;
    type FitnessParam = String;

    fn new(index: usize) -> Self {
        Self {
            index,
            calc_fitness: 0.0,
        }
    }

    fn fitness(&mut self, genotype: &[u8], target_string: &String) {
        self.calc_fitness = target_string
            .as_bytes()
            .iter()
            .zip(genotype)
            .filter(|&(target_char, gene_char)| target_char == gene_char)
            .count() as f64;
    }

    fn mutate(genotype: &mut [u8], rng: &mut ThreadRng) {
        let char_range = Uniform::new_inclusive(32, 126).unwrap();
        for gene in genotype.iter_mut() {
            if rng.random_bool(MUTATION_RATE) {
                *gene = rng.sample(char_range);
            }
        }
    }

    fn crossover(
        parent1: &[u8],
        parent2: &[u8],
        child1: &mut [u8],
        child2: &mut [u8],
        size: usize,
        rng: &mut ThreadRng,
    ) {
        crossover::single_split(parent1, parent2, child1, child2, size, rng);
    }

    fn get_fitness(&self) -> f64 {
        self.calc_fitness
    }

    fn index(&self) -> usize {
        self.index
    }

    fn reset(&mut self) {}
}

fn next_generation() -> (String, f64, usize) {
    GAME_STATE.with(|cell| {
        if let Some(state) = &mut *cell.borrow_mut() {
            if !state.solved {
                state.population.evolve(&state.target.to_string());
                state.generation += 1;
            }
            let (max_fitness, fittest_genotype) = state.population.fittest();
            let str = String::from_utf8_lossy(&fittest_genotype).to_string();
            state.solved = str == state.target;
            (str, max_fitness, state.generation)
        } else {
            panic!("can not get game state.");
        }
    })
}

fn main() {
    let target = "All the world's a stage, and all the men and women merely players.";

    // initialize the maze
    let game_state = GameState::new(target.to_string(), 500);
    GAME_STATE.with(|cell| {
        *cell.borrow_mut() = Some(game_state);
    });

    #[cfg(target_arch = "wasm32")]
    {
        unsafe {
            emscripten_set_main_loop_arg(main_loop_wrapper, std::ptr::null_mut(), 0, 1);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("Target: \"{target}\"");
        println!("---------------------------------------------------");

        let mut fittest_str = String::new();
        let mut round = 0;
        let mut max_fitness = 0.0;
        let mut count = 0;
        while fittest_str != target {
            (fittest_str, max_fitness, count) = next_generation();
            println!("#{count} {fittest_str}");
            round += 1;
        }
    }
}
