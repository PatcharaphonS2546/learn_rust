fn reverse_words(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut reversed = String::new();
    for word in words.iter().rev() {
        reversed.push_str(word);
        reversed.push(' ');
    }
    reversed.trim_end().to_string()
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return true;
    }

    let mut i = 0;
    let mut j = chars.len() - 1;
    while i < j {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn main() {
    println!("{}", reverse_words("one"));
    println!("{}", reverse_words("สวัสดี ชาวโลก"));
    println!("{}", is_palindrome("level")); // true
    println!("{}", is_palindrome("rust")); // false
    println!("{}", is_palindrome("a")); // true
    println!("{}", is_palindrome("นาน")); // true
    println!("{}", is_palindrome("")); // true
}
