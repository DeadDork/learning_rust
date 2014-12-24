// Explores whether its possible to export a nested function.

// Conclusion: it doesn't appear that you can.

// PS I've only spent a few days exploring module systems, but it doesn't
// seem that Rust's is terribly interesting. Aside from attributes that give
// some fine-grained control, I don't see anything here that differentiates
// Rust from Haskell's simple & conservative module system.

fn main() {
  hello::print_hello();
}

// Doesn't work
mod hello {
  fn dummy() {
    pub fn print_hello() {
      println!("Hello, world!");
    }
  }
}

// Does work
mod hello {
  pub fn print_hello() {
    println!("Hello, world!");
  }
}
