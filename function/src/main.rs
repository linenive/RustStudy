fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    example_statements();
    let x = return5();
    println!("The value of return5 is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*
구문은 어떤 동작을 수행하고 값을 반환하지 않는 명령입니다.
표현식은 결괏값을 평가합니다. 몇 가지 예제를 살펴봅시다.
*/

fn example_statements() {
    let y = 6; // 이것은 구문입니다.
               // let x = (let y = 6); // 이것은 표현식이 아니므로 컴파일되지 않습니다.
    let x = {
        let y = 6;
        y + 1 // 이것은 표현식입니다.
    };
    println!("The value of x is: {x}");
}

fn return5() -> i32 {
    5
}
