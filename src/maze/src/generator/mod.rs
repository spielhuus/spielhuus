use core::fmt;

pub mod aldous_broder;
pub mod backtracking;
pub mod binary_tree;
pub mod eller;
pub mod growing_tree;
pub mod hunt_and_kill;
pub mod kruskal;
pub mod prim;
pub mod recursive_division;
pub mod sidewinder;
pub mod wilson;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MazeAlgorithm {
    RecursiveBacktracker,
    Kruskal,
    Eller,
    Prim,
    RecursiveDivision,
    AldousBroder,
    Wilson,
    HuntAndKill,
    GrowingTree,
    BinaryTree,
    Sidewinder,
}

impl MazeAlgorithm {
    // Returns a slice containing all enum variants.
    pub const fn all_variants() -> &'static [MazeAlgorithm] {
        &[
            MazeAlgorithm::RecursiveBacktracker,
            MazeAlgorithm::Kruskal,
            MazeAlgorithm::Eller,
            MazeAlgorithm::Prim,
            MazeAlgorithm::RecursiveDivision,
            MazeAlgorithm::AldousBroder,
            MazeAlgorithm::Wilson,
            MazeAlgorithm::HuntAndKill,
            MazeAlgorithm::GrowingTree,
            MazeAlgorithm::BinaryTree,
            MazeAlgorithm::Sidewinder,
        ]
    }
}

impl fmt::Display for MazeAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MazeAlgorithm::RecursiveBacktracker => write!(f, "Recursive Backtracker"),
            MazeAlgorithm::Kruskal => write!(f, "Kruskal"),
            MazeAlgorithm::Eller => write!(f, "Eller"),
            MazeAlgorithm::Prim => write!(f, "Prim"),
            MazeAlgorithm::RecursiveDivision => write!(f, "Recursive Division"),
            MazeAlgorithm::AldousBroder => write!(f, "Aldous Broder"),
            MazeAlgorithm::Wilson => write!(f, "Wilson"),
            MazeAlgorithm::HuntAndKill => write!(f, "Hunt and Kill"),
            MazeAlgorithm::GrowingTree => write!(f, "Growing Tree"),
            MazeAlgorithm::BinaryTree => write!(f, "Binary Tree"),
            MazeAlgorithm::Sidewinder => write!(f, "Sidewinder"),
        }
    }
}
