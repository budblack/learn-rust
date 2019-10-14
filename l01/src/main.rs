extern "C" {
  pub fn db(a :i32)->i32;
  pub fn sayHello();
}
fn main() {
  unsafe{
    println!("Hello, 3*2={}", db(3));
    sayHello();
  }
}
