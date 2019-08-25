use std::io;

fn main() {
  println!("Guess the number!");

  println!("Please input your guess.");

  // 宣告 guess 是 String 的類別
  // The :: syntax in the ::new line indicates that new is an associated function of the String type
  // Associated functions are functions associated with a type.
  // To summarize, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
  let mut guess = String::new();

  // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. 
  // 指標
  io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

  println!("You guessed: {}", guess);
}

// In Rust, variables are immutable by default.
fn variables() {
  let foo = 5; // immutable
  let mut bar = 5; // mutable
}