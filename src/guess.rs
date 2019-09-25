use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess_number() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess.");

    // 宣告 guess 是 String type 的 associated function
    // The :: syntax in the ::new line indicates that new is an associated function of the String type
    // Associated functions are functions associated with a type.
    // To summarize, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
    let mut guess = String::from("test");

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