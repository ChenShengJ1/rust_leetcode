//3.无重复字符的最长子串
pub fn length_of_longest_substring(s: String) -> i32 {
    let (mut ans, mut count) = (0, 0);
    let mut map = std::collections::HashMap::new();
    let mut s: Vec<char> = s.chars().collect();
    let mut l = 0;

    s.iter().enumerate().for_each(|(i, c)| {
        match map.get(c) {
            Some(&i) => {
                for c in &s[l..i] {
                    map.remove(c);
                }
                count -= i - l;
                l = i + 1;
            }
            None => {
                count += 1;
                ans = ans.max(count);
            }
        }
        map.insert(*c, i);
    });

    ans as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn example01() {
        let s = String::from("abcabcbb");
        assert_eq!(length_of_longest_substring(s), 3);
        println!("example01: pass");
    }

    #[test]
    fn example02() {
        let s = String::from("bbbbb");
        assert_eq!(length_of_longest_substring(s), 1);
        println!("example02: pass");
    }

    #[test]
    fn example03() {
        let s = String::from("pwwkew");
        assert_eq!(length_of_longest_substring(s), 3);
        println!("example03: pass");
    }

    #[test]
    fn example04() {
        let s = String::from("aabaab!bb");
        assert_eq!(length_of_longest_substring(s), 3);
        println!("example04: pass");
    }
}