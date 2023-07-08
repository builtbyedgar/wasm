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


// Create a static mutable byte buffer.
// NOTE: global `static mut` means we will have "unsafe" code
// but for passing memory between js and wasm should be fine.
const WASM_MEMORY_BUFFER_SIZE: usize = 2;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

// Function to store the passed value at index 0,
// in our buffer
#[wasm_bindgen]
pub fn store_value_in_wasm_memory_buffer_index_zero(value: u8) {
  unsafe {
    WASM_MEMORY_BUFFER[0] = value;
  }
}

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_wasm_memory_buffer_pointer() -> *const u8 {
  let pointer: *const u8;
  unsafe {
    pointer = WASM_MEMORY_BUFFER.as_ptr();
  }

  return pointer;
}

// Function to read from index 1 of our buffer
// And return the value at the index
#[wasm_bindgen]
pub fn read_wasm_memory_buffer_and_return_index_one() -> u8 {
  let value: u8;
  unsafe {
    value = WASM_MEMORY_BUFFER[1];
  }
  return value;
}