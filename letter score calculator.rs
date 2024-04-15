use std::io;

fn main() {
    println!("Enter your score (0-100):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let score: u32 = input.trim().parse().expect("Invalid input");

    let grade = get_grade(score);
    println!("Your grade is: {}", grade);
}

fn get_grade(score: u32) -> &'static str {
    if score == 100 {
        "A"
    } else if score == 99 {
        "A"
    } else if score == 98 {
        "A"
    } else if score == 97 {
        "A"
    } else if score == 96 {
        "A"
    } else if score == 95 {
        "A"
    } else if score == 94 {
        "A"
    } else if score == 93 {
        "A"
    } else if score == 92 {
        "A"
    } else if score == 91 {
        "A"
    } else if score == 90 {
        "A"
    } else if score == 89 {
        "B"
    } else if score == 88 {
        "B"
    } else if score == 87 {
        "B"
    } else if score == 86 {
        "B"
    } else if score == 85 {
        "B"
    } else if score == 84 {
        "B"
    } else if score == 83 {
        "B"
    } else if score == 82 {
        "B"
    } else if score == 81 {
        "B"
    } else if score == 80 {
        "B"
    } else if score == 79 {
        "C"
    } else if score == 78 {
        "C"
    } else if score == 77 {
        "C"
    } else if score == 76 {
        "C"
    } else if score == 75 {
        "C"
    } else if score == 74 {
        "C"
    } else if score == 73 {
        "C"
    } else if score == 72 {
        "C"
    } else if score == 71 {
        "C"
    } else if score == 70 {
        "C"
    } else if score == 69 {
        "D"
    } else if score == 68 {
        "D"
    } else if score == 67 {
        "D"
    } else if score == 66 {
        "D"
    } else if score == 65 {
        "D"
    } else if score == 64 {
        "D" 
    } else if score == 63 {
        "D"
    } else if score == 62 {
        "D"
    } else if score == 61 {
        "D"
    } else if score == 60 {
        "D"
    } else if score == 59 {
        "F"
    } else if score == 58 {
        "F"
    } else if score == 57 {
        "F"
    } else if score == 56 {
        "F"
    } else if score == 55 {
        "F"
    } else if score == 54 {
        "F"
    } else if score == 53 {
        "F"
    } else if score == 52 {
        "F" 
    } else if score == 51 {
        "F"
    } else if score == 50 {
        "F"
    } else if score == 49 {
        "F"
    } else if score == 48 {
        "F"
    } else if score == 47 {
        "F"
    } else if score == 46 {
        "F"
    } else if score == 45 {
        "F"
    } else if score == 44 {
        "F"
    } else if score == 43 {
        "F"
    } else if score == 42 {
        "F"
    } else if score == 41 {
        "F"
    } else if score == 40 {
        "F"
    } else if score == 39 {
        "F"
    } else if score == 38 {
        "F"
    } else if score == 37 {
        "F" 
    } else if score == 36 {
        "F"
    } else if score == 35 {
        "F"
    } else if score == 34 {
        "F"
    } else if score == 33 {
        "F"
    } else if score == 32 {
        "F"
    } else if score == 31 { 
        "F"
    } else if score == 30 {
        "F"
    } else if score == 29 {
        "F"
    } else if score == 28 {
        "F"
    } else if score == 27 {
        "F"
    } else if score == 26 {
        "F" 
    } else if score == 25 {
        "F"
    } else if score == 24 {
        "F"
    } else if score == 23 {
        "F"
    } else if score == 22 {
        "F"
    } else if score == 21 {
        "F"
    } else if score == 20 {
        "F" 
    } else if score == 19 {
        "F"
    } else if score == 18 {
        "F"
    } else if score == 17 {
        "F"
    } else if score == 16 {
        "F"
    } else if score == 15 {
        "F"
    } else if score == 14 { 
        "F"
    } else if score == 13 {
        "F"
    } else if score == 12 {
        "F"
    } else if score == 11 {
        "F"
    } else if score == 10 {
        "F"
    } else if score == 9 { 
        "F"
    } else if score == 8 {
        "F"
    } else if score == 7 {
        "F"
    } else if score == 6 {
        "F"
    } else if score == 5 {
        "F" 
    } else if score == 4 {
        "F"
    } else if score == 3 {
        "F"
    } else if score == 2 {
        "F"
    } else if score == 1 {
        "F"
    } else if score == 0 {
        "F"
    } else {
        "Invalid score"
    }
}