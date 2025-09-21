//! # Generic Genetic Algorithm Framework
//!
//! This crate provides a flexible and extensible framework for implementing
//! genetic algorithms in Rust. It defines core traits and a `Population`
//! structure to manage the evolutionary process.
//!

use rand::{distr::Uniform, prelude::*};

pub trait Phenotype {
    /// The type of a single gene (e.g., `u8`, `f64`).
    type Gene: Copy + Default + GenotypeInitializer;
    /// The type of the parameter needed for fitness calculation.
    type FitnessParam;

    fn new(index: usize) -> Self;

    fn fitness(&mut self, genotype: &[Self::Gene], param: &Self::FitnessParam);
    fn mutate(genotype: &mut [Self::Gene], rng: &mut ThreadRng);

    fn crossover(
        parent1: &[Self::Gene],
        parent2: &[Self::Gene],
        child1: &mut [Self::Gene],
        child2: &mut [Self::Gene],
        size: usize,
        rng: &mut ThreadRng,
    );

    fn get_fitness(&self) -> f64;

    /// Returns the phenotype's index within the population.
    fn index(&self) -> usize;
    fn reset(&mut self);
}

#[derive(Debug)]
pub struct Population<E: Phenotype> {
    genotype_arena: Vec<E::Gene>,
    next_gen_arena: Vec<E::Gene>,
    phenotypes: Vec<E>,
    // phenotype_size: usize,
    genotype_size: usize,
    // weights: Vec<f64>,
    rng: ThreadRng,

    _phantom: std::marker::PhantomData<E>,
}

impl<E: Phenotype> Population<E> {
    pub fn new(phenotype_size: usize, genotype_size: usize) -> Self {
        let mut rng = rand::rng();
        let arena_size = phenotype_size * genotype_size;

        let mut genotype_arena = vec![E::Gene::default(); arena_size];

        for genotype_slice in genotype_arena.chunks_mut(genotype_size) {
            E::Gene::initial_genotypes(genotype_slice, &mut rng);
        }

        Self {
            genotype_arena,
            next_gen_arena: vec![E::Gene::default(); arena_size],
            phenotypes: (0..phenotype_size).map(|i| E::new(i)).collect(),
            // phenotype_size,
            genotype_size,
            // weights: vec![0.0; phenotype_size],
            rng,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn evolve(&mut self, param: &E::FitnessParam) {
        // 1. Calculate fitness for the current generation.
        self.calculate_fitness(param);

        // 2. Select parents and create the next generation via crossover and mutation.
        self.create_next_generation();

        // 3. Swap arenas. The new generation is now the current one.
        std::mem::swap(&mut self.genotype_arena, &mut self.next_gen_arena);
    }

    fn calculate_fitness(&mut self, param: &E::FitnessParam) {
        for p in self.phenotypes.iter_mut() {
            let start = p.index() * self.genotype_size;
            let end = start + self.genotype_size;
            let genotype = &self.genotype_arena[start..end];
            p.fitness(genotype, param);
        }
    }

    fn select_parent_by_tournament(
        phenotypes: &[E],
        rng: &mut ThreadRng,
        tournament_size: usize,
    ) -> usize {
        let population_size = phenotypes.len();
        if population_size == 0 {
            // Handle empty population case to avoid panic on range
            return 0;
        }

        // Select the first contender as the initial best
        let mut best_idx = rng.random_range(0..population_size);
        let mut best_fitness = phenotypes[best_idx].get_fitness();

        // Run the rest of the tournament (starting from the second contender)
        for _ in 1..tournament_size {
            let contender_idx = rng.random_range(0..population_size);
            let contender_fitness = phenotypes[contender_idx].get_fitness();

            if contender_fitness > best_fitness {
                best_fitness = contender_fitness;
                best_idx = contender_idx;
            }
        }
        best_idx
    }
    fn create_next_generation(&mut self) {
        // Elitism part is unchanged and correct
        let fittest_idx = self
            .phenotypes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .map(|(index, _)| index)
            .unwrap_or(0);

        let elite_start = fittest_idx * self.genotype_size;
        let elite_genotype = &self.genotype_arena[elite_start..elite_start + self.genotype_size];
        self.next_gen_arena[0..self.genotype_size].copy_from_slice(elite_genotype);

        // This creates the first mutable borrow. It's fine.
        let mut child_chunks =
            self.next_gen_arena[self.genotype_size..].chunks_mut(self.genotype_size);

        while let (Some(child1_geno), Some(child2_geno)) =
            (child_chunks.next(), child_chunks.next())
        {
            const TOURNAMENT_SIZE: usize = 3;

            // We pass immutable &self.phenotypes and mutable &mut self.rng.
            // This is allowed because they are different fields from self.next_gen_arena.
            let parent1_idx =
                Self::select_parent_by_tournament(&self.phenotypes, &mut self.rng, TOURNAMENT_SIZE);
            let parent2_idx =
                Self::select_parent_by_tournament(&self.phenotypes, &mut self.rng, TOURNAMENT_SIZE);

            // This part remains the same
            let parent1_start = parent1_idx * self.genotype_size;
            let parent2_start = parent2_idx * self.genotype_size;

            let parent1_geno =
                &self.genotype_arena[parent1_start..parent1_start + self.genotype_size];
            let parent2_geno =
                &self.genotype_arena[parent2_start..parent2_start + self.genotype_size];

            E::crossover(
                parent1_geno,
                parent2_geno,
                child1_geno,
                child2_geno,
                self.genotype_size,
                &mut self.rng,
            );

            E::mutate(child1_geno, &mut self.rng);
            E::mutate(child2_geno, &mut self.rng);
        }
    }

    pub fn fittest(&self) -> (f64, Vec<E::Gene>) {
        let fittest_phenotype = self
            .phenotypes
            .iter()
            .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .unwrap(); // Should not panic in a non-empty population

        let start = fittest_phenotype.index() * self.genotype_size;
        let end = start + self.genotype_size;

        (
            fittest_phenotype.get_fitness(),
            self.genotype_arena[start..end].to_vec(),
        )
    }
    pub fn max_fitness(&self) -> f64 {
        self.phenotypes
            .iter()
            .map(|p| p.get_fitness())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
    pub fn get_phenotypes(&self) -> &[E] {
        &self.phenotypes
    }
    pub fn get_phenotypes_mut(&mut self) -> &mut [E] {
        &mut self.phenotypes
    }

    /// Retrieves an immutable slice of the genotype for a given phenotype.
    ///
    /// # Panics
    /// Panics if the phenotype's index is out of bounds for the current population size.
    pub fn get_genotype(&self, phenotype: &E) -> &[E::Gene] {
        let start = phenotype.index() * self.genotype_size;
        let end = start + self.genotype_size;
        // This slice access is safe because `Phenotype::index()` is designed to be
        // an index into `Population::phenotypes`, and `genotype_arena` is sized
        // based on `phenotype_size * genotype_size`.
        &self.genotype_arena[start..end]
    }

    /// Iterates over each phenotype mutably, providing its corresponding genotype slice.
    ///
    /// This method safely handles the borrowing of phenotypes and genotypes by using
    /// a closure, avoiding the conflicting borrow issues of a standard iterator.
    pub fn for_each_phenotype_mut<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut E, &[E::Gene]),
    {
        // Here, we can manually iterate and use split borrows on the struct fields.
        // The borrow checker allows us to mutably borrow `self.phenotypes` while
        // immutably borrowing `self.genotype_arena` because they are distinct fields.
        let genotype_size = self.genotype_size;
        let genotype_arena = &self.genotype_arena;

        for (i, p) in self.phenotypes.iter_mut().enumerate() {
            let start = i * genotype_size;
            let end = start + genotype_size;
            let genotype = &genotype_arena[start..end];

            // Call the user-provided closure with the phenotype and its genotype
            func(p, genotype);
        }
    }
}

/// A trait for types that can initialize a vector of their own type, typically
/// used for creating initial genotypes.
pub trait GenotypeInitializer {
    fn initial_genotypes(genotype: &mut [Self], rng: &mut ThreadRng)
    where
        Self: Sized;
}

impl GenotypeInitializer for u8 {
    fn initial_genotypes(genotype: &mut [u8], rng: &mut ThreadRng) {
        let char_range = Uniform::new_inclusive(32u8, 126u8).unwrap();
        for gene in genotype.iter_mut() {
            *gene = rng.sample(char_range);
        }
    }
}

pub mod crossover {
    use rand::prelude::*;

    pub fn single_split<T: Copy>(
        parent1: &[T],
        parent2: &[T],
        child1: &mut [T],
        child2: &mut [T],
        size: usize,
        rng: &mut ThreadRng,
    ) {
        let crossover_point = rng.random_range(1..size);
        let (p1_head, p1_tail) = parent1.split_at(crossover_point);
        let (p2_head, p2_tail) = parent2.split_at(crossover_point);

        // Create child 1 by combining head of parent 1 and tail of parent 2
        child1[..crossover_point].copy_from_slice(p1_head);
        child1[crossover_point..].copy_from_slice(p2_tail);

        // Create child 2 by combining head of parent 2 and tail of parent 1
        child2[..crossover_point].copy_from_slice(p2_head);
        child2[crossover_point..].copy_from_slice(p1_tail);
    }

    pub fn double_split<T: Copy>(
        parent1: &[T],
        parent2: &[T],
        child1: &mut [T],
        child2: &mut [T],
        size: usize,
        rng: &mut ThreadRng,
    ) {
        // Edge Case: If the genotype has fewer than 3 elements,
        // it's impossible to pick two distinct internal points.
        // Falling back to no crossover prevents panics or infinite loops.
        if size < 3 {
            child1.copy_from_slice(parent1);
            child2.copy_from_slice(parent2);
            return;
        }

        // 1. Generate two DISTINCT crossover points.
        let mut point1 = rng.random_range(1..size);
        let mut point2 = rng.random_range(1..size);
        while point1 == point2 {
            point2 = rng.random_range(1..size);
        }

        // 2. Ensure point1 < point2 to make slicing logic simple.
        if point1 > point2 {
            std::mem::swap(&mut point1, &mut point2);
        }

        // 3. Assemble the children by swapping the middle segment.
        // Child 1 = P1_head + P2_middle + P1_tail
        child1[..point1].copy_from_slice(&parent1[..point1]);
        child1[point1..point2].copy_from_slice(&parent2[point1..point2]);
        child1[point2..].copy_from_slice(&parent1[point2..]);

        // Child 2 = P2_head + P1_middle + P2_tail
        child2[..point1].copy_from_slice(&parent2[..point1]);
        child2[point1..point2].copy_from_slice(&parent1[point1..point2]);
        child2[point2..].copy_from_slice(&parent2[point2..]);
    }
}
