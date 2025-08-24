use monkey_bindgen::{GameState, TARGET};

pub fn main() {
    println!("Target: \"{TARGET}\"");
    println!("---------------------------------------------------");

    let mut game_state = GameState::new(TARGET.to_string(), 500);
    let mut _max_fitness = 0.0;
    let mut fittest: String = String::new();
    while fittest != TARGET {
        game_state.population.evolve(&game_state.target.to_string());
        game_state.generation += 1;

        let (_, fittest_genotype) = game_state.population.fittest();
        fittest = String::from_utf8_lossy(&fittest_genotype).to_string();
        println!("#{} {}", game_state.generation, fittest);
    }
}
