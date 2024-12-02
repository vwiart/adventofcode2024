use std::{collections::HashMap, fs};

const FILENAME: &str = "input.txt";

fn read_file(left: &mut Vec<i32>, right: &mut Vec<i32>) -> Result<(), String> {
    let content = fs::read_to_string(FILENAME).expect("Failed to read the file");

    for l in content.lines() {
        let parsed = sscanf::scanf!(l, "{}   {}", i32, i32);
        let (a, b) = parsed.expect("Error parsing line");
        left.push(a);
        right.push(b);
    }
    Ok(())
}

fn process(left: &Vec<i32>, right: &Vec<i32>) -> Result<i32, String> {
    if left.len() != right.len() {
        return Err("Left and right have different length".into());
    }
    Ok(left
        .iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + (*l - *r).abs()))
}

fn process2(left: &Vec<i32>, right: &Vec<i32>) -> Result<i32, String> {
    if left.len() != right.len() {
        return Err("Left and right have different length".into());
    }
    let mut map: HashMap<i32, i32> = HashMap::new();

    for item in right {
        *map.entry(*item).or_insert(0) += 1;
    }
    Ok(left.iter().fold(0, |acc, item| {
        acc + item
            * match map.get(item) {
                Some(v) => *v,
                None => 0,
            }
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() {
        let mut left: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let mut right: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
        left.sort();
        right.sort();
        assert_eq!(super::process(&left, &right).unwrap(), 11);
    }

    #[test]
    fn test_process2() {
        let mut left: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let mut right: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
        left.sort();
        right.sort();
        assert_eq!(super::process2(&left, &right).unwrap(), 31);
    }

    #[test]
    fn test_not_same_length() {
        let mut left: Vec<i32> = vec![1, 2, 3];
        let mut right: Vec<i32> = vec![4, 5];
        assert_eq!(
            super::process(&mut left, &mut right).unwrap_err(),
            "Left and right have different length"
        );
    }
}

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    read_file(&mut left, &mut right).expect("Unable to read file");

    left.sort();
    right.sort();
    match process(&left, &right) {
        Ok(distance) => println!("Total distance is {}", distance),
        Err(e) => println!("Error: {}", e),
    };

    match process2(&left, &right) {
        Ok(distance) => println!("Total distance is {}", distance),
        Err(e) => println!("Error: {}", e),
    };
}
