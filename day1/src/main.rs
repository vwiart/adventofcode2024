use std::fs;

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

fn process(left: &mut Vec<i32>, right: &mut Vec<i32>) -> Result<i32, String> {
    if left.len() != right.len() {
        return Err("left and right have different length".into());
    }
    left.sort();
    right.sort();
    Ok(left
        .iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + (*l - *r).abs()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() {
        let mut left: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let mut right: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(super::process(&mut left, &mut right).unwrap(), 11);
    }

    #[test]
    fn test_not_same_length() {
        let mut left: Vec<i32> = vec![1, 2, 3];
        let mut right: Vec<i32> = vec![4, 5];
        assert_eq!(
            super::process(&mut left, &mut right).unwrap_err(),
            "left and right have different length"
        );
    }
}

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    read_file(&mut left, &mut right).expect("unable to read file");

    match process(&mut left, &mut right) {
        Ok(distance) => println!("Total distance is {}", distance),
        Err(e) => println!("Error: {}", e),
    };
}
