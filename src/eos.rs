extern "C" {
  fn printi(c: i64);
  fn printui(c: u64);
  fn prints_l(bytes: *const u8, len: i32);
}

pub fn print_i64(i: i64) {
  unsafe {
    printi(i);
  }
}

pub fn print_u64(u: u64) {
  unsafe {
    printui(u);
  }
}

pub fn print_str(s: &str) {
  unsafe {
    let bytes = s.as_bytes();
    prints_l(bytes.as_ptr(), bytes.len() as i32);
  }
}
