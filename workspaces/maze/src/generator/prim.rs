use rand::prelude::*;

use crate::{Board, CURSOR_COLOR, Generator, State};

use raylib_egui_rs::color::Color;
use raylib_egui_rs::raylib;

#[derive(Debug)]
struct FreeCell {
    index: usize,
    neighbor: usize,
}

pub struct Prim {
    visited: Vec<usize>,
    cells: Vec<FreeCell>,
    rng: ThreadRng,
}

impl Prim {
    pub fn new(board: &Board) -> Self {
        let mut rng = rand::rng();
        let current = rng.random_range(0..board.board_size ^ 2) as usize;
        let cells = board
            .neighbors(current)
            .into_iter()
            .flatten()
            .map(|index| FreeCell {
                index,
                neighbor: current,
            })
            .collect();

        Self {
            visited: vec![current],
            cells,
            rng,
        }
    }

    fn contains(&self, index: &usize) -> bool {
        self.cells
            .iter()
            .filter(|item| item.index == *index)
            .count()
            > 0
            || self.visited.contains(index)
    }
}

impl Generator for Prim {
    fn step(&mut self, board: &mut Board) -> State {
        let index = self.rng.random_range(0..self.cells.len());
        let item = self.cells.remove(index);

        // remove wall
        board.remove_wall(item.index, item.neighbor);

        // calc next cells
        let neighbors: Vec<usize> = board.neighbors(item.index).into_iter().flatten().collect();
        for n in &neighbors {
            if !self.contains(n) {
                self.cells.push(FreeCell {
                    index: *n,
                    neighbor: item.index,
                });
            }
        }

        self.visited.push(item.index);

        if self.cells.is_empty() {
            State::GenerationDone
        } else {
            State::Generate
        }
    }

    fn draw(&self, board: &Board) {
        // draw the next cells
        for i in &self.cells {
            raylib::DrawCircle(
                (board.x + board.cells[i.index].x * board.cell_size + board.cell_size / 2) as i32,
                (board.y + board.cells[i.index].y * board.cell_size + board.cell_size / 2) as i32,
                board.cell_size as f32 / 5.0,
                CURSOR_COLOR,
            );
        }
    }
}
