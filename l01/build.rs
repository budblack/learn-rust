extern crate cc;
use std::env;
// use std::process::{Command, Stdio};

fn build_linux_unkonw() {
  cc::Build::new()
    .cpp(true)
    .flag("-std=c++11")
    .warnings(false)
    .include("./lib/hello/linux")
    .file("./src/cpp/hello/double.cc")
    .compile("l01");
  // -------------
  println!("cargo:rustc-link-search=native=./lib/hello/linux");
  println!("cargo:rustc-link-lib=hello");
  // -------------
}

fn main() {
  match env::var("TARGET") {
    Ok(val) => {
      match val.as_str() {
        "x86_64-unknown-linux-gnu" => build_linux_unkonw(),
        &_ => {}
      }
    }
    _ => {
     
    }
  }
}