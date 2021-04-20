// first try
pub fn reverse(input: &str) -> String {
    let mut chars: Vec<&str> = input.split("").collect();
    chars.reverse();
    let string: String = chars.concat();
    string
}

// after mentoring
pub fn reverse_refactored(input: &str) -> String {
    input.chars().rev().collect()
}