use std::{collections::HashMap, fs};
fn main() {
    let contents = fs::read_to_string("list.txt")
        .unwrap()
        .replace("   ", ",")
        .replace("\n", ",");
    let mut left_vec = Vec::<i32>::new();
    let mut right_vec = Vec::<i32>::new();
    let mut diff = 0;

    for (i, el) in contents.split(',').enumerate() {
        if i % 2 == 0 {
            left_vec.push(el.parse::<i32>().unwrap());
        } else {
            right_vec.push(el.parse::<i32>().unwrap());
        }
    }

    left_vec.sort();
    right_vec.sort();

    // Part 1
    for n in 0..left_vec.len() {
        let difference = left_vec[n] - right_vec[n];
        diff += difference.abs();
    }

    println!("{}", diff);

    // Part 2
    let mut similarity_score = 0;
    let mut right_hash = HashMap::<i32, i32>::new();

    for el in right_vec {
        if right_hash.contains_key(&el) {
            *right_hash.get_mut(&el).unwrap() += 1;
        } else {
            right_hash.insert(el, 1);
        }
    }

    for el in left_vec {
        if right_hash.contains_key(&el) {
            similarity_score += el * *right_hash.get_mut(&el).unwrap();
        }
    }
    println!("{}", similarity_score);
}
