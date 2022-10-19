fn main() {
    let mut s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..11];
    println!("{}{}", hello, world);

    let len = s.len();
    let tail1 = &s[6..];
    let tail2 = &s[6..len];
    println!("{}{}", tail1, tail2);

    let len = first_word(&mut s);
    println!("{}", len);

    s = String::from("good morning");
    println!("{}", len);
}

fn first_word(s: &mut String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}