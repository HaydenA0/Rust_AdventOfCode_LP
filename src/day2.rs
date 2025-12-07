use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day2_main() -> io::Result<()> {
    let file = File::open("./material/day2_puzzle_document1.txt")?;
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let ranges: Vec<&str> = first_line.split(',').collect();
    let mut sum: u64 = 0;
    for range in ranges {
        let invalid_ids = get_invalid_id(range);
        print!("number of invalid_ids : {}\n", invalid_ids.len());
        print!("Invalid IDs\n : ");
        for id in invalid_ids {
            print!("{} - ", id);
            sum += id;
        }
        print!("\n");
    }
    print!("The final sum is {sum}\n");

    Ok(())
}

fn get_invalid_id(range: &str) -> Vec<u64> {
    let numbers: Vec<&str> = range.split('-').collect();
    let number1: u64 = numbers[0].parse().unwrap();
    let number2: u64 = numbers[1].trim().parse().unwrap();
    let mut invalid_ids: Vec<u64> = vec![];
    for i in number1..number2 + 1 {
        let s_i = i.to_string();
        if repeating_characters(&s_i) {
            let s_i_number: u64 = s_i.parse().unwrap();
            invalid_ids.push(s_i_number);
        }
    }
    return invalid_ids;
}
fn repeating_characters(s_i: &String) -> bool {
    if s_i.len() % 2 != 0 {
        return false;
    }
    let mid = s_i.chars().count() / 2;
    let (left, right) = s_i.split_at(mid);

    if left != right {
        return false;
    }
    return true;
}
