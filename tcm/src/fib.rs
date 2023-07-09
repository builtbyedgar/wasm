// wasm-bindgen is Rust library that facilitate high-level interactions between Wasm and JavaScript
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
  if n == 0 {
      return 0;
  } else if n == 1 {
      return 1;
  }

  let mut fib = (0, 1);
  for _ in 2..=n {
      fib = (fib.1, fib.0 + fib.1);
  }

  fib.1
}


#[wasm_bindgen]
pub fn fibo(n: u32) -> u64 {
  let mut prev: u64 = 0;
  let mut curr: u64 = 1;
  for _ in 1..n {
      let next = prev + curr;
      prev = curr;
      curr = next;
  }
  curr
}
