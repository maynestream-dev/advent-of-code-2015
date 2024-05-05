use std::fs;

fn main() {
  let contents = fs::read_to_string("input.txt")
    .expect("Should have been able to read the file");

  let mut floor = 0;
  let mut entered: bool = false;
  for (i, c) in contents.chars().enumerate() {
    if c == '(' {
      floor += 1;
    } else if c == ')' {
      floor -= 1;
    }
    if floor == -1 && !entered {
      println!("First entered basement on position: {position}", position = i + 1);
      entered = true;
    }
  }

  println!("Finish on floor: {floor}");
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_case_1() {
    let result = 2 + 2;
    assert_eq!(result, 4); 
  }
}
