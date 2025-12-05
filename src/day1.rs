use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn day1_main() -> io::Result<()> {
    let file =
        File::open("./material/day1_puzzle_document1.txt").expect("Failed to open the file.");
    let reader = io::BufReader::new(file);
    let mut dial: i32 = 50;
    let mut password: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        print!("Dial {dial} : ");
        println!("{line}");
        treat_line(&line, &mut dial, &mut password);
    }
    print!("Password : {password}\n");

    Ok(())
}
fn treat_line(file_reff: &str, dial: &mut i32, password: &mut i32) {
    let direction: char = file_reff.chars().next().unwrap();
    let distance: i32 = file_reff[1..].trim().parse().unwrap();

    match direction {
        'R' => {
            *password += (*dial + distance) / 100;
            *dial = (*dial + distance) % 100;
        }
        'L' => {
            let gap = if *dial == 0 { 100 } else { *dial };
            if distance >= gap {
                *password += 1 + (distance - gap) / 100;
            }
            *dial = ((*dial - distance) % 100 + 100) % 100;
        }
        _ => {}
    }
}
