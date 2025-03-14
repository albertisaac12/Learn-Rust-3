use std::collections;
fn main() {
    // let mut v:Vec<u32> = Vec::new();

    // v.push(10);
    // v.push(10);
    // v.push(10);

    // let mut m = vec![1,2,3];

    // // two ways in which you can access the value of the vector
    // // 1) simply take a reference to that data
    // // 2) Use the Option<T> with .get() method to see if the value exists or not.
    // let second = &m[1]; // immutable borrow

    // m.push(10); // will throw error as it a mutable borrow

    // // m[0] = 100;

    // // let first = m.get(0);

    // // match first {
    // //     Some(t) => println!("{t}"),
    // //     None=> println!("there is no index")
    // // }





    // let mut m = vec![1,3,4,5];

    // let first = &m[0]; // this gets dropped if never used before making a mutable borrow

    // m.push(100); 

    // println!("{}",&m[4]);

    // println!("{}", first);



    // iteration over vectors

    let mut m = vec![1,2,3,4];

    for values in &m {
        println!("{values}");
    }

    for values in &mut m {
        *values+=50;
    }

    {
        let v = vec![1,2,3,4];
    } // v is dropped here as soon as it goes out of the scope

}



