func protectSecret(_ secret: String, withPassword password: String) -> (String) -> String {
  func validate(_ pw: String) -> String {
    pw == password ? secret : "Sorry. No hidden secrets here."
  }

  return validate
}

func generateCombination(forRoom room: Int, usingFunction f: (Int) -> Int) -> (Int, Int, Int) {
  return (f(room), f(f(room)), f(f(f(room))))
}
