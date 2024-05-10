extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자 추측!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("추측한 수를 입력하세요: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("입력한 값이 숫자가 아닙니다. 다시 입력해주세요.");
                    continue;
                },
            };

        println!("당신이 입력한 수는 {} 입니다.", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너무 작아요!"),
            Ordering::Greater => println!("너무 커요!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}