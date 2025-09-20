use genetic::{Phenotype, Population, crossover};

use rand::{distr::Uniform, prelude::*};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
use web_sys::HtmlElement;

const MUTATION_RATE: f64 = 0.01;
pub const TARGET: &str = "All the world's a stage, and all the men and women merely players.";

pub struct GameState<E: Phenotype> {
    pub target: String,
    pub generation: usize,
    pub population: Population<E>,
    pub solved: bool,
}

impl GameState<StringEvolver> {
    pub fn new(target: String, population_size: usize) -> Self {
        let target_len = target.len();
        Self {
            target,
            generation: 1,
            population: Population::<StringEvolver>::new(population_size, target_len),
            solved: false,
        }
    }
}

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

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub async fn run_evolution_async(
    target: String,
    max_fitness: HtmlElement,
    result_string: HtmlElement,
    count_string: HtmlElement,
) {
    use std::time::Duration;
    let mut game_state = GameState::new(target.to_string(), 500);
    let mut _max_fitness = 0.0;
    let mut fittest: String = String::new();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let mut generation = 0;
    while fittest != target {
        game_state.population.evolve(&game_state.target.to_string());
        game_state.generation += 1;

        let (fitness, fittest_genotype) = game_state.population.fittest();
        fittest = String::from_utf8_lossy(&fittest_genotype).to_string();
        max_fitness.set_text_content(Some(&format!("{} / {}", fitness, target.len())));
        count_string.set_text_content(Some(&format!("{}", generation)));

        result_string.set_inner_html("");
        for (i, letter) in fittest.chars().enumerate() {
            let span = document.create_element("span").expect("new span element");
            if target.chars().nth(i).expect("char at") == letter {
                span.set_class_name("letter-good");
            } else {
                span.set_class_name("letter");
            }
            span.set_text_content(Some(&letter.to_string()));
            result_string.append_child(&span).expect("span append");
        }
        gloo_timers::future::sleep(Duration::from_millis(1)).await;
        generation += 1;
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    use log::{Level, info};
    use wasm_bindgen_futures::spawn_local;
    use web_sys::{HtmlElement, HtmlInputElement};

    console_log::init_with_level(Level::Trace).expect("error initializing log");

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Ahoi Saylor!");

    body.append_child(&val)?;

    let text_input = document
        .get_element_by_id("target")
        .expect("should have an input with id 'target'");
    let text_input: HtmlInputElement = text_input.dyn_into().map_err(|_| ()).unwrap();
    text_input.set_value(TARGET);

    // text_input.set_value("0");

    let max_fitness = document
        .get_element_by_id("maxFitness")
        .expect("should have an element with id 'maxFitness'");
    let max_fitness: HtmlElement = max_fitness.dyn_into().map_err(|_| ()).unwrap();

    let result_string = document
        .get_element_by_id("result-string")
        .expect("should have an element with id 'result_string'");
    let result_string: HtmlElement = result_string.dyn_into().map_err(|_| ()).unwrap();

    let count_string = document
        .get_element_by_id("count-string")
        .expect("should have an element with id 'count_string'");
    let count_string: HtmlElement = count_string.dyn_into().map_err(|_| ()).unwrap();

    let button = document
        .get_element_by_id("targetSubmit")
        .expect("should have a button with id 'targetSubmit'");
    let button: HtmlElement = button.dyn_into::<HtmlElement>().map_err(|_| ()).unwrap();

    let on_click_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let value = text_input.value();
        info!("The input value is: '{}'", value);

        let max_fitness_clone = max_fitness.clone();
        let result_string_clone = result_string.clone();
        let count_string_clone = count_string.clone();

        // Spawn a future to run our async task
        spawn_local(async move {
            run_evolution_async(
                value,
                max_fitness_clone,
                result_string_clone,
                count_string_clone,
            )
            .await;
        });
    }) as Box<dyn FnMut(_)>);

    button.add_event_listener_with_callback("click", on_click_callback.as_ref().unchecked_ref())?;
    on_click_callback.forget();

    Ok(())
}
