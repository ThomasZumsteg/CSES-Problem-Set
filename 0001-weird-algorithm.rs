use std::io::{self, BufRead};
use std::process::exit;

struct Collatz {
    num: Option<usize>
}

impl Collatz {
    fn new(start: usize) -> Collatz {
        Collatz { num: Some(start) }
    }
}

impl Iterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.num;
        if value == Some(1) {
            self.num = None
        };
        self.num = self.num.map(|num| {
            if num % 2 == 0 { num / 2 }
            else { num * 3 + 1 }
        });
        value
    }
}

fn collatz(start: usize) -> impl Iterator<Item=usize> {
    Collatz::new(start)
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    if let Ok(num) = input.parse::<usize>() {
        println!("{}", collatz(num)
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
        );
        exit(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base_case() {
        assert!(Collatz::new(1).collect::<Vec<usize>>() == [1])
    }

    #[test]
    fn test_small_integers() {
        let tests = vec!{
            (1, vec!{1}),
            (2, vec!{2, 1}),
            (3, vec!{3, 10, 5, 16, 8, 4, 2, 1}),
        };
        for (test, result) in tests {
            assert!(Collatz::new(test).collect::<Vec<usize>>() == result)
        }
    }
}
