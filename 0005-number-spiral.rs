use std::io::{self, BufRead};
use std::cmp::max;

fn spiral(x: usize, y: usize) -> usize {
    assert!(x > 0 && y > 0);
    let n = max(x, y) - 1;
    let middle_num = 1 + n * (n + 1);
    if (x > y && x % 2 == 0) || (x < y && y % 2 == 0) {
        middle_num + x - y
    } else {
        middle_num + y - x
    }
    
}


fn main() {
    let stdin = io::stdin();
    let count = stdin.lock().lines()
        .next().unwrap().unwrap()
        .parse::<usize>().unwrap();
    for _ in 0..count {
        let line = stdin.lock().lines().
            next().unwrap().unwrap();
        let mut nums = line.split(" ");
        println!("{}", spiral(
            nums.next().unwrap().parse::<usize>().unwrap(),
            nums.next().unwrap().parse::<usize>().unwrap(),
        ));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(spiral(1, 1), 1);
        assert_eq!(spiral(2, 2), 3);
        assert_eq!(spiral(3, 3), 7);
        assert_eq!(spiral(2, 3), 8);
        assert_eq!(spiral(4, 2), 15);
    }
}
