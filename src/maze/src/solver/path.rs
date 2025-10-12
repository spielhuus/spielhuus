use crate::{Board, Cell};

#[derive(Debug)]
pub enum PathDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Horizontal,
    Vertical,
    StartLeft,
    StartDown,
    StartUp,
    StartRight,
    EndLeft,
    EndDown,
    EndUp,
    EndRight,
}

impl From<PathDirection> for u32 {
    fn from(direction: PathDirection) -> u32 {
        match direction {
            PathDirection::Horizontal => crate::PATH_HORIZONTAL,
            PathDirection::Vertical => crate::PATH_VERTICAL,
            PathDirection::UpLeft => crate::PATH_UP_LEFT,
            PathDirection::UpRight => crate::PATH_UP_RIGHT,
            PathDirection::DownLeft => crate::PATH_DOWN_LEFT,
            PathDirection::DownRight => crate::PATH_DOWN_RIGHT,
            PathDirection::StartLeft => crate::START_LEFT,
            PathDirection::StartRight => crate::START_RIGHT,
            PathDirection::StartUp => crate::START_UP,
            PathDirection::StartDown => crate::START_DOWN,
            PathDirection::EndLeft => crate::END_LEFT,
            PathDirection::EndRight => crate::END_RIGHT,
            PathDirection::EndUp => crate::END_UP,
            PathDirection::EndDown => crate::END_DOWN,
        }
    }
}

pub fn update_path(board: &mut Board, path: &[usize]) {
    if path.len() >= 3 {
        // second last step in path
        let dir = direction(
            &board.cells[path[path.len() - 2]],
            Some(&board.cells[path[path.len() - 3]]),
            Some(&board.cells[path[path.len() - 1]]),
        );
        clear_direction(board, path[path.len() - 2]);
        board.gpu_data[path[path.len() - 2]][0] |=
            <PathDirection as std::convert::Into<u32>>::into(dir);
        // end of path
        let dir = direction(
            &board.cells[path[path.len() - 1]],
            Some(&board.cells[path[path.len() - 2]]),
            None,
        );
        clear_direction(board, path[path.len() - 1]);
        board.gpu_data[path[path.len() - 1]][0] |=
            <PathDirection as std::convert::Into<u32>>::into(dir);
    }
    if path.len() == 2 {
        let dir = direction(
            &board.cells[path[path.len() - 2]],
            None,
            Some(&board.cells[path[path.len() - 1]]),
        );
        clear_direction(board, path[path.len() - 2]);
        board.gpu_data[path[path.len() - 2]][0] |=
            <PathDirection as std::convert::Into<u32>>::into(dir);
    }
}

pub fn clear_direction(board: &mut Board, cell: usize) {
    board.gpu_data[cell][0] &= !crate::PATH_HORIZONTAL;
    board.gpu_data[cell][0] &= !crate::PATH_VERTICAL;
    board.gpu_data[cell][0] &= !crate::PATH_UP_LEFT;
    board.gpu_data[cell][0] &= !crate::PATH_UP_RIGHT;
    board.gpu_data[cell][0] &= !crate::PATH_DOWN_LEFT;
    board.gpu_data[cell][0] &= !crate::PATH_DOWN_RIGHT;
    board.gpu_data[cell][0] &= !crate::START_LEFT;
    board.gpu_data[cell][0] &= !crate::START_RIGHT;
    board.gpu_data[cell][0] &= !crate::START_UP;
    board.gpu_data[cell][0] &= !crate::START_DOWN;
    board.gpu_data[cell][0] &= !crate::END_LEFT;
    board.gpu_data[cell][0] &= !crate::END_RIGHT;
    board.gpu_data[cell][0] &= !crate::END_UP;
    board.gpu_data[cell][0] &= !crate::END_DOWN;
}

pub fn direction(current: &Cell, prev: Option<&Cell>, next: Option<&Cell>) -> PathDirection {
    // +---+---+---+    current.y < previous.y &&
    // +   +   +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // +   + c + n +    current.x < next.x
    // +---+---+---+
    // +   + p +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   +   +   +    current.y < next.y &&
    // +---+---+---+    current.x < previous.x &&
    // +   + c + p +    currnet.x == next.x
    // +---+---+---+
    // +   + n +   +
    // +---+---+---+
    //
    // +---+---+---+    current.y < previous.y &&
    // +   +   +   +    current.y == next.y &&
    // +---+---+---+    currnet.x == previous.x &&
    // + n + c +   +    current.x > next.x
    // +---+---+---+
    // +   + p +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   +   +   +    current.y < next.y &&
    // +---+---+---+    current.x > previous.x &&
    // + p + c +   +    current.x == next.x
    // +---+---+---+
    // +   + n +   +
    // +---+---+---+

    //
    // +---+---+---+    current.y > previous.y &&
    // +   + p +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // +   + c + n +    currnet.x < next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   + n +   +    current.y > next.y &&
    // +---+---+---+    current.x < prvious.x &&
    // +   + c + p +    current.x == next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    //
    // +---+---+---+    current.y > previous.y &&
    // +   + p +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // + n + c +   +    current.x > next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   + n +   +    current.y > next.y &&
    // +---+---+---+    current.x > previous.x &&
    // + p + c +   +    current.x == next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    //
    if let (Some(prev), Some(next)) = (prev, next) {
        if current.x == next.x && current.x == prev.x {
            return PathDirection::Vertical;
        } else if current.y == next.y && current.y == prev.y {
            return PathDirection::Horizontal;
        } else if (current.y < prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x < next.x)
            || (current.y == prev.y
                && current.y < next.y
                && current.x < prev.x
                && current.x == next.x)
        {
            return PathDirection::DownRight;
        } else if (current.y < prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x > next.x)
            || (current.y == prev.y
                && current.y < next.y
                && current.x > prev.x
                && current.x == next.x)
        {
            return PathDirection::DownLeft;
        } else if (current.y > prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x < next.x)
            || (current.y == prev.y
                && current.y > next.y
                && current.x < prev.x
                && current.x == next.x)
        {
            return PathDirection::UpRight;
        } else if (current.y > prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x > next.x)
            || (current.y == prev.y
                && current.y > next.y
                && current.x > prev.x
                && current.x == next.x)
        {
            return PathDirection::UpLeft;
        }
    } else if let Some(next) = next {
        if next.x > current.x {
            return PathDirection::StartRight;
        } else if next.x < current.x {
            return PathDirection::StartLeft;
        } else if next.y > current.y {
            return PathDirection::StartDown;
        } else if next.y < current.y {
            return PathDirection::StartUp;
        } else {
            panic!("direction not found")
        }
    } else if let Some(prev) = prev {
        if prev.x > current.x {
            return PathDirection::EndLeft;
        } else if prev.x < current.x {
            return PathDirection::EndRight;
        } else if prev.y > current.y {
            return PathDirection::EndUp;
        } else if prev.y < current.y {
            return PathDirection::EndDown;
        } else {
            panic!("direction not found")
        }
    }
    panic!("direction not found for cell: {:?}", current)
}
