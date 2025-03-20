// struct with generics
// struct point<T:Copy> { // T should be of the same data type
//     x:T,
//     y:T
// }

// impl<T:Copy> point<T> {
//     fn x(&self) -> T {
//         self.x
//     }
// }
struct point<T> { // T should be of the same data type
    x:T,
    y:T
}

impl<T> point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct point2<T,U>{ // now all T's will be of same type whereas all U's will be of a different type 
    x:T,
    y:U
}
fn main() {

    let v = vec![5,89,6,2,8,3,4,5,2,1];

    let largest = largest(&v);

    println!("{largest}");

    // println!("{:?}",v);

    let integer = point { x: 5, y: 10 };
    let float = point { x: 1.0, y: 4.0 };

    let both_integer = point2 { x: 5, y: 10 };
    let both_float = point2 { x: 1.0, y: 4.0 };
    let integer_and_float = point2 { x: 5, y: 4.0 };
}

// extracting function => basically make the code modular to avoid code duplication

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// writing the largest function with generics in it

// fn genric_largest<T>(list: &[T])-> &T {

//     let mut largest = &list[0];

//     for item in list {
//         if item > largest { // things to note with generics you cant actually use it to compare different values reason being generic can hold any value and those two types may not be comparable
//             largest = item;
//         }
//     }
//     /*
//         For now, know that this error states that the body of largest won’t work for all possible types that T could be. 
//         Because we want to compare values of type T in the body, we can only use types whose values can be ordered. 
//         To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types 
//     */

//     largest

// }

fn genric_largest_working<T: PartialOrd>(list: &[T])-> &T { // we implement the PartialOrd on the generic T
/*
    To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types
    we restrict the types valid for T to only those that implement PartialOrd
*/

    let mut largest = &list[0];

    for item in list {
        if item > largest { 
        }
    }
    largest

}


