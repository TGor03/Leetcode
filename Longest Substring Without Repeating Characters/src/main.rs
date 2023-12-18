fn length_of_longest_substring(s: String) -> i32 {
   
}

fn main() {
    let s = String::from("abcabcbb");
    let result = length_of_longest_substring(s);
    println!(
        "Length of longest substring without repeating characters: {}",
        result
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring_case1() {
        // Test case 1: "abcabcbb"
        let s = String::from("abcabcbb");
        assert_eq!(length_of_longest_substring(s), 3);
    }

    #[test]
    fn test_length_of_longest_substring_case2() {
        // Test case 2: "bbbbb"
        let s = String::from("bbbbb");
        assert_eq!(length_of_longest_substring(s), 1);
    }

    #[test]
    fn test_length_of_longest_substring_case3() {
        // Test case 3: "pwwkew"
        let s = String::from("pwwkew");
        assert_eq!(length_of_longest_substring(s), 3);
    }

    #[test]
    fn test_length_of_longest_substring_case4() {
        // Test case 4: ""
        let s = String::from("");
        assert_eq!(length_of_longest_substring(s), 0);
    }

    #[test]
    fn test_length_of_longest_substring_case5() {
        // Test case 5: "abcdefg"
        let s = String::from("abcdefg");
        assert_eq!(length_of_longest_substring(s), 7);
    }
}
