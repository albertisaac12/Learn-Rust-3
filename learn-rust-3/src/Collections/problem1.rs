use std::collections::HashMap;

/*
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
    and mode (the value that occurs most often; a hash map will be helpful here) of the list.


*/


fn main() {
    // step 1 : = take a list , sort it
    let list = vec![1,2,3,3,3,3,4,6,8,10];

    let len = list.len();

    let median = if len% 2 == 0 {
        (list[len / 2 - 1] + list[len / 2]) / 2  
    }else {
        list[len/2]
    };

    let mut map = HashMap::new();


    for val in list {
       let count = map.entry(val).or_insert(0);
       *count+=1;
    }

    

    

}