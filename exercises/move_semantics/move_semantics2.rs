// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// Solution 3 - mutably borrow mutable Vec<i32>
fn main() {
    let mut vec0: Vec<i32> = Vec::new();

    fill_vec(&mut vec0); // mutably borrow vec0

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    let vec1 = &mut vec0; // vec1 points to vec0
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    // println!("Vec0 is the same as Vec1: {}", eq(&vec0[0], &vec1[0]));
}

fn fill_vec(vec: &mut Vec<i32>) { // allow for mut borrows
    vec.push(22);
    vec.push(44);
    vec.push(66);
}

// Solution 1, create new object
// fn main() {
//     let vec0: Vec<i32> = Vec::new();

//     let mut vec1 = fill_vec(Vec::new()); // instead of using vec0 as arg, can just instantiate a new Vec<i32>

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// Another similar solution:
// fn main() {
//     let vec0: Vec<i32> = Vec::new();

//     let mut vec1 = fill_vec(vec0.clone()); // instead of using vec0 as arg, can just instantiate a new Vec<i32>

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }


// Solution 2: fill_vec borrows arg and creates a clone
// fn main() {
//     let vec0: Vec<i32> = Vec::new();

//     let mut vec1 = fill_vec(&vec0); // instead of using vec0 as arg, can just instantiate a new Vec<i32>

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }