use std::fs::{self,File};
use std::io::{self,ErrorKind,Read};
fn main() {
    let v = vec![1,2,3];

    // v[99]; // will panic


    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {

            ErrorKind::NotFound => match File::create("./hello.txt") {
                Ok(fc) => fc,
                Err(e)=> panic!("Problem creating the file: {e:?}")
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
            
        }
    };




}



fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}



fn read_username_from_file_better() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}



fn read_username_from_file_even_better() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
