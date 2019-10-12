pub fn user() {
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  println!("{}", user1.email);

  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };

  println!("{}", user2.email);
}