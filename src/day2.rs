use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day2_main() -> io::Result<()> {
    let file = File::open("./material/day2_puzzle_document1.txt")?;
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let ranges: Vec<&str> = first_line.split(',').collect();
    let mut _sum: u64 = 0;
    for range in ranges {
        let _ = get_invalid_id(range);
    }

    Ok(())
}

fn get_invalid_id(range: &str) {
    let numbers: Vec<&str> = range.split('-').collect();
    for number in numbers {
        print!("number: {number}\n");
    }
    // let number1: i32 = numbers[0].parse().unwrap();
    // let number2: i32 = numbers[1].parse().unwrap();
    // let mut invalid_ids: Vec<i32> = vec![];
    // for i in number1..number2 {
    //     let s_i = i.to_string();
    //     if repeating_characters(&s_i) {
    //         let s_i_number: i32 = s_i.parse().unwrap();
    //         invalid_ids.push(s_i_number);
    //     }
    // }
}
fn repeating_characters(s_i: &String) -> bool {
    for c in s_i.chars() {
        print!("c : {c}i\n");
    }
    return true;
}
