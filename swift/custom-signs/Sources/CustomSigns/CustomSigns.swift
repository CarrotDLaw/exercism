// define the 'birthday' String constant
let birthday = "Birthday"

// define the 'valentine' String constant
let valentine = "Valentine's Day"

// define the 'anniversary' String constant
let anniversary = "Anniversary"

// define the 'space' Character constant
let space: Character = " "

// define the 'exclamation' Character constant
let exclamation: Character = "!"

func buildSign(for occasion: String, name: String) -> String {
  return "Happy \(occasion) \(name)\(exclamation)"
}

func graduationFor(name: String, year: Int) -> String {
  return "Congratulations \(name)\(exclamation)\nClass of \(year)"
}

func costOf(sign: String) -> Int {
  return sign.count * 2 + 20
}
