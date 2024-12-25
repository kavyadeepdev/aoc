use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> i32 {
    let mut is_visited = HashMap::new();
    let mut visited = 1;
    let mut x = 0;
    let mut y = 0;
    is_visited.insert((x, y), true);
    for ch in input.chars() {
        match ch {
            '^' => y += 1,
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            _ => println!("skipping character {}", ch),
        };
        if !is_visited.contains_key(&(x, y)) {
            visited += 1;
        } else {
            is_visited.insert((x, y), true);
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solve("^v^v^v^v^v");
        assert_eq!(result, 2);
    }
}
