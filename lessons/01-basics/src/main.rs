fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fizz_buzz() {
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn fib(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b += temp;
    }
    a
}

fn main() {
    let c = 0.0;
    let f = c_to_f(c);
    println!("{}°C = {}°F", c, f);
    println!("FizzBuzz");
    fizz_buzz();
    println!("fib(50) = {}", fib(50));
}
