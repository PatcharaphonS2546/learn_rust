fn calc_len(s: &str) -> usize {
    s.len()
}

fn add_exclaim(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("ownership");
    let len = calc_len(&s);
    println!("s: {}, len: {}", s, len);
    for _ in 1..=3 {
        add_exclaim(&mut s);
    }
    println!("s: {}", s);
}
