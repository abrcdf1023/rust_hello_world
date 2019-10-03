use std::io;
mod guess;
mod datatype;
mod my_functions;
mod control_flow;
mod ownership;
mod references;
mod slice_type;

fn main() {
  // guess::guess_number();
  // datatype::mutable();
  // datatype::variables();
  // datatype::variables_shadowing();
  // datatype::wrong_types();
  // datatype::data_types();
  // datatype::data_types_integer();
  // datatype::data_types_float();
  // datatype::numeric_operations();
  // datatype::data_types_boolean();
  // datatype::data_types_char();
  // datatype::compound_types();
  // datatype::tuple_with_mixed_data_type();
  // my_functions::hello_world();
  // my_functions::fn_with_parameters(5, 5);
  // my_functions::expression_based();
  // my_functions::statement_test();
  // ERROR: 這會報錯因為他是 private 的 function
  // my_functions::private_fn(5, 5);
  // println!("5 + 5 = {}", my_functions::add(5, 5));
  // control_flow::mul_condition(6);
  // control_flow::use_match(6);
  // control_flow::loop_result();
  // control_flow::use_if_expression();
  // control_flow::add_to_100();
  // control_flow::reduce_to_0();
  // control_flow::for_in_array();
  // ownership::move_vs_copy();
  // ownership::double_free_error();
  // ownership::to_fixed_double_free_error();
  // ownership::without_double_free_error();
  // ownership::ownership_and_functions();
  // ownership::ownership_and_return_values();
  // let s = String::from("hello");
  // references::unchangeable(&s);
  // let mut s = String::from("hello");
  // references::changeable(&mut s);
  // println!("s = {}", s);
  // references::cause_error();
  // references::cause_error_too();
  // references::this_work();
  // references::this_work_too();
  // let reference_to_nothing = references::dangling_references();
  // let reference_to_nothing = references::no_dangling_references();
  
  // let mut text = String::from("hello world");
  // println!("first word end index = {}", slice_type::first_word(&text));

  // let mut s = String::from("hello world");
  // 把 s 借給了 word
  // let word = slice_type::first_word_enhance(&s);
  // 把 s 清掉
  // s.clear();
  // 因為 s 已經被清掉了所以就不可能 reference 的到他因此會報錯。
  // println!("the first word is: {}", word);
}