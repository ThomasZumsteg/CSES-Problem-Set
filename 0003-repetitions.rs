use std::io::{self, BufRead};

fn longest(chars: &str) -> usize {
    let mut last: Option<char> = None;
    let mut longest: usize = 0;
    let mut current: usize = 0;
    for chr in chars.chars() {
        if last != Some(chr) {
            last = Some(chr);
            current = 0;
        }
        current+= 1;
        longest = if current > longest { current } else { longest };
    }
    longest
}

fn main() {
    let stdin = io::stdin();
    let chars = stdin.lock()
        .lines().next()
        .unwrap().unwrap();
    println!("{}", longest(&chars));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(longest(""), 0);
        assert_eq!(longest("A"), 1);
        assert_eq!(longest("ACA"), 1);
        assert_eq!(longest("AGGA"), 2);
        assert_eq!(longest("ABBATTT"), 3);
    }
}
