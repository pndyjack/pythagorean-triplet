pub fn find() -> Option<u32> {
  for a in 1..1000 {
    for b in 1..(1000 - a) {
      let mut c = 1000 - a - b;
      if ((a * a) + (b * b)) == (c * c) {
        if a + b + c == 1000 {
          return Some(a * b * c);
        }
      }
    }
  }
  None
}
