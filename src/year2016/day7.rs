use std::fs::read_to_string;

fn is_abba(s: &[u8]) -> bool {
    s[0] == s[3] && s[1] == s[2] && s[0] != s[1]
}

fn is_ip_tls(ip: &str) -> bool {
    let mut in_brackets = false;
    let mut has_abba = false;
    for i in 0..ip.len() - 3 {
        let s = &ip.as_bytes()[i..i + 4];
        if s[0] == b'[' {
            in_brackets = true;
        } else if s[0] == b']' {
            in_brackets = false;
        } else if is_abba(s) {
            if in_brackets {
                return false;
            }
            has_abba = true;
        }
    }
    has_abba
}

fn is_ip_ssl(ip: &str) -> bool {
    let mut in_brackets = false;
    let mut abas = Vec::new();
    let mut babs = Vec::new();
    for window in ip.as_bytes().windows(3) {
        if window[0] == b'[' {
            in_brackets = true;
        } else if window[0] == b']' {
            in_brackets = false;
        } else if window[0] == window[2] && window[0] != window[1] {
            if in_brackets {
                babs.push([window[1], window[0]]);
            } else {
                abas.push([window[0], window[1]]);
            }
        }
    }
    for aba in &abas {
        if babs.contains(aba) {
            return true;
        }
    }
    false
}

pub fn solve() {
    let content = read_to_string("inputs/Year2016/Day7.txt").unwrap();
    let result = content.lines().filter(|ip| is_ip_ssl(ip)).count();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abba() {
        assert_eq!(is_abba(b"abba"), true);
        assert_eq!(is_abba(b"aaaa"), false);
        assert_eq!(is_abba(b"abca"), false);
    }
}
