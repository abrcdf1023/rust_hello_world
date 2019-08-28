pub fn fn_with_parameters(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}

// 回傳值
pub fn add(x: i32, y: i32) -> i32 {
  x + y
}

fn private_fn(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}