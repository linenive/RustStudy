fn main() {
    println!("Hello, world!");

    let f = 32.0;
    let c = fehrer_to_celsius(f);
    println!("{} F = {} C", f, c);

    let c = 0.0;
    let f = celsius_to_fehrer(c);
    println!("{} C = {} F", c, f);

    let n = 10;
    let fib = fibonacci(n);
    println!("fibonacci({}) = {}", n, fib);

    christmas_song();
}

fn fehrer_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fehrer(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn christmas_song() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            if j == 0 && i != 0 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }
        println!();
    }
}
