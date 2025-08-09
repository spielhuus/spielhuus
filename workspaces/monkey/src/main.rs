use genetic::{Phenotype, Population, crossover};

use rand::{distr::Uniform, prelude::*};

const MUTATION_RATE: f64 = 0.01;

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

fn main() {
    let target = "All the world's a stage, and all the men and women merely players.";
    let target_len = target.len();
    let population_size = 500;
    let generations = 10_000;

    // 3. Initialize the population, specifying the `Evolvable` type.
    let mut population = Population::<StringEvolver>::new(population_size, target_len);

    println!("Target: \"{}\"", target);
    println!("---------------------------------------------------");

    for i in 0..=generations {
        // The parameter is passed to the `evolve` method.
        population.evolve(&target.to_string());

        // Reporting every 100 generations
        if i % 100 == 0 {
            let (max_fitness, fittest_genotype) = population.fittest();
            let fittest_str = String::from_utf8_lossy(&fittest_genotype);

            println!(
                "Generation {:>4}: Fittest \"{}\" (Fitness: {:.0}/{})",
                i, fittest_str, max_fitness, target_len
            );

            if fittest_str == target {
                println!("\nTarget string found in {} generations!", i);
                break;
            }
        }

        if i == generations {
            println!("\nTarget not found within {} generations.", generations);
        }
    }
    let (max_fitness, fittest_genotype) = population.fittest();
    let fittest_str = String::from_utf8_lossy(&fittest_genotype);
    println!(
        "Result: \"{}\" (Fitness: {:.0}/{})",
        fittest_str, max_fitness, target_len
    );
}
