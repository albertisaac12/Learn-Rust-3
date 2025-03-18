use std::fs::File;

fn main() {


    // types of errors 
    // recoverable => are errors which do not cause the program to fail.
    // unrecoverable => this allows program to terminate immediately and provide feedback to the caller of the program.


    /*
        1) unwrap(self): T
        2) expect(self, msg: &str): T // outputs a custom message before panicking in addition to the contents of the error.
    */
    
    // panic!("This will end");
    // println!("The program will not work");

    // let f = File::open("./meow.txt").expect("no such file");

    // let f = File::open("./meow.txt").unwrap();
    let i:u32 =8;
    // match isSeven(i) {
    //     Ok(t)=> {
    //         println!("{t}");
    //     }
    //     Err(q) => {
    //         panic!("{q}");
    //     }
    // }

    let solution = isSeven(8).unwrap(); // optimal solution
    println!("{solution}");

}

fn isSeven(i:u32) -> Result<bool,String> {
    match i {
        7 => {
            Ok(true)
        }
        _=> {
            Err("Not the number 7".to_string())
        }
    }
}