#[derive(PartialEq)]
#[repr(u8)]
pub enum Direction {
    Down,
    Up,
    First,
    Last,
}

pub fn go(dir: Direction, size: usize, index: &mut usize) {
    match dir {
        Direction::Down => {
            if size > 0 {
                *index = (*index + 1) % size;
            }
        }
        Direction::Last => {
            if *index + 1 < size {
                *index = size - 1;
            }
        }
        Direction::Up => {
            if size > 0 {
                if *index > 0 {
                    *index = (*index - 1) % size;
                } else {
                    *index = size - 1;
                }
            }
        }
        Direction::First => {
            if *index > 0 {
                *index = 0;
            }
        }
    }
}

pub fn delete(src: &mut Vec<String>, curr: &mut usize) {
    if *curr < src.len() {
        src.remove(*curr);
        if *curr >= src.len() {
            go(Direction::Up, src.len(), curr);
        }
    }
}

pub fn drag(dir: Direction, src: &mut [String], curr: &mut usize) {
    match dir {
        Direction::Down => {
            if *curr + 1 < src.len() {
                src.swap(*curr, *curr + 1);
                *curr += 1;
            }
        }
        Direction::Up => {
            if *curr > 0 {
                src.swap(*curr, *curr - 1);
                *curr -= 1;
            }
        }
        _ => {}
    }
}

pub fn transfer(dst: &mut Vec<String>, src: &mut Vec<String>, curr: &mut usize) {
    if !src.is_empty() && *curr < src.len() {
        dst.push(src.remove(*curr));
        if *curr >= src.len() {
            go(Direction::Up, src.len(), curr);
        }
    }
}
