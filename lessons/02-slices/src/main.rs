fn first_word(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[..i];
        }
    }
    s
}

fn sum(xs: &[i32]) -> i32 {
    let mut total = 0;
    for x in xs {
        total += x;
    }
    total
}

fn largest(xs: &[i32]) -> i32 {
    let mut largest = xs[0];
    for x in xs {
        if *x > largest {
            largest = *x;
        }
    }
    largest
}

fn main() {
    let mut s = String::from("rust");
    let word = first_word(&s);
    println!("First word: {word}");
    println!("{}", first_word("rust"));
    s.clear();

    let arr = [3, 9, 2, 7];
    let v = vec![1, 2, 3, 4];
    println!("Sum: {}", sum(&arr));
    println!("Sum: {}", sum(&v));
    println!("Sum: {}", sum(&v[1..3]));
    println!("Largest: {}", largest(&arr));

    let t = String::from("สวัสดี");
    println!("{}", t.len());
    println!("{}", t.chars().count());

    println!("{}", &t[0..3]);
    println!("{:?}", t.chars().next());
}
