// Tests whether you can have a type signature a la Haskell (I hate this
// immanent crap).

// Conclusion: nope, you have to embed the type signature.

// Doesn't work
//fn main() -> ();
//fn main() {
//  println!("Hello, world!");
//}

// Works
fn main() -> () {
  println!("Hello, world!");
}
