fn main() {
    let num = 6;

    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, 2");
    }

    let condition  = true;
    let number = if condition { 5 } else { 6 };

    println!("the value of number is: {}", number);
}
