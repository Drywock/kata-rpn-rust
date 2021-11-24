fn calc_rpn(expression: &str) -> u32 {
    if expression.is_empty() {
        0
    } else {
        let tokens = expression.split(' ');
        let mut stack = Vec::new();
        for token in tokens {
            let value: Result<u32, _> = token.parse();
            if value.is_ok() {
                stack.push(value.unwrap());
            } else {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                if token == "+" {
                    stack.push(b + a);
                } else if token == "-" {
                    stack.push(b - a);
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(calc_rpn(""), 0)
    }

    #[test]
    fn test_1_1_add() {
        assert_eq!(calc_rpn("1 1 +"), 2)
    }

    #[test]
    fn test_1_2_add() {
        assert_eq!(calc_rpn("1 2 +"), 3)
    }

    #[test]
    fn test_2_1_sub() {
        assert_eq!(calc_rpn("2 1 -"), 1)
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_1_2_sub_fail() {
        calc_rpn("1 2 -");
    }
}
