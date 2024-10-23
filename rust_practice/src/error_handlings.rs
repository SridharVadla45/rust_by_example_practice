use std::{io::Error, fs::{self, File}};


#[allow(dead_code)]
pub fn  test_errors_panic(){
  panic!("explict panic call ");
}

#[allow(unused)]
pub fn test_panic_vec() -> i32 {
   let numbers =vec![1,2,3];

  numbers[99]
}
pub fn test_recoverable_errors() -> Result<File, Error>{
 fs::File::open("hello.txt")

}

