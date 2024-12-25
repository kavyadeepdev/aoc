fn main () {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> i32 {
    let mut answer: i32 = 0;
    for ch in input.chars() {
        if ch == '(' {
            answer += 1;
        } else if ch == ')' {
            answer -= 1;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solve("))(((((");
        assert_eq!(result, 3);
    }
}
