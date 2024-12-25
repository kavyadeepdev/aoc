fn main () {
    let input = include_str!("./input2.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> i32 {
    let mut answer: i32 = 0;
    let lines = input.lines();
    for line in lines {
        let mut dimensions: Vec<i32> = line.split("x").map(|s| s.trim().parse().expect("failed to parse &str into i32")).collect();
        dimensions.sort();
        if dimensions.len() == 3 {
            let l1 = dimensions[0];
            let l2 = dimensions[1];
            let l3 = dimensions[2];
            let length = 2 * (l1 + l2) + l1 * l2 *l3;
            answer += length;
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
        assert_eq!(result, 34);
    }
}

