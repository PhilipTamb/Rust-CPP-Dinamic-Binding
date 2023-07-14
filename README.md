# Rust-CPP-Dinamic-Binding
Dinamic bindig to call C++ library into Rust for using C++ function from Rust code. Use Libloading for load the C++ library


 Cargo.toml
 ```
[dependencies]
libloading = "0.8"

[build-dependencies]
cc = "1.0"
```

build.rs
```
extern crate cc;
fn main() {
    cc::Build::new().cpp(true).file("src/num.cpp").compile("libnum.dylib");
}
```

main.rs
```
code in main.rs to call dinamically the library
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
```
