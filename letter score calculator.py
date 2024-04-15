def get_grade(score):
  if score == 100:
    return "A"
  elif score == 99:
    return "A"
  elif score == 98:
    return "A"
  elif score == 97:
    return "A"
  elif score == 96:
    return "A"
  elif score == 95:
    return "A"
  elif score == 94:
    return "A"
  elif score == 93:
    return "A"
  elif score == 92:
    return "A"
  elif score == 91:
    return "A"
  elif score == 90:
    return "A"
  elif score == 89:
    return "B"
  elif score == 88:
    return "B"
  elif score == 87:
    return "B"
  elif score == 86:
    return "B"
  elif score == 85:
    return "B"
  elif score == 84:
    return "B"
  elif score == 83:
    return "B"
  elif score == 82:
    return "B"
  elif score == 81:
    return "B"
  elif score == 80:
    return "B"
  elif score == 79:
    return "C"
  elif score == 78:
    return "C"
  elif score == 77:
    return "C"
  elif score == 76:
    return "C"
  elif score == 75:
    return "C"
  elif score == 74:
    return "C"
  elif score == 73:
    return "C"
  elif score == 72:
    return "C"
  elif score == 71:
    return "C"
  elif score == 70:
    return "C"
  elif score == 69:
    return "D"
  elif score == 68:
    return "D"
  elif score == 67:
    return "D"
  elif score == 66:
    return "D" 
  elif score == 65:
    return "D"
  elif score == 64:
    return "D"
  elif score == 63:
    return "D"
  elif score == 62:
    return "D" 
  elif score == 61:
    return "D"
  elif score == 60:
    return "D"
  elif score == 59:
    return "F"
  elif score == 58:
    return "F"
  elif score == 57:
    return "F"
  elif score == 56:
    return "F"
  elif score == 55:
    return "F"
  elif score == 54:
    return "F"
  elif score == 53:
    return "F"
  elif score == 52:
    return "F"
  elif score == 51:
    return "F"
  elif score == 50:
    return "F"
  elif score == 49:
    return "F"
  elif score == 48:
    return "F"
  elif score == 47:
    return "F"
  elif score == 46:
    return "F"
  elif score == 45:
    return "F"
  elif score == 44:
    return "F"
  elif score == 43:
    return "F"
  elif score == 42:
    return "F"
  elif score == 41:
    return "F"
  elif score == 40:
    return "F"
  elif score == 39:
    return "F"
  elif score == 38:
    return "F"
  elif score == 37:
    return "F"
  elif score == 36:
    return "F"
  elif score == 35:
    return "F"
  elif score == 34:
    return "F"
  elif score == 33:
    return "F"
  elif score == 32:
    return "F"
  elif score == 31:
    return "F"
  elif score == 30:
    return "F"
  elif score == 29:
    return "F"
  elif score == 28:
    return "F"
  elif score == 27:
    return "F"
  elif score == 26:
    return "F"
  elif score == 25:
    return "F"
  elif score == 24:
    return "F"
  elif score == 23:
    return "F"
  elif score == 22:
    return "F"
  elif score == 21:
    return "F"
  elif score == 20:
    return "F"
  elif score == 19:
    return "F"
  elif score == 18:
    return "F"
  elif score == 17:
    return "F"
  elif score == 16:
    return "F"
  elif score == 15:
    return "F"
  elif score == 14:
    return "F"
  elif score == 13:
    return "F"
  elif score == 12:
    return "F"
  elif score == 11:
    return "F"
  elif score == 10:
    return "F"
  elif score == 9:
    return "F"
  elif score == 8:
    return "F"
  elif score == 7:
    return "F"
  elif score == 6:
    return "F"
  elif score == 5:
    return "F"
  elif score == 4:
    return "F"
  elif score == 3:
    return "F"
  elif score == 2:
    return "F"
  elif score == 1:
    return "F"
  elif score == 0:
    return "F"
  else:
    return "Invalid score"

# Get user input
score = int(input("Enter your score (0-100): "))

# Get the letter grade
grade = get_grade(score)

# Print the result
print("Your grade is:", grade)