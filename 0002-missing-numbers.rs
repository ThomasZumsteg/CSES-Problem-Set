use std::io::{self, BufRead};
use std::collections::HashSet;

fn missing(limit: usize, nums: HashSet<usize>) -> HashSet<usize> {
    (1..limit+1).filter(|n| !nums.contains(n)).collect()
}

fn main() {
    let stdin = io::stdin();
    let start = stdin.lock()
        .lines().next()
        .unwrap().unwrap()
        .parse::<usize>().unwrap();
    let nums = stdin.lock()
        .lines().next()
        .unwrap().unwrap().split(' ')
        .map(|n| { n.parse::<usize>().unwrap() })
        .collect();
    println!("{}", missing(start, nums).iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ")
    );
}

#[cfg(test)]
mod test {
    #[test]
    fn test_simple() {
        assert_eq!(missing(5, vec!{2,3,1,5}.into()), vec!{4}.into());
    }
}
