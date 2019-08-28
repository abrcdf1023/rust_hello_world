use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  // guessNumber()
  // variables()
  // variables_shadowing()
  // data_types()
  // data_types_integer()
  // data_types_float()
  // numeric_operations()
  // data_types_boolean()
  // data_types_char()
  compound_types()
}

fn guess_number() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess.");

    // 宣告 guess 是 String type 的 associated function
    // The :: syntax in the ::new line indicates that new is an associated function of the String type
    // Associated functions are functions associated with a type.
    // To summarize, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
    let mut guess = String::new();

    // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. 
    // 使用指標減少記憶體耗損
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // 轉換型態
    // shadowing 讓我們可以不必重新命名一個變數名稱
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      // 錯誤時的處理
      Err(_) => {
        println!("Please type a number!");
        continue;
      },
    };

    println!("You guessed: {}", guess);

    // 比對結果是否符合
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}

// In Rust, variables are immutable by default.
fn variables() {
  // let x = 5;
  // println!("The value of x is: {}", x);
  // x = 6;
  // println!("The value of x is: {}", x);

  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}

// Variable Shadowing
// Shadowing & mutable 不同之處在於
// 1. Shadowing 是重新給一個新的值在同樣的名稱上但他仍然是 immutable 的
// 2. Shadowing 可以改變型態 mutable 則不行
fn variables_shadowing() {
  let x = 5;

  let x = x + 1;

  let x = x * 2;

  println!("The value of x is: {}", x);
}

fn data_types() {
  // Rust 是強型別的語言也無需特別宣告
  let mut y = 30;
  println!("The value of y is: {}", y);
  // Error: 這裡會報錯
  // y = "40";
  println!("The value of y is: {}", y);
}

fn data_types_integer() {
  // 整數型態
  // integer types default to i32: this type is generally the fastest
  let mut x: i32 = "42".parse().expect("Not a number!");
  println!("The value of x is: {}", x);
  // Signed and unsigned refer to whether it’s possible for the number to be negative or positive
  // i 包含正數以及負數, i8 的範圍為從 負2的7次方 到 正2的7次方 (-128 to 127)
  // u 只包含正數eg, u8 的範圍為從 0 到 2的8次方 - 1 (0 to 255)
  x = x * 30;
  println!("The value of x is: {}", x);
}

fn data_types_float() {
  // 浮點數型態
  // Rust’s floating-point types are f32 and f64
  // 64 位元跟 32 位元
  let x = 2.5; // f64
  println!("The value of x is: {}", x);
  let y: f32 = 3.3; // f32
  println!("The value of y is: {}", y);
}

fn numeric_operations() {
  // 同樣的型態才可以做運算
  // addition
  let sum = 5 + 10;
  println!("sum = {}", sum);
  // subtraction
  let difference = 95.5 - 4.0;
  println!("difference = {}", difference);
  // multiplication
  let product = 4 * 30;
  println!("product = {}", product);
  // division
  let quotient = 56.7 / 32.2;
  println!("quotient = {}", quotient);
  // remainder
  let remainder = 43 % 5;
  println!("remainder = {}", remainder);
}

fn data_types_boolean() {
  // 布林型態
  let t = true;
  println!("t = {}", t);
  let f: bool = false; // with explicit type annotation
  println!("f = {}", f);
}

fn data_types_char() {
  // 字串型態
  let c = 'z';
  println!("c = {}", c);
  let z = 'ℤ';
  println!("z = {}", z);
  let heart_eyed_cat = '😻';
  println!("heart_eyed_cat = {}", heart_eyed_cat);
}

fn compound_types() {
  // The Tuple Type
  // 組合不同的型態成為一個單元
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of y is: {}", y);
  println!("The value of tup.0 is: {}", tup.0);
  println!("The value of tup.1 is: {}", tup.1);
  println!("The value of tup.2 is: {}", tup.2);

  // Array
  // 陣列裡的物件必須都為同樣的類型
  let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let a = [3; 5]; // equals let a = [3, 3, 3, 3, 3];
  // 取得陣列物件
  let j = months[0];
  println!("The value of j is: {}", j);
  // 取物件時 index 不能大於或等於陣列長度否則編譯會錯
  // Error: 這裡會報錯
  // let element = a[10];
}