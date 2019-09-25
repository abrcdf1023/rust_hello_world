// rust 可以使用 reference 指向原本的 value 並且不轉移 ownership

// 例如
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// 這樣便可以解決 ownership 一直被傳來傳去的問題

// 同時如果我們打算修改這個 reference 會發生什麼事情？
// pub fn change(some_string: &String) {
//   some_string.push_str(", world");
// }
// 由於 rust 預設是 immutable 的資料型態所以 reference 也是 immutable 的因此會報錯
// 除非我們把 reference 改成使用 mutable
pub fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

// 但是 mutable reference 有個很大的限制是一個 scope 裡面只能有一個，例如下列程式碼將會報錯
// fn error_mut_reference() {
//   let mut s = String::from("hello");

//   let r1 = &mut s;
//   let r2 = &mut s;

//   println!("{}, {}", r1, r2);
// }
// 道理很簡單你不該有兩個指標可以同時修改一個記憶體位置！這有可能導致 race condition

// 下列三種情況會導致這個問題
// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.

