fn main() {
    println!("hello world");

    let mut s = String::new();

    let data = "initial content";
    let s = data.to_string();

    let s = "initial content".to_string();

    let s = String::from("initial content");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let mut s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意
    println!("s3 {}", s3);
    s3 = "a".to_string();
    println!("s3 {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
    //let h = s1[0];

    let len = String::from("Hola").len();
    let len = String::from("Здравствуйте").len();
    println!("len {}", len);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

}