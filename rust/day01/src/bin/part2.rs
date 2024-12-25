fn main () {
    let input = include_str!("./input2.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let mut story: i32 = 0;
    let mut answer: usize = 0;
    for (index, ch) in input.char_indices() {
        if ch == '(' {
            story += 1;
        } else if ch == ')' {
            story -= 1;
        }

        if story < 0 {
            answer = index + 1;
            break;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solve("()())");
        assert_eq!(result, 5);
    }
}

