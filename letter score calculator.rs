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
    }  if score == 99 {
        "A"
    }  if score == 98 {
        "A"
    }  if score == 97 {
        "A"
    }  if score == 96 {
        "A"
    }  if score == 95 {
        "A"
    }  if score == 94 {
        "A"
    }  if score == 93 {
        "A"
    }  if score == 92 {
        "A"
    }  if score == 91 {
        "A"
    }  if score == 90 {
        "A"
    }  if score == 89 {
        "B"
    }  if score == 88 {
        "B"
    }  if score == 87 {
        "B"
    }  if score == 86 {
        "B"
    }  if score == 85 {
        "B"
    }  if score == 84 {
        "B"
    }  if score == 83 {
        "B"
    }  if score == 82 {
        "B"
    }  if score == 81 {
        "B"
    }  if score == 80 {
        "B"
    }  if score == 79 {
        "C"
    }  if score == 78 {
        "C"
    }  if score == 77 {
        "C"
    }  if score == 76 {
        "C"
    }  if score == 75 {
        "C"
    }  if score == 74 {
        "C"
    }  if score == 73 {
        "C"
    }  if score == 72 {
        "C"
    }  if score == 71 {
        "C"
    }  if score == 70 {
        "C"
    }  if score == 69 {
        "D"
    }  if score == 68 {
        "D"
    }  if score == 67 {
        "D"
    }  if score == 66 {
        "D"
    }  if score == 65 {
        "D"
    }  if score == 64 {
        "D"
    }  if score == 63 {
        "D"
    }  if score == 62 {
        "D"
    }  if score == 61 {
        "D"
    }  if score == 60 {
        "D"
    }  if score == 59 {
        "F"
    }  if score == 58 {
        "F"
    }  if score == 57 {
        "F"
    }  if score == 56 {
        "F"
    }  if score == 55 {
        "F"
    }  if score == 54 {
        "F"
    }  if score == 53 {
        "F"
    }  if score == 52 {
        "F"
    }  if score == 51 {
        "F"
    }  if score == 50 {
        "F"
    }  if score == 49 {
        "F"
    }  if score == 48 {
        "F"
    }  if score == 47 {
        "F"
    }  if score == 46 {
        "F"
    }  if score == 45 {
        "F"
    }  if score == 44 {
        "F"
    }  if score == 43 {
        "F"
    }  if score == 42 {
        "F"
    }  if score == 41 {
        "F"
    }  if score == 40 {
        "F"
    }  if score == 39 {
        "F"
    }  if score == 38 {
        "F"
    }  if score == 37 {
        "F"
    }  if score == 36 {
        "F"
    }  if score == 35 {
        "F"
    }  if score == 34 {
        "F"
    }  if score == 33 {
        "F"
    }  if score == 32 {
        "F"
    }  if score == 31 {
        "F"
    }  if score == 30 {
        "F"
    }  if score == 29 {
        "F"
    }  if score == 28 {
        "F"
    }  if score == 27 {
        "F"
    }  if score == 26 {
        "F"
    }  if score == 25 {
        "F"
    }  if score == 24 {
        "F"
    }  if score == 23 {
        "F"
    }  if score == 22 {
        "F"
    }  if score == 21 {
        "F"
    }  if score == 20 {
        "F"
    }  if score == 19 {
        "F"
    }  if score == 18 {
        "F"
    }  if score == 17 {
        "F"
    }  if score == 16 {
        "F"
    }  if score == 15 {
        "F"
    }  if score == 14 {
        "F"
    }  if score == 13 {
        "F"
    }  if score == 12 {
        "F"
    }  if score == 11 {
        "F"
    }  if score == 10 {
        "F"
    }  if score == 9 {
        "F"
    }  if score == 8 {
        "F"
    }  if score == 7 {
        "F"
    }  if score == 6 {
        "F"
    }  if score == 5 {
        "F"
    }  if score == 4 {
        "F"
    }  if score == 3 {
        "F"
    }  if score == 2 {
        "F"
    }  if score == 1 {
        "F"
    }  if score == 0 {
        "F"
    } else {
        "Invalid score"
    }
}
