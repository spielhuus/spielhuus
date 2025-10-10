use crate::{Board, Solver, MazeState};

// use raylib_egui_rs::color::Color;
// use raylib_egui_rs::raylib;

#[derive(Default, Clone, Copy, Debug)]
pub struct Weight {
    pub x: usize,
    pub y: usize,
    pub weight: usize,
}

pub struct Djikstra {
    start: (usize, usize),
    end: (usize, usize),
    positions: Vec<usize>,
    pub path: Vec<usize>,
    pub weights: Vec<Option<Weight>>,
    pub reached_end: bool,
    pub solved: bool,
}

impl Djikstra {
    pub fn new(board: &Board) -> Self {
        let mut weights = vec![None; (board.board_size) * (board.board_size)];
        weights[0] = Some(Weight {
            x: 0,
            y: 0,
            weight: 1,
        });
        Self {
            start: (0, 0),
            end: (board.board_size - 1, board.board_size - 1),
            positions: vec![0],
            path: vec![],
            weights,
            reached_end: false,
            solved: false,
        }
    }

    fn get_max_weight(&self) -> usize {
        self.weights
            .iter()
            .flatten()
            .max_by(|a, b| a.weight.cmp(&b.weight))
            .unwrap()
            .weight
    }

    fn search_path(&mut self, board: &Board) -> MazeState {
        let mut next_cells: Vec<usize> = vec![];
        for index in &self.positions {
            let weight = self.weights[*index].unwrap();
            let neighbors = board.neighbors(*index);
            let free: Vec<(usize, &Option<usize>)> = neighbors
                .iter()
                .enumerate()
                .filter(|&(d, i)| {
                    if (i.is_some() && self.weights[i.unwrap()].is_none())
                        && ((d == 0 && !board.cells[*index].walls.top)
                            || (d == 1 && !board.cells[*index].walls.bottom)
                            || (d == 2 && !board.cells[*index].walls.left)
                            || (d == 3 && !board.cells[*index].walls.right))
                    {
                        return true;
                    }
                    false
                })
                .collect();

            if !free.is_empty() {
                for (_, j) in free {
                    self.weights[j.unwrap()] = Some(Weight {
                        x: board.cells[j.unwrap()].x,
                        y: board.cells[j.unwrap()].y,
                        weight: weight.weight + 1,
                    });
                    next_cells.push(j.unwrap());
                    if self.weights[j.unwrap()].unwrap().x == self.end.0
                        && self.weights[j.unwrap()].unwrap().y == self.end.1
                    {
                        self.reached_end = true;
                        self.path.push(j.unwrap())
                    }
                }
            }
        }
        self.positions = next_cells;
        MazeState::Solve
    }
    fn path(&mut self, board: &Board) -> MazeState {
        let index: usize = *self.path.last().unwrap();
        let neighbors = board.neighbors(index);
        let mut free: Vec<(usize, &Option<usize>)> = neighbors
            .iter()
            .enumerate()
            .filter(|&(d, i)| {
                if (i.is_some()
                    && !self.path.contains(&i.unwrap())
                    && self.weights[i.unwrap()].is_some())
                    && ((d == 0 && !board.cells[index].walls.top)
                        || (d == 1 && !board.cells[index].walls.bottom)
                        || (d == 2 && !board.cells[index].walls.left)
                        || (d == 3 && !board.cells[index].walls.right))
                {
                    return true;
                }
                false
            })
            .collect();
        free.sort_by(|a, b| {
            self.weights[a.1.unwrap()]
                .unwrap()
                .weight
                .cmp(&self.weights[b.1.unwrap()].unwrap().weight)
        });
        if let Some(next) = free.first() {
            self.path.push(next.1.unwrap());
            if self.weights[next.1.unwrap()].unwrap().x == self.start.0
                && self.weights[next.1.unwrap()].unwrap().y == self.start.1
            {
                self.solved = true;
                return MazeState::Solve;
            }
        }

        MazeState::Solve
    }
}

impl Solver for Djikstra {
    fn step(&mut self, board: &mut Board) -> Result<MazeState, String> {
        if self.solved {
            Ok(MazeState::Done)
        } else if !self.reached_end {
            Ok(self.search_path(board))
        } else if !self.solved {
            Ok(self.path(board))
        } else {
            panic!("unknown state")
        }
    }

    fn get_path(&self) -> &Vec<usize> {
        &self.path
    }

    fn draw(&self, board: &Board) {
        // // draw the result
        // if !self.solved {
        //     for (index, weight) in self.weights.iter().enumerate() {
        //         if let Some(weight) = weight {
        //             if self.path.contains(&index) {
        //                 raylib::DrawCircle(
        //                     (board.x + weight.x * board.cell_size + board.cell_size / 2) as i32,
        //                     (board.y + weight.y * board.cell_size + board.cell_size / 2) as i32,
        //                     board.cell_size as f32 / 5.0,
        //                     Color::WHITE,
        //                 );
        //             } else {
        //                 raylib::DrawCircle(
        //                     (board.x + weight.x * board.cell_size + board.cell_size / 2) as i32,
        //                     (board.y + weight.y * board.cell_size + board.cell_size / 2) as i32,
        //                     board.cell_size as f32 / 5.0,
        //                     raylib::ColorFromHSV(
        //                         115.0,
        //                         0.75,
        //                         1.0 / self.get_max_weight() as f32 * weight.weight as f32,
        //                     ),
        //                 );
        //             }
        //         }
        //     }
        // }
    }
}
