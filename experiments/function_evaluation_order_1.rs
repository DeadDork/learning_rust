// Tests whether it's possible to call a function before it is declared,
// but without having to declare a function prototype.

// Conclusion: you can! This is very cool!

fn main() {
  hello_print();
}

fn hello_print() {
  println!("Hello, world!");
}
