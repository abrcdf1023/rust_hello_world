use std::cmp::Ordering;

pub fn mul_condition(number: i32) {
  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }
}

pub fn use_match(number: i32) {
  let compare = 3;

  match number.cmp(&compare) {
    Ordering::Less => println!("Less"),
    Ordering::Greater => println!("Greater"),
    Ordering::Equal => println!("Equal"),
  }
}

pub fn use_if_expression() {
  let condition = true;
  let number = if condition {
      5
  } else {
      6
  };

  println!("The value of number is: {}", number);
}

pub fn add_to_100() {
  let mut number = 0;
  loop {
    number += 1;

    if number == 100 {
      println!("Add to 100 is done. result = {}", number);
      break;
    }
  }
}

pub fn loop_result() {
  let mut counter = 0;

  let result = loop {
      counter += 1;

      if counter == 10 {
          break counter * 2;
      }
  };

  println!("The result is {}", result);
}

pub fn reduce_to_0() {
  let mut number = 100;
  while number > 0 {
    number -= 1;
  }
  println!("Reduce to 0 is done. result = {}", number);
}

pub fn for_in_array() {
  let array = [10, 20, 30];
  for element in array.iter() {
    println!("element = {}", element);
  }
}