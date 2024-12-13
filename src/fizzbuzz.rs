use std::{
    cmp::max, collections::{hash_set, HashMap, HashSet}, i32::MIN, result
};

pub fn fizz_buzz(num: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in 1..num {
        if i % 3 == 0 && i % 5 == 0 {
            result.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            result.push("Fizz".to_string());
        } else if i % 5 == 0 {
            result.push("Buzz".to_string());
        } else {
            result.push(i.to_string());
        }
    }

    // let a = 0..num;
    // let w: Vec<String> = a.filter(|x| x %3 ==0 && x % 5 ==0)
    // .map(|i| result.push(i.to_string()))
    // println!("{:?}", a);

    // let result: Vec<String> = (1..num).map(|i| {
    //     if i % 3 == 0 && i % 5 == 0 {
    //         "FizzBuzz".to_string();
    //     } else if i % 3 == 0 {
    //         "Fizz".to_string();
    //     } else if i % 5 == 0 {
    //         "Buzz".to_string();
    //     } else{
    //         i.to_string();
    //     }
    // }).collect();
    result
}

