use std::fs;

const FILENAME: &str = "input.txt";

#[derive(PartialEq, Eq)]
enum Change {
    Increase,
    Decrease,
    None,
}

fn read_file(matrix: &mut Vec<Vec<u8>>) -> Result<(), String> {
    let content = fs::read_to_string(FILENAME).expect("Failed to read the file");

    for l in content.lines() {
        let mut row: Vec<u8> = Vec::new();
        for v in l.split_whitespace().enumerate().map(|x| x.1.parse::<u8>()) {
            row.push(v.unwrap());
        }
        matrix.push(row);
    }
    Ok(())
}

fn process_row(row: &Vec<u8>) -> Result<bool, String> {
    if row.len() < 2 {
        return Err("Not enough element in the row".into());
    }

    let mut direction = Change::None;
    for (i, current) in row.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let previous = row[i - 1] as i8;
        let diff = previous - *current as i8;
        if diff == 0 || diff.abs() >= 4 {
            return Ok(false);
        } else if direction == Change::None && diff > 0 {
            direction = Change::Increase;
            continue;
        } else if direction == Change::None && diff < 0 {
            direction = Change::Decrease;
            continue;
        } else if direction == Change::Increase && diff < 0 {
            return Ok(false);
        } else if direction == Change::Decrease && diff > 0 {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_row() {
        assert_eq!(
            process_row(&vec![]).unwrap_err(),
            "Not enough element in the row"
        );
        assert_eq!(
            process_row(&vec![1]).unwrap_err(),
            "Not enough element in the row"
        );
        let matrix: Vec<Vec<u8>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(
            matrix
                .iter()
                .filter(|row| process_row(row).unwrap())
                .count(),
            2
        );
    }
}

fn main() {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    read_file(&mut matrix).expect("Unable to read the file");

    let count = matrix
        .iter()
        .filter(|row| process_row(row).unwrap())
        .count();
    println!("Total: {:?}", count);
}
