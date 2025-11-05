fn main() {
  //  println!("print");
  let s=String::from("hello");
  // this string can be mutated :
  let mut s1=String::from("hello");
  s1.push_str(",world");
  println!("{s1}");
}
