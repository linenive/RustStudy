extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자 추측!");
    let mut prev = 0;
    let mut trolling_count = 0;

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("추측한 수를 입력하세요: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("입력한 값이 숫자가 아닙니다. 다시 입력해주세요.");
                continue;
            }
        };

        println!("당신이 입력한 수는 {guess} 입니다.");

        let minus = secret_number - 5;
        match guess.cmp(&secret_number) {
            Ordering::Less => match guess.cmp(&minus) {
                Ordering::Less => println!("너무 작아요!"),
                // 기본
                _ => println!("조금 작아요!"),
            },
            Ordering::Greater => println!("너무 커요!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }

        if (prev < secret_number && guess < prev) || (prev > secret_number && guess > prev) {
            if trolling_count == 0 {
                println!("그렇게 하면 의미가 없을 텐데요.");
            } else {
                println!("맞출 마음이 없구나?");
                // 에러 발생시키기
                panic!("프로그램 종료");
            }

            trolling_count += 1;
        }
        prev = guess;
    }
}
