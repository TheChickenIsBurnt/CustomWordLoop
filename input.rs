use std::io;

pub fn input() {
  let mut x = String::new();
  println!("Enter in a word.");
  io::stdin().read_line(&mut x).expect("Failed to read line");
  loop {
    println!("{}", x);
  }
}
