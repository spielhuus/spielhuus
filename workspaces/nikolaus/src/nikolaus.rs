const MOVES: [[usize; 5]; 5] = [
    [0, 1, 0, 1, 1],
    [1, 0, 1, 1, 1],
    [0, 1, 0, 1, 0],
    [1, 1, 1, 0, 1],
    [1, 1, 0, 1, 0],
];

pub type ResultItem = (Vec<usize>, Vec<(usize, usize)>);
pub type Result = Vec<ResultItem>;

fn extend_paths(current_paths: Result) -> Result {
    current_paths
        .into_iter()
        .flat_map(|path_item| {
            let last_node = *path_item.0.last().expect("Path should not be empty.");
            MOVES[last_node]
                .iter()
                .enumerate()
                .filter_map(move |(next_node, &can_move)| {
                    if can_move == 1
                        && !path_item.1.contains(&(last_node, next_node))
                        && !path_item.1.contains(&(next_node, last_node))
                    {
                        let mut new_path_item = path_item.clone();
                        new_path_item.0.push(next_node);
                        new_path_item.1.push((last_node, next_node));
                        Some(new_path_item)
                    } else {
                        None
                    }
                })
        })
        .collect()
}

pub fn nikolaus(start: usize) -> Result {
    // add the possible moves from the `start` node.
    let mut current_results: Result = MOVES[start]
        .iter()
        .enumerate()
        .filter_map(|(node, &possible_move)| {
            if possible_move == 1 {
                Some((vec![node], vec![(start, node)]))
            } else {
                None
            }
        })
        .collect();

    for _ in 0..7 {
        current_results = extend_paths(current_results);
    }

    current_results
}
