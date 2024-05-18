use std::io;

fn main() {
    /* 데이터 타입 */

    let korean = '뷁';
    println!("The value of korean is: {korean}");
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The value of first is: {first}");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
