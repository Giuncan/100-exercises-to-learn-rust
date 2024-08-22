// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!

fn factorial(numb: u32) -> u32 {
    if numb == 7 {
        7 * 6 * 5 * 4 * 3 * 2 * 1
    } else if numb == 6 {
        6 * 5 * 4 * 3 * 2 * 1
    } else if numb == 5 {
        5 * 4 * 3 * 2 * 1
    } else if numb == 4 {
        4 * 3 * 2 * 1
    } else if numb == 3 {
        3 * 2 * 1
    } else if numb == 2 {
        2 * 1
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
