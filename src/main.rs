// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;
use rand::{distributions::Uniform, Rng};

fn main() {

    let mut rng = rand::thread_rng();
    let range = Uniform::new(0.0, 1.0);

    let v: Vec<f64> = (0..10).map(|_| rng.sample(&range)).collect();

    // // placeholder array
    // let arr = [1, 2, 3, 4, 5, 6, 7, 8];

    // let mut idx = String::new();
    // io::stdin().read_line(&mut idx).expect("Failed to read line");

    // // attempt to convert guess String to a usize
    // let idx: usize = match idx.trim().parse() {
    //     Ok(num) => num,
    //     Err(error) => panic!("Problem parsing input: {:?}", error),
    // };
    // if idx >= arr.len() {
    //     println!("Invalid index");
    // }
    println!("vector: {:?}", v);
    // println!("element at index {}: {}", idx, arr[idx]);
}

fn mean() -> f64 {
    unimplemented!();
}

fn variance() -> f64 {
    unimplemented!();
}
