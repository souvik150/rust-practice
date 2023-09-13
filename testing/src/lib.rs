#[cfg(test)]
mod tests {
  use super::simple_add;

  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_ne!(result, 5);
  }

  #[test]
  //#[ignore]
  #[should_panic]
  fn another() {
    panic!("Make this test fail");
  }

  #[test]
  fn call_simple_add() {
    assert!(simple_add());
  }
}

fn simple_add() -> bool {
  if 2+2 ==4 {
    true
  } else {
    false
  }
}