pub fn is_valid(code: &str) -> bool {
    if code.trim().len() > 1 && is_numeric(code) {
        let code = remove_whitespaces(code);
        validation(&code)
    } else {
        false
    }
}

// checks for invalid characters (only digits and spaces are allowed)
pub fn is_numeric(code: &str) -> bool {
    let chars_are_valid: Vec<bool> = code.chars().map(|c| c.is_numeric() || c == ' ').collect();
    !chars_are_valid.contains(&false)
}

// removes all whitespaces
pub fn remove_whitespaces(code: &str) -> String {
    code.chars().filter(|c| !c.is_whitespace()).collect()
}

// validates the remaining number with the appropiate calculation
pub fn validation(code: &str) -> bool {
    let mut arr: Vec<u32> = code.chars().map(|n| n.to_digit(10).unwrap()).collect();
    arr.iter_mut().enumerate().for_each(|(i, num)| {
        if code.len() % 2 == 0 {
            if i % 2 == 0 {
                *num *= 2;
                if *num > 9 {
                    *num -= 9;
                }
            }
        } else {
            if i % 2 == 1 {
                *num *= 2;
                if *num > 9 {
                    *num -= 9;
                }
            }
        }
    });
    let sum = arr.iter().sum::<u32>();
    sum % 10 == 0
}
