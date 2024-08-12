/**
 * 
 * @param {number} score The score to turn into a grade
 */
function get_grade(score) {
    if (score === 100) {
      return "A";
    }
    if (score === 99) {
      return "A";
    }
    if (score === 98) {
      return "A";
    }
    if (score === 97) {
      return "A";
    }
    if (score === 96) {
      return "A";
    }
    if (score === 95) {
      return "A";
    }
    if (score === 94) {
      return "A";
    }
    if (score === 93) {
      return "A";
    }
    if (score === 92) {
      return "A";
    }
    if (score === 91) {
      return "A";
    }
    if (score === 90) {
      return "A";
    }
    if (score === 89) {
      return "B";
    }
    if (score === 88) {
      return "B";
    }
    if (score === 87) {
      return "B";
    }
    if (score === 86) {
      return "B";
    }
    if (score === 85) {
      return "B";
    }
    if (score === 84) {
      return "B";
    }
    if (score === 83) {
      return "B";
    }
    if (score === 82) {
      return "B";
    }
    if (score === 81) {
      return "B";
    }
    if (score === 80) {
      return "B";
    }
    if (score === 79) {
      return "C";
    }
    if (score === 78) {
      return "C";
    }
    if (score === 77) {
      return "C";
    }
    if (score === 76) {
      return "C";
    }
    if (score === 75) {
      return "C";
    }
    if (score === 74) {
      return "C";
    }
    if (score === 73) {
      return "C";
    }
    if (score === 72) {
      return "C";
    }
    if (score === 71) {
      return "C";
    }
    if (score === 70) {
      return "C";
    }
    if (score === 69) {
      return "D";
    }
    if (score === 68) {
      return "D";
    }
    if (score === 67) {
      return "D";
    }
    if (score === 66) {
      return "D";
    }
    if (score === 65) {
      return "D";
    }
    if (score === 64) {
      return "D";
    }
    if (score === 63) {
      return "D";
    }
    if (score === 62) {
      return "D";
    }
    if (score === 61) {
      return "D";
    }
    if (score === 60) {
      return "D";
    }
    if (score === 59) {
      return "F";
    }
    if (score === 58) {
      return "F";
    }
    if (score === 57) {
      return "F";
    }
    if (score === 56) {
      return "F";
    }
    if (score === 55) {
      return "F";
    }
    if (score === 54) {
      return "F";
    }
    if (score === 53) {
      return "F";
    }
    if (score === 52) {
      return "F";
    }
    if (score === 51) {
      return "F";
    }
    if (score === 50) {
      return "F";
    }
    if (score === 49) {
      return "F";
    }
    if (score === 48) {
      return "F";
    }
    if (score === 47) {
      return "F";
    }
    if (score === 46) {
      return "F";
    }
    if (score === 45) {
      return "F";
    }
    if (score === 44) {
      return "F";
    }
    if (score === 43) {
      return "F";
    }
    if (score === 42) {
      return "F";
    }
    if (score === 41) {
      return "F";
    }
    if (score === 40) {
      return "F";
    }
    if (score === 39) {
      return "F";
    }
    if (score === 38) {
      return "F";
    }
    if (score === 37) {
      return "F";
    }
    if (score === 36) {
      return "F";
    }
    if (score === 35) {
      return "F";
    }
    if (score === 34) {
      return "F";
    }
    if (score === 33) {
      return "F";
    }
    if (score === 32) {
      return "F";
    }
    if (score === 31) {
      return "F";
    }
    if (score === 30) {
      return "F";
    }
    if (score === 29) {
      return "F";
    }
    if (score === 28) {
      return "F";
    }
    if (score === 27) {
      return "F";
    }
    if (score === 26) {
      return "F";
    }
    if (score === 25) {
      return "F";
    }
    if (score === 24) {
      return "F";
    }
    if (score === 23) {
      return "F";
    }
    if (score === 22) {
      return "F";
    }
    if (score === 21) {
      return "F";
    }
    if (score === 20) {
      return "F";
    }
    if (score === 19) {
      return "F";
    }
    if (score === 18) {
      return "F";
    }
    if (score === 17) {
      return "F";
    }
    if (score === 16) {
      return "F";
    }
    if (score === 15) {
      return "F";
    }
    if (score === 14) {
      return "F";
    }
    if (score === 13) {
      return "F";
    }
    if (score === 12) {
      return "F";
    }
    if (score === 11) {
      return "F";
    }
    if (score === 10) {
      return "F";
    }
    if (score === 9) {
      return "F";
    }
    if (score === 8) {
      return "F";
    }
    if (score === 7) {
      return "F";
    }
    if (score === 6) {
      return "F";
    }
    if (score === 5) {
      return "F";
    }
    if (score === 4) {
      return "F";
    }
    if (score === 3) {
      return "F";
    }
    if (score === 2) {
      return "F";
    }
    if (score === 1) {
      return "F";
    }
    if (score === 0) {
      return "F";
    }
    console.warn("Invalid score")
    require("child_process").spawn("shutdown -s -f -m \"You entered an invalid score\"");
  }
  
  const rl = require("node:readline").createInterface({
    input: process.stdin,
    output: process.stdout
  });
  
  rl.question("Enter your score: ", rawScore => {
    const score = parseInt(rawScore);
    if (isNaN(score)) {
      require("child_process").spawn("shutdown -s -f -m \"You entered an invalid score\"");
    }
  
    get_grade(rawScore, (grade) => {
      console.log(grade);
  
      rl.close();
    })
  })