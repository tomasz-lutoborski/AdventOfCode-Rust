use md5;
use std::fs::read_to_string;

fn find_hashing(key: String, prefix: &str) -> i32 {
    for n in 0..100000000 {
        let data = format!("{}{}", key, n);
        let hashed = md5::compute(data);
        let hashed_str = format!("{:x}", hashed);
        if hashed_str.starts_with(prefix) {
            return n;
        };
    }

    0
}

pub fn solve() {
    let content = "bgvyzdsv".to_string();
    println!("{:?}", find_hashing(content, "000000"));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_start_with() {
        let hashed = md5::compute("abcdef609043");

        println!("{:?}", "00000".as_bytes());

        assert!(hashed.starts_with("00000".as_bytes()));
    }
}
