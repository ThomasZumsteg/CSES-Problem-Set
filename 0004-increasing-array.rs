use std::io::{self, BufRead};

fn increasing(mut nums: impl Iterator<Item=usize>) -> usize {
    let mut moves = 0;
    let mut last = nums.next().unwrap();
    for num in nums {
        if num < last {
            moves += last - num;
        } else {
            last = num
        }
    }
    moves
}

fn main() {
    let stdin = io::stdin();
    let count = stdin.lock().lines()
        .next().unwrap().unwrap()
        .parse::<usize>().unwrap();
    let line = stdin.lock().lines().
        next().unwrap().unwrap();
    let nums = line.split(" ").take(count)
        .map(|n| n.parse::<usize>().unwrap());
    println!("{}", increasing(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(increasing(vec![0].into_iter()), 0);
        assert_eq!(increasing(vec![0, 1].into_iter()), 0);
        assert_eq!(increasing(vec![1, 0].into_iter()), 1);
        assert_eq!(increasing(vec![1, 0, 2].into_iter()), 1);
        assert_eq!(increasing(vec![2, 0, 1].into_iter()), 3);
        assert_eq!(increasing(vec![3, 2, 5, 1, 7].into_iter()), 5);
    }
}
