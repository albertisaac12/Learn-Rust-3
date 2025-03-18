use std::io::{self,ErrorKind,Read};
use std::fs::{self,File};
use std::error::Error;

/*

    The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode. 
    Consult the standard library documentation for more information on implementing the Termination trait for your own types.


*/


fn main()-> Result<(), Box<dyn Error>> {

    // let blah = propagation().expect("Something went wrong"); // now the propagated error is handeled over here

    
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

// error propagation means that simply pass the error generated to a different scope for it to be handled

fn propagation() -> Result<String,io::Error> { // here in this function return the error
    let file = File::open("blah.txt");

    let mut read = match file {
        Ok(fp)=> fp,
        Err(e) => return  Err(e)  // if you remove the return this will throw an error meaning match type will return two different values which is not good
    };

    let mut username = String::new();

    match read.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }

}
 

// ? implements the from trait that essentially converts one type to a different type
fn propagation_with_question_mark()-> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut userName = String::new();
    file.read_to_string(&mut userName)?;
    Ok(userName)

}


fn better_Way_to_Read_files() {
    let read = fs::read_to_string("blah.txt").expect("File must exist");

}


fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}