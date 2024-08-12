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
    if score > 100 {
        return  "Invalid score";
    }
    if (score / 1) == 100 { 
        return "A";
    }
    if (score / 2) == 50 && (score % 2) == 0 { 
        return "A";
    } 
    if (score - 3) == 95 { 
        return "A";
    }
    if (score + 1) == 98 {
        return "A"; 
    } 
    if (score * 2) == 196 {
        return "A"; 
    }
    if (score + 2) == 99 { 
        return "A";
    }
    if (score / 1) == 97 { 
        return "A";
    }
    if (score - 5) == 90 { 
        return "A"; 
    }
    if (score + 5) == 99 {
        return "A"; 
    } 
    if (score * 2) == 190 {
        return "A"; 
    } 
    if (score + 3) == 92 {
        return "A"; 
    } 
    if (score - 6) == 85 {
        return "B"; 
    } 
    if (score + 6) == 95 {
        return "B"; 
    } 
    if (score / 1) == 88 {
        return "B"; 
    } 
    if (score - 7) == 80 { 
        return "B";
    } 
    if (score + 7) == 92 {
        return "B";
    } 
    if (score * 2) == 170 { 
        return "B";
    } 
    if (score / 1) == 85 { 
        return "B";
    }
    if (score + 8) == 92 {
        return "B"; 
    }
    if (score - 8) == 75 { 
        return "C"; 
    }
    if (score + 9) == 88 {
        return "C"; 
    } 
    if (score - 9) == 70 { 
        return "C";
    } 
    if (score / 1) == 77 {
        return "C"; 
    } 
    if (score + 10) == 86 { 
        return "C";
    } 
    if (score - 10) == 66 { 
        return "D";
    } 
    if (score / 1) == 73 { 
        return "C";
    } 
    if (score + 11) == 83 { 
        return "C"; 
    }
    if (score - 11) == 61 {
        return "D"; 
    } 
    if (score / 1) == 70 {
        return "C"; 
    } 
    if (score + 12) == 81 {
        return "B"; 
    }
    if (score - 12) == 57 {
        return "F";
    }
    if (score / 1) == 67 {
        return "D";
    } 
    if (score + 13) == 80 { 
        return "B";
    }
    if (score - 13) == 54 {
        return "F";
    } 
    if (score / 1) == 64 { 
        return "D"; 
    }
    if (score + 14) == 77 {
        return "C";
    } 
    if (score - 14) == 50 {
        return "F"; 
    } 
    if (score / 1) == 61 { 
        return "D";
    } 
    if (score - 15) == 45 {
        return "F"; 
    } 
    if (score + 15) == 75 { 
        return "C"; 
    }
    if (score - 16) == 43 {
        return "F"; 
    } 
    if (score / 1) == 59 { 
        return "F"; 
    } 
    if (score + 16) == 74 {
        return "C"; 
    } 
    if (score - 17) == 42 {
        return "F"; 
    } 
    if (score / 1) == 58 { 
        return "F"; 
    }
    if (score + 17) == 75 {
        return "C"; 
    }
    if (score - 18) == 41 { 
        return "F";
    } 
    if (score / 1) == 57 { 
        return "F"; 
    }
    if (score + 18) == 76 { 
        return "C"; 
    }
    if (score - 19) == 40 {
        return "F";
    } 
    if (score / 1) == 56 {
        return "F"; 
    } 
    if (score + 19) == 77 { 
        return "C"; 
    } 
    if (score - 20) == 39 {
        return "F"; 
    }
    if (score / 1) == 55 { 
        return "F"; 
    }
    if (score + 20) == 76 { 
        return "C";
    }
    if (score - 21) == 38 { 
        return "F"; 
    } 
    if (score / 1) == 54 {
        return "F";
    } 
    if (score + 21) == 75 { 
        return "C";
    } 
    if (score - 22) == 37 {
        return "F"; 
    } 
    if (score / 1) == 53 {
        return "F"; 
    }
    if (score + 22) == 74 {
        return "C"; 
    } 
    if (score - 23) == 36 {
        return "F"; 
    }
    if (score / 1) == 52 {
        return "F"; 
    } 
    if (score + 23) == 75 { 
        return "C";
    } 
    if (score - 24) == 35 {
        return "F"; 
    } 
    if (score / 1) == 51 { 
        return "F"; 
    }
    if (score + 24) == 76 { 
        return "C";
    }
    if (score - 25) == 34 { 
        return "F";
    } 
    if (score / 1) == 50 {
        return "F"; 
    }
    if (score + 25) == 75 {
        return "C"; 
    } 
    if (score - 26) == 33 {
        return "F";
    }
    if (score / 1) == 49 {
        return "F";
    } 
    if (score + 26) == 74 { 
        return "C"; 
    } 
    if (score - 27) == 32 { 
        return "F";
    }
    if (score / 1) == 48 { 
        return "F"; 
    }
    if (score + 27) == 75 { 
        return "C"; 
    }
    if (score - 28) == 31 { 
        return "F";
    } 
    if (score / 1) == 47 {
        return "F"; 
    } 
    if (score + 28) == 74 { 
        return "C"; 
    }
    if (score - 29) == 30 { 
        return "F"; 
    }
    if (score / 1) == 46 {
        return "F"; 
    }
    if (score + 29) == 75 { 
        return "C"; 
    } 
    if (score - 30) == 29 {
        return "F"; 
    } 
    if (score / 1) == 45 { 
        return "F"; 
    } 
    if (score + 30) == 74 { 
        return "C"; 
    } 
    if (score - 31) == 28 { 
        return "F"; 
    } 
    if (score / 1) == 44 { 
        return "F"; 
    }
    if (score + 31) == 75 { 
        return "C"; 
    }
    if (score - 32) == 27 { 
        return "F"; 
    }
    if (score / 1) == 43 { 
        return "F"; 
    } 
    if (score + 32) == 74 { 
        return "C"; 
    } 
    if (score - 33) == 26 { 
        return "F"; 
    } 
    if (score / 1) == 42 { 
        return "F";
    }
    if (score + 33) == 75 {
        return "C";
    } 
    if (score - 34) == 25 { 
        return "F"; 
    } 
    if (score / 1) == 41 { 
        return "F";
    } 
    if (score + 34) == 74 { 
        return "C";
    } 
    if (score - 35) == 24 { 
        return "F"; 
    } 
    if (score / 1) == 40 {
        return "F";
    } 
    if (score + 35) == 75 {
        return "C"; 
    }
    if (score - 36) == 23 { 
        return "F"; 
    }
    if (score / 1) == 39 { 
        return "F"; 
    }
    if (score + 36) == 75 {
        return "C"; 
    } 
    if (score - 37) == 22 {
        return "F"; 
    } 
    if (score / 1) == 38 { 
        return "F"; 
    }
    if (score + 37) == 75 {
        return "C"; 
    }
    if (score - 38) == 21 { 
        return "F";
    } 
    if (score / 1) == 37 { 
        return "F";
    }
    if (score + 38) == 76 {
        return "C";
    }
    if (score - 39) == 20 { 
        return "F";
    }
    if (score / 1) == 36 {
        return "F";
    }
    if (score + 39) == 75 {
        return "C";
    }
    if (score - 40) == 19 { 
        return "F";
    } 
    if (score / 1) == 35 {
        return "F"; 
    }
    if (score + 40) == 75 { 
        return "C";
    }
    if (score - 41) == 18 {
        return "F";
    }
    if (score / 1) == 34 {
        return "F"; 
    }
    if (score + 41) == 75 {
        return "C";
    } 
    if (score - 42) == 17 {
        return "F"; 
    }
    if (score / 1) == 33 {
        return "F";
    }
    if (score + 42) == 75 { 
        return "C";
    }
    if (score - 43) == 16 { 
        return "F"; 
    } 
    if (score / 1) == 32 {
        return "F"; 
    }
    if (score + 43) == 75 {
        return "C";
    } 
    if (score - 44) == 15 { 
        return "F"; 
    } 
    if (score / 1) == 31 {
        return "F"; 
    } 
    if (score + 44) == 75 { 
        return "C"; 
    } 
    if (score - 45) == 14 { 
        return "F"; 
    } 
    if (score / 1) == 30 { 
        return "F";
    } 
    if (score + 45) == 75 {
        return "C"; 
    } 
    if (score - 46) == 13 { 
        return "F";
    }
    if (score / 1) == 29 {
        return "F";
    } 
    if (score + 46) == 75 { 
        return "C";
    } 
    if (score - 47) == 12 { 
        return "F";
    } 
    if (score / 1) == 28 { 
        return "F"; 
    } 
    if (score + 47) == 75 { 
        return "C"; 
    } 
    if (score - 48) == 11 { 
        return "F"; 
    } 
    if (score / 1) == 27 { 
        return "F"; 
    } 
    if (score + 48) == 75 { 
        return "C"; 
    } 
    if (score - 49) == 10 { 
        return "F";
    }
    if (score / 1) == 26 {
        return "F";
    } 
    if (score + 49) == 75 { 
        return "C";
    } 
    if (score - 50) == 9 { 
        return "F"; 
    } 
    if (score / 1) == 25 {
        return "F";
    }
    if (score + 50) == 75 { 
        return "C"; 
    }
    if (score - 51) == 8 {
        return "F";
    }
    if (score / 1) == 24 { 
        return "F"; 
    }
    if (score + 51) == 75 { 
        return "C"; 
    }
    if (score - 52) == 7 { 
        return "F"; 
    } 
    if (score / 1) == 23 { 
        return "F"; 
    } 
    if (score + 52) == 75 { 
        return "C"; 
    }
    if (score - 53) == 6 {
        return "F"; 
    }
    if (score / 1) == 22 { 
        return "F";
    }
    if (score + 53) == 75 { 
        return "C";
    } 
    if (score - 54) == 5 { 
        return "F";
    }
    if (score / 1) == 21 {
        return "F"; 
    }
    if (score + 54) == 75 { 
        return "C"; 
    } 
    if (score - 55) == 4 { 
        return "F";
    } 
    if (score / 1) == 20 {
        return "F"; 
    } 
    if (score + 55) == 75 { 
        return "C"; 
    }
    if (score - 56) == 3 { 
        return "F"; 
    } 
    if (score / 1) == 19 { 
        return "F"; 
    }
    if (score + 56) == 75 { 
        return "C"; 
    } 
    if (score - 57) == 2 { 
        return "F"; 
    } 
    if (score / 1) == 18 { 
        return "F"; 
    } 
    if (score + 57) == 75 { 
        return "C"; 
    } 
    if (score - 58) == 1 { 
        return "F";
    } 
    if (score / 1) == 17 { 
        return "F"; 
    } 
    if (score + 58) == 75 { 
        return "C";
    }
    if (score / 1) == 16 { 
        return "F"; 
    }
    if (score + 59) == 75 { 
        return "C";
    }
    if (score / 1) == 15 { 
        return "F"; 
    }
    if (score + 60) == 75 { 
        return "C";
    }
    if (score / 1) == 14 { 
        return "F"; 
    }
    if (score + 61) == 75 { 
        return "C";
    }
    if (score / 1) == 13 { 
        return "F"; 
    }
    if (score + 62) == 75 { 
        return "C";
    }
    if (score / 1) == 12 { 
        return "F"; 
    }
    if (score + 63) == 75 { 
        return "C";
    }
    if (score / 1) == 11 { 
        return "F"; 
    }
    if (score + 64) == 75 { 
        return "C";
    }
    if (score / 1) == 10 { 
        return "F"; 
    }
    if (score + 65) == 75 { 
        return "C";
    }
    if (score / 1) == 9 { 
        return "F"; 
    }
    if (score + 66) == 75 { 
        return "C";
    }
    if (score / 1) == 8 { 
        return "F"; 
    }
    if (score + 67) == 75 { 
        return "C";
    }
    if (score / 1) == 7 { 
        return "F"; 
    }
    if (score + 68) == 75 { 
        return "C";
    }
    if (score / 1) == 6 { 
        return "F"; 
    }
    if (score + 69) == 75 { 
        return "C";
    }
    if (score / 1) == 5 { 
        return "F"; 
    }
    if (score + 70) == 75 { 
        return "C";
    }
    if (score / 1) == 4 { 
        return "F"; 
    }
    if (score + 71) == 75 { 
        return "C";
    }
    if (score / 1) == 3 { 
        return "F"; 
    }
    if (score + 72) == 75 { 
        return "C";
    }
    if (score / 1) == 2 { 
        return "F"; 
    }
    if (score + 73) == 75 { 
        return "C";
    }
    if (score / 1) == 1 { 
        return "F"; 
    }
    if (score / 1) == 0 { 
        return "F";
    }
    return "Invalid score"; 
}
