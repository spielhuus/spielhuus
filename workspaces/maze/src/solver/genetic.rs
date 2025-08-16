use rand::{distr::StandardUniform, prelude::*};

use crate::{Board, Direction, Solver, State, path};
use genetic::{GenotypeInitializer, Phenotype, Population, crossover};

use raylib_egui_rs::color::Color;
use raylib_egui_rs::raylib;

const POPULATION_SIZE: usize = 1000;
const MUTATION_RATE: f64 = 0.02;

const DISTANCE: f64 = 100.0;
const MISSED_STEPS: f64 = 0.3;
const DEAD_ENDS: f64 = 10.0;
const BACKWALK_PENALTY: f64 = 0.5;
const LENGTH_PENALTY: f64 = 0.01;

#[derive(Copy, Clone, Debug, Default)]
pub enum Move {
    #[default]
    Forward,
    Left,
    Right,
}

impl Distribution<Move> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        let index: u8 = rng.random_range(0..3);
        match index {
            0 => Move::Forward,
            1 => Move::Left,
            2 => Move::Right,
            _ => unreachable!(),
        }
    }
}
impl GenotypeInitializer for Move {
    fn initial_genotypes(genotype: &mut [Self], rng: &mut ThreadRng) {
        for gene in genotype.iter_mut() {
            *gene = rng.random();
        }
    }
}

pub struct Maze {
    board_size: usize,
}

impl Maze {
    pub fn new(board_size: usize) -> Self {
        Self { board_size }
    }
}

pub struct PathEvolver {
    index: usize,
    cell: usize,
    x: usize,
    y: usize,
    direction: Direction,
    calc_fitness: f64,
    backwalks: Vec<usize>,
    dead_ends: usize,
    missed_steps: Vec<usize>,
    path: Vec<usize>,
    reached_end: bool,
}

impl Phenotype for PathEvolver {
    type Gene = Move;
    type FitnessParam = Maze;

    fn new(index: usize) -> Self {
        Self {
            index,
            calc_fitness: 0.0,
            cell: 0,
            x: 0,
            y: 0,
            direction: Direction::East,
            backwalks: vec![],
            dead_ends: 0,
            missed_steps: vec![],
            path: vec![0],
            reached_end: false,
        }
    }

    fn fitness(&mut self, _: &[Move], maze: &Maze) {
        // Calculate Manhattan distance to the end
        let dx = (maze.board_size as i32 - 1 - self.x as i32).abs() as f64;
        let dy = (maze.board_size as i32 - 1 - self.y as i32).abs() as f64;
        let manhattan_distance_to_end = dx + dy;
        let mut fitness_score =
            (maze.board_size as f64 * 2.0 - manhattan_distance_to_end) * DISTANCE;

        fitness_score += self.path.len() as f64 * 2.0;
        fitness_score -= self.missed_steps.len() as f64 * MISSED_STEPS;
        fitness_score -= self.dead_ends as f64 * DEAD_ENDS;
        fitness_score -= self.backwalks.len() as f64 * BACKWALK_PENALTY;
        fitness_score -= self.path.len() as f64 * LENGTH_PENALTY;

        if self.reached_end {
            fitness_score += 1_000.0;
        }

        self.calc_fitness = fitness_score.max(0.0)
    }

    fn mutate(genotype: &mut [Move], rng: &mut ThreadRng) {
        for gene in genotype.iter_mut() {
            if rng.random_bool(MUTATION_RATE) {
                let random_move: Move = rand::random();
                *gene = random_move;
            }
        }
    }

    fn crossover(
        parent1: &[Move],
        parent2: &[Move],
        child1: &mut [Move],
        child2: &mut [Move],
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
        // Reset all state to the same as `new()`
        self.calc_fitness = 0.0;
        self.cell = 0;
        self.x = 0;
        self.y = 0;
        self.direction = Direction::East;
        self.backwalks.clear();
        self.dead_ends = 0;
        self.missed_steps.clear();
        self.path = vec![0];
        self.reached_end = false;
    }
}

pub struct Genetic<T: Phenotype> {
    population: Population<T>,
    steps: usize,
    maze: Maze,
}

impl<PathEvolver: Phenotype> Genetic<PathEvolver> {
    pub fn new(board: &Board) -> Self {
        let population = Population::<PathEvolver>::new(POPULATION_SIZE, board.board_size.pow(2));

        Self {
            population,
            steps: 1,
            maze: Maze::new(board.board_size),
        }
    }
}

impl Genetic<PathEvolver> {
    fn direction(mve: &Move, direction: &Direction) -> Direction {
        match direction {
            Direction::North => match mve {
                Move::Forward => Direction::North,
                Move::Left => Direction::West,
                Move::Right => Direction::East,
            },
            Direction::South => match mve {
                Move::Forward => Direction::South,
                Move::Left => Direction::East,
                Move::Right => Direction::West,
            },
            Direction::East => match mve {
                Move::Forward => Direction::East,
                Move::Left => Direction::North,
                Move::Right => Direction::South,
            },
            Direction::West => match mve {
                Move::Forward => Direction::West,
                Move::Left => Direction::South,
                Move::Right => Direction::North,
            },
        }
    }

    fn move_floor(phenotype: &mut PathEvolver, step: usize, board: &Board, item: &Move) {
        let cell = &board.cells[phenotype.cell];
        let new_direction = Self::direction(item, &phenotype.direction);

        let mut move_in_direction = |intended_direction: Direction| {
            let mut backwalk_logged = false;

            // This inner loop moves down a corridor based on a single gene.
            while {
                // Determine if we can continue moving in the intended direction
                let current_cell = &board.cells[phenotype.cell];
                match intended_direction {
                    Direction::North => current_cell.y > 0 && !current_cell.walls.top,
                    Direction::South => {
                        current_cell.y < board.board_size - 1 && !current_cell.walls.bottom
                    }
                    Direction::East => {
                        current_cell.x < board.board_size - 1 && !current_cell.walls.right
                    }
                    Direction::West => current_cell.x > 0 && !current_cell.walls.left,
                }
            } {
                let (next_x, next_y) = match intended_direction {
                    Direction::North => (phenotype.x, phenotype.y - 1),
                    Direction::South => (phenotype.x, phenotype.y + 1),
                    Direction::East => (phenotype.x + 1, phenotype.y),
                    Direction::West => (phenotype.x - 1, phenotype.y),
                };
                let next_cell_index = board.get_index(next_x, next_y);

                if !backwalk_logged && phenotype.path.contains(&next_cell_index) {
                    phenotype.backwalks.push(step);
                    backwalk_logged = true;
                }

                // Update phenotype state
                phenotype.x = next_x;
                phenotype.y = next_y;
                phenotype.cell = next_cell_index;
                phenotype.direction = intended_direction;
                phenotype.path.push(phenotype.cell);

                // Stop at junctions (a simple heuristic)
                if board.cells[phenotype.cell].count_walls() <= 1 {
                    break;
                }
            }
        };

        // Check if the initial move is valid. If not, it's a "missed step".
        let can_move = match new_direction {
            Direction::North => cell.y > 0 && !cell.walls.top,
            Direction::South => cell.y < board.board_size - 1 && !cell.walls.bottom,
            Direction::East => cell.x < board.board_size - 1 && !cell.walls.right,
            Direction::West => cell.x > 0 && !cell.walls.left,
        };

        if can_move {
            move_in_direction(new_direction);
        } else {
            phenotype.missed_steps.push(step);
        }

        // Check for dead ends after any move attempt (successful or not)
        if board.cells[phenotype.cell].is_dead_end() {
            phenotype.dead_ends += 1;
        }

        // Check if the goal has been reached
        if phenotype.x == board.board_size - 1 && phenotype.y == board.board_size - 1 {
            phenotype.reached_end = true;
        }
    }
}

impl Solver for Genetic<PathEvolver> {
    fn step(&mut self, board: &Board) -> Result<State, String> {
        self.population.for_each_phenotype_mut(|p, genotype| {
            p.reset();
            for (step_index, current_move) in genotype.iter().enumerate() {
                if p.reached_end {
                    break;
                }
                Genetic::move_floor(p, step_index, board, current_move);
            }
        });

        self.population.evolve(&self.maze);
        self.steps += 1;

        // check the fittest
        let winner = self
            .population
            .get_phenotypes()
            .iter()
            .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .unwrap();

        if winner.reached_end {
            Ok(State::Done)
        } else {
            Ok(State::Solve)
        }
    }

    fn get_path(&self) -> &Vec<usize> {
        let winner = self
            .population
            .get_phenotypes()
            .iter()
            .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .unwrap();

        &winner.path
    }

    fn draw(&self, board: &Board) {
        for p in self.population.get_phenotypes().iter() {
            path::draw_path(board, &p.path, raylib::ColorFromHSV(80.0, 0.75, 1.0));
        }
        let winner = self
            .population
            .get_phenotypes()
            .iter()
            .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .unwrap();
        path::draw_path(board, &winner.path, Color::GREEN);
    }
}
