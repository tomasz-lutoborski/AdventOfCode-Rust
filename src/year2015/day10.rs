fn look_and_say(s: String) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        let mut count = 1;
        while let Some(&next) = chars.peek() {
            if next == c {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }
        result.push_str(&count.to_string());
        result.push(c);
    }
    result
}

pub fn solve() {
    let mut content = "1113122113".to_string();
    for _ in 0..50 {
        content = look_and_say(content);
    }
    println!("{}", content.len());
}
