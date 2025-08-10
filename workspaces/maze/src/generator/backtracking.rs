use rand::prelude::*;

use crate::{Board, CURSOR_COLOR, Generator, State};
use raylib_egui_rs::raylib;

#[derive(Default)]
pub struct Backtracking {
    current: usize,
    rng: ThreadRng,
}

impl Backtracking {
    pub fn new() -> Self {
        Self {
            current: 0,
            rng: rand::rng(),
        }
    }
}

impl Generator for Backtracking {
    fn step(&mut self, board: &mut Board) -> State {
        let n = board.neighbors(self.current);
        let free: Option<&Option<usize>> = n
            .iter()
            .filter(|i| i.is_some() && !board.cells[i.unwrap()].visited)
            .choose(&mut self.rng);

        if let Some(&Some(free)) = free {
            // remove the walls
            board.remove_wall(self.current, free);
            // set next cell as current
            board.cells[free].visited = true;
            self.current = free;
            board.path.push(free)
        } else if let Some(last) = board.path.pop() {
            self.current = last;
        } else {
            return State::GenerationDone;
        }

        State::Generate
    }

    fn draw(&self, board: &Board) {
        // draw the result
        raylib::DrawCircle(
            (board.x + board.cells[self.current].x * board.cell_size + board.cell_size / 2) as i32,
            (board.y + board.cells[self.current].y * board.cell_size + board.cell_size / 2) as i32,
            board.cell_size as f32 / 10.0,
            CURSOR_COLOR,
        );
    }
}
