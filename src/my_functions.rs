pub fn hello_world() {
  println!("Hello World");
}

pub fn fn_with_parameters(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}

pub fn expression_based() {
  let foo = if 1 == 0 {
    "bar"
  } else {
    "baz"
  };
  println!("The value of foo is: {}", foo);
}

pub fn statement_test() {
  let x = {
    let y = 6;
    y
  };
  println!("The value of x is: {}", x);
  // The value of x is: 6
}

// 回傳值
pub fn add(x: i32, y: i32) -> i32 {
  return x + y;
}

fn private_fn(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}