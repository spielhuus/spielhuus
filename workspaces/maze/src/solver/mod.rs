use std::fmt;

pub mod a_star;
pub mod backtracker;
pub mod dead_end_filing;
pub mod djikstra;
pub mod genetic;
pub mod wall_follower;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PathfindingAlgorithm {
    Dijkstra,
    RecursiveBacktracker,
    AStar,
    DeadEndFilling,
    WallFollower,
    Genetic,
}

impl PathfindingAlgorithm {
    /// Returns a slice containing all enum variants.
    pub const fn all_variants() -> &'static [PathfindingAlgorithm] {
        &[
            PathfindingAlgorithm::Dijkstra,
            PathfindingAlgorithm::RecursiveBacktracker,
            PathfindingAlgorithm::AStar,
            PathfindingAlgorithm::DeadEndFilling,
            PathfindingAlgorithm::WallFollower,
            PathfindingAlgorithm::Genetic,
        ]
    }
}

impl fmt::Display for PathfindingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathfindingAlgorithm::Dijkstra => write!(f, "Dijkstra"),
            PathfindingAlgorithm::RecursiveBacktracker => write!(f, "Recursive Backtracker"),
            PathfindingAlgorithm::AStar => write!(f, "A*"),
            PathfindingAlgorithm::DeadEndFilling => write!(f, "Dead End Filling"),
            PathfindingAlgorithm::WallFollower => write!(f, "Wall Follower"),
            PathfindingAlgorithm::Genetic => write!(f, "Genetic"),
        }
    }
}
