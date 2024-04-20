pub struct Scanner<> {
  pub a: i8,
}

pub struct Scanner<Local> {}

pub struct Scanner<Remote> {}

impl Scanner {
  fn new() -> Self {
    Self { a: 0 }
  }
}

impl<Local> Scanner {
  fn new() -> Self {
    Self { a: 0 }
  }
}
