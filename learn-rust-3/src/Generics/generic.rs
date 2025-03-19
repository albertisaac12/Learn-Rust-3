fn main() {

    let v = vec![5,89,6,2,8,3,4,5,2,1];

    let largest = largest(&v);

    println!("{largest}");

    // println!("{:?}",v);

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