fn main() {
    let mut s: String = String::from("hello world");

    let literal = "hello world";

    let word = first_word(&s);
    println!("{}", word);

    let word = first_word(&literal);
    println!("{}", word);

    let word = first_word(literal);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes
        .iter()
        .enumerate()
    {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
