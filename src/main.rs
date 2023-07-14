extern crate core;
extern crate libloading;
use core::ffi::c_int;
use std::i32;
use libloading::{Library, Symbol};

type AddFunc = extern "C" fn ( a: c_int, b: c_int) -> c_int; 

fn call_dynamic() -> Result<i32, Box<dyn std::error::Error>> {
    unsafe { // /Users/philip/dev/rust-practice/c-function/bridge-dinamic/bridge-c/lib.dylib
        let library_path = "/Users/philip/dev/rust-practice/c-function/bridge-dinamic/bridge-c/lib.dylib";
        let lib = Library::new(library_path).unwrap();
        let func: Symbol<AddFunc> = lib.get(b"add").unwrap();

        let answer = func(1, 2);
        println!("1 + 2 = {}", answer);
        Ok(answer)
    }
}


//extern "C"{
//    pub fn covert_and_sum( a: c_int, b: c_int) -> c_int;
//}


fn main() {
    let result = call_dynamic();

    match  result {
        Ok(v) => println!("Extracted from result {:?}", v),
        Err(e) => println!("Bitter disappointment {:?}", e),
  }

}