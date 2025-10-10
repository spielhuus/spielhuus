use disjoint::DisjointSet;
use rand::prelude::*;

use crate::{Board, Generator, MazeState};

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    West,
}

#[derive(Debug)]
struct Edge {
    x: usize,
    y: usize,
    direction: Direction,
}

pub struct Kruskal {
    edges: Vec<Edge>,
    cells: Vec<(usize, usize, usize)>,
    merged: DisjointSet,
    visited_edges: Vec<Edge>,
    step: usize,
}

impl Kruskal {
    pub fn new(board: &Board) -> Self {
        let mut rng = rand::rng();
        // pupulate the edges
        let mut edges: Vec<Edge> = vec![];
        for y in 0..board.board_size {
            for x in 0..board.board_size {
                if y > 0 {
                    edges.push(Edge {
                        x,
                        y,
                        direction: Direction::North,
                    })
                }
                if x > 0 {
                    edges.push(Edge {
                        x,
                        y,
                        direction: Direction::West,
                    })
                }
            }
        }
        edges.shuffle(&mut rng);

        Self {
            edges,
            cells: Vec::new(),
            merged: DisjointSet::with_len(board.cells.len()),
            visited_edges: Vec::new(),
            step: 1,
        }
    }
}

impl Generator for Kruskal {
    fn step(&mut self, board: &mut Board) -> MazeState {
        let edge: Option<Edge> = self.edges.pop();
        if let Some(edge) = edge {
            let index_cell = board.get_index(edge.x, edge.y);
            let index_neighbor = if edge.direction == Direction::North {
                board.get_index(edge.x, edge.y - 1)
            } else {
                board.get_index(edge.x - 1, edge.y)
            };

            if !self.merged.is_joined(index_cell, index_neighbor) {
                self.merged.join(index_cell, index_neighbor);
                self.cells.push((self.step, index_cell, index_neighbor));

                //remove walls
                match edge.direction {
                    Direction::North => {
                        board.cells[index_cell].walls.top = false;
                        board.cells[index_neighbor].walls.bottom = false;
                    }
                    Direction::West => {
                        board.cells[index_cell].walls.left = false;
                        board.cells[index_neighbor].walls.right = false;
                    }
                }
                board.cells[index_cell].visited = true;
                board.cells[index_neighbor].visited = true;
            }
            self.visited_edges.push(edge);
        } else {
            return MazeState::GenerationDone;
        }

        self.step += 1;
        MazeState::Generate
    }

    fn draw(&self, _board: &Board) {}
}
