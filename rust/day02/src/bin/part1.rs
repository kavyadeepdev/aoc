use std::cmp::min;

fn main () {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> i32 {
    let mut answer: i32 = 0;
    let lines = input.lines();
    for line in lines {
        let dimensions: Vec<&str> = line.split("x").collect();
        // println!("l: {}, b: {}, h: {}", dimensions[0], dimensions[1], dimensions[2]);
        if dimensions.len() == 3 {
            let l: i32 = dimensions[0].parse().unwrap_or(0);
            let w: i32 = dimensions[1].parse().unwrap_or(0);
            let h: i32 = dimensions[2].parse().unwrap_or(0);
            let area = 2*l*w + 2*w*h + 2*h*l + min(l*w, min(w*h, h*l));
            answer += area;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solve("2x3x4");
        assert_eq!(result, 58);
    }
}
