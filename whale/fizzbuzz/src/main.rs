fn main() {

}

fn sosu(count: u32) -> [u32; count] {

}

fn encrypt(text: &str, shift: i16) -> String{
    let code_a: i16 = 'a' as i16;
    let code_z: i16 = 'z' as i16;
    println!("A{}, Z{}", code_a, code_z);

    let mut result = String::new();
    for char in text.chars() {
        let mut code = char as i16;
        println!("ずらす前{}", code);

        if code_a <= code && code <= code_z {
            code -= shift
        }

        println!("{}", code);

        result.push((code as u8) as char)
    }

    result
}

fn coin() {
    let target = 3950;

    for count500 in 0..11 {
        for count100 in 0..4 {
            for count50 in 0..11 {
                let total = 500 * count500 + 100 * count100 + 50 * count50;
                if target == total {
                    println!("500: {}, 100: {}, 50: {}", count500, count100, count50);
                }
            }
        }
    }
}

fn kaimono() {
    let price = 98000.0;

    let postage_a = 1200.0;
    let discount_a = 0.8;

    let postage_b = 0.0;
    let discount_b = 0.9;

    let selling_price_a = price * discount_a + postage_a;
    let selling_price_b = price * discount_b + postage_b;

    println!("A: {}, B: {}", selling_price_a, selling_price_b)
}

fn fib() {
    let mut a = 1;
    let mut b = 1;

    println!("{}", a);
    println!("{}", b);
    for _ in 1..30 {
        let tmp = a;
        a = b;
        b = tmp + b;
        println!("b {}", b);
    }
}

fn gengo() {
    for year in 1926..2027 {
        print!("西暦{}年 = ", year);
        if 2019 <= year {
            if year == 2019 {
                println!("令和元年");
            } else {
                println!("令和{}年", year - 2019 + 1);
            }
        } else if 1989 <= year {
            if year == 1989 {
                println!("平成元年");
            } else {
                println!("平成{}年", year - 1989 + 1);
            }
        } else if 1926 <= year {
            if year == 1926 {
                println!("昭和元年");
            } else {
                println!("昭和{}年", year - 1926 + 1);
            }
        }
    }
}

fn kuku() {
    for num1 in 1..10 {
        for num2 in 1..10 {
            print!("{:3},", num1 * num2)
        }
        println!("")
    }
}

fn three() {
    for number in 1..51 {
        if number % 3 == 0 || number % 10 == 3 || number / 10 == 3 {
            println!("A");
        } else {
            println!("{}", number);
        }
    }
}

fn fizzbuzz() {
    for number in 1..101 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}
