const MOVES: [[usize; 5]; 5] = [
    [0, 1, 0, 1, 1],
    [1, 0, 1, 1, 1],
    [0, 1, 0, 1, 0],
    [1, 1, 1, 0, 1],
    [1, 1, 0, 1, 0],
];

pub type ResultItem = (Vec<usize>, Vec<(usize, usize)>);
pub type Result = Vec<ResultItem>;

fn next_step(result: Result, start: usize) -> Result {
    let mut new_result: Result = Vec::new();
    if result.is_empty() {
        for (node, possible_move) in MOVES[start].iter().enumerate() {
            if *possible_move == 1 {
                let mut new_move = (Vec::<usize>::new(), Vec::<(usize, usize)>::new());
                new_move.0.push(node);
                new_move.1.push((start, node));
                new_result.push(new_move);
            }
        }
    } else {
        for i in 0..result.len() {
            let last_move = result[i].0.last().unwrap();
            for (node, possible_move) in MOVES[*last_move].iter().enumerate() {
                if *possible_move == 1
                    && !result[i].1.contains(&(*last_move, node))
                    && !result[i].1.contains(&(node, *last_move))
                {
                    let mut new_move = result[i].clone();
                    new_move.0.push(node);
                    new_move.1.push((*last_move, node));
                    new_result.push(new_move);
                }
            }
        }
    }
    new_result
}

pub fn nikolaus(start: usize) -> Result {
    let mut result: Result = Result::new();
    for _ in 0..8 {
        result = next_step(result, start);
    }
    result
}
