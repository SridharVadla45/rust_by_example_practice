mod error_handlings;
fn main() {
    // println!("Hello, world!");
    // error_handlings::test_errors_panic();
    // let value = error_handlings::test_panic_vec();
    // println!("{0}",value);
    let result = error_handlings::test_recoverable_errors();

    match result {
    
        Ok(file) => {print!("file name : {:#?}",file);}
        Err(error) => {panic!("Error occured when opening  a file : {error:?}");}
    }

    
}
