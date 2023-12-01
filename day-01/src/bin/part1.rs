use std::fs::read_to_string;

fn main() {
    println!("{}", part1(String::from("./src/bin/input1.txt")));
}

fn part1(input: String) -> i32 {
    let mut result = Vec::new();
    for line in read_to_string(input).unwrap().lines() {
        let mut tmp = String::from("");
        for c in line.chars() { 
            if c.is_digit(10) {
                tmp.push(c);
            }
        }
        if tmp.chars().count() > 2 {
           let first = tmp.chars().nth(0).unwrap();
           let last = tmp.chars().nth(tmp.chars().count() - 1).unwrap();
           tmp = String::from("");
           tmp.push(first);
           tmp.push(last);
        }
        if tmp.chars().count() < 2 {
            tmp.push(tmp.parse::<char>().unwrap());
        }
        result.push(tmp.parse::<i32>().unwrap());
    }
    println!("{:?}", result);
    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        assert_eq!(part1(String::from("./src/bin/test1.txt")), 142);
    }
}
