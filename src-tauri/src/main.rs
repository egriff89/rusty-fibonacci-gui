// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use num_bigint::BigUint;
use num_traits::{Zero, One};

#[tauri::command]
fn fib(nth: usize) -> String {
  let mut f0: BigUint = Zero::zero();
  let mut f1: BigUint = One::one();

  for _ in 0..nth {
    let f2 = f0 + &f1;
    f0 = f1;
    f1 = f2
  }
  
  f0.to_string()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fib])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
