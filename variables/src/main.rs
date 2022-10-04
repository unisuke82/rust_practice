use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    const MAX_POINTS: u32 = 100_000;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is : {}", x);

    let spaces = "       ";
    let spaces = spaces.len();
    println!("The value of spaces is : {}", spaces);

    let f1 = 2.0; // f64

    let f2: f32 = 3.0; // f32

    let sum = 5 + 10;

    // subtraction
    // 引き算
    let difference = 95.5 - 4.3;

    // multiplication
    // 掛け算
    let product = 4 * 30;

    // division
    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // 結果は0

    // remainder
    // 余り
    let remainder = 43 % 5;

    let tup = (500, 'h', 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {}", a[3]);

    let a = [3; 5];
    println!("The value of a is: {}", a[2]);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );
}
