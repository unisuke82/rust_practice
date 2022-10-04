fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    let reference_to_nothing = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s //sへの参照を返す
// } //sがスコープを抜けてドロップされる

fn no_dangle() -> String {
    let s = String::from("hello");

    s //sへの参照を返す
} //sがスコープを抜けてドロップされる