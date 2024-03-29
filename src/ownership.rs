pub fn move_vs_copy() {
  let x = 5;
  let y = x;
  println!("{}", x);
  let s1 = String::from("hello");
  let s2 = s1;
  // println!("{}", s1);
}

// Rust 沒有 garbage collector 的機制取而代之的是當變數 out of scope 就會釋放記憶體位置(drop function)。
// 但是這樣會導致一些問題
// 1. 當兩個變數指到同一個記憶體位子時會導致 double free error
// 例如這個 function

pub fn double_free_error() {
  let s1 = String::from("hello");
  // We "move" the s1 to s2
  let s2 = s1;
  // give a new s1 to avoid double free error
  let s1 = String::from("free me");

  println!("{}", s1);
  println!("{}", s2);
}

// 由於 rust 預設都是 shallow copy 因此記憶體位子相同的情況下就會發生 double free error。
// 所以要解這個問題可以使用 deep copy 來複製一個全新的記憶體位置，rust 提供了 clone 的方法。
pub fn to_fixed_double_free_error() {
  let s1 = String::from("hello");
  // Deep copy a brand new s1
  let s2 = s1.clone();

  println!("{}", s1);
  println!("{}", s2);
}

// 由於 integers 類型有已知的記憶體大小且存放在 stack 中所以複製起來很快。
// 因此我們不需要 deep clone。
pub fn without_double_free_error() {
  let s1 = 5;
  let s2 = s1;

  println!("{}", s1);
  println!("{}", s2);
}

pub fn ownership_and_functions() {
  let s = String::from("hello"); // s 現在在 scope 裡面

  takes_ownership(s); // 由於 s 是用 heap 法處理所以 move
                      // 因此 s 的所有權已經被轉移了

  // println!("s = {}", s); // 這時已經取得不了 s 編譯會報錯
                     // ^ value borrowed here after move

  let x = 5; // 反之如果是 stack 法處理

  makes_copy(x); // x 就會 copy 一份後進到函數裡面
                 // 所以還是可以印得出 x 不會報錯
  println!("x = {}", x);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
} 

pub fn ownership_and_return_values() {
  let s1 = gives_ownership(); // gives_ownership moves its return
                              // value into s1

  let s2 = String::from("hello"); // s2 comes into scope

  let s3 = takes_and_gives_back(s2); // s2 is moved into
                                     // takes_and_gives_back, which also
                                     // moves its return value into s3
                                     
  // Error because s2 ownership has moved into takes_and_gives_back.
  // println!("{}", s2);
  // Not Error s3 has the ownership
  println!("{}", s3);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
  // gives_ownership will move its
  // return value into the function
  // that calls it

  let some_string = String::from("hello"); // some_string comes into scope

  some_string // some_string is returned and
              // moves out to the calling
              // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
  // a_string comes into
  // scope

  a_string // a_string is returned and moves out to the calling function
}

// 如果我們還需要使用 s 時必須再把他回傳
fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length)
}

// 結論，如果我們還必須使用原本的 value 時該怎麼辦？因為 ownership 已經被傳到 function 裡面了。
// 這個問題的解法是 references

