use md5;

fn generate_hash(n: u32, prefix: &str) -> md5::Digest {
    md5::compute(format!("{}{}", prefix, n))
}

fn starts_with_five_zeros(digest: md5::Digest) -> bool {
    let bytes = digest.0;
    bytes[0] == 0 && bytes[1] == 0 && bytes[2] & 0xF0 == 0
}

fn decode_password(prefix: &str) -> String {
    let mut password = String::new();
    let mut n = 0;

    while password.len() < 8 {
        let hash = generate_hash(n, prefix);
        if starts_with_five_zeros(hash) {
            password.push(format!("{:x}", hash).chars().nth(5).unwrap());
        }

        n += 1;
    }

    password
}

fn decode_clever_password(prefix: &str) -> String {
    let mut password: [Option<char>; 8] = [None; 8];
    let mut n = 0;
    let mut elems = 0;

    while elems < 8 {
        let hash = generate_hash(n, prefix);
        if starts_with_five_zeros(hash) {
            let hash_string = format!("{:x}", hash);
            if let Some(digit) = hash_string.clone().chars().nth(5).unwrap().to_digit(10) {
                match digit {
                    0..=7 => {
                        if password
                            .into_iter()
                            .nth(usize::try_from(digit).unwrap())
                            .is_none()
                        {
                            password[digit as usize] = hash_string.clone().chars().nth(6);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    password.into_iter().map(|c| c.unwrap()).collect()
}

pub fn solve() {
    let content = "wtnhxymk";
    println!("{}", decode_clever_password(content));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_hash() {
        assert!(starts_with_five_zeros(generate_hash(5278568, "abc")))
    }
}
