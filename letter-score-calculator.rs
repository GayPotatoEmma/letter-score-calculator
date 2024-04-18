use std::io;

fn main() {
    println!("Enter your score (0-100):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let score: u32 = input.trim().parse().expect("Invalid input");

    let grade = get_grade(score);
    println!("Your grade is: {}", grade);
}

fn get_grade(score: u32) -> &'static str {
    if score >= 90 && score <= 100 {
        "A"
    } else if score >= 80 && score < 90 {
        "B"
    } else if score >= 70 && score < 80 {
        "C"
    } else if score >= 60 && score < 70 {
        "D"
    } else if score >= 0 && score < 60 {
        "F"
    } else {
        "Invalid score"
    }
}
