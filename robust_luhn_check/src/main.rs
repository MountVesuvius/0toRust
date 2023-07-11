// Thank you to JayDepp
// https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c: &char| !c.is_whitespace()).collect()
}

pub fn is_valid(code: &str) -> bool {
    if remove_whitespace(code).len() <= 1 {
        return false
    }

    let mut digit: usize = 0;
    for (i, val) in remove_whitespace(code).chars().rev().enumerate() {
        if !val.is_digit(10) {
            return false
        } 

        let mut item:usize = val.to_digit(10).unwrap() as usize;
        item = item * (i % 2 + 1);
        // Less than 9 add, else subtract 9 then add
        if item <= 9 {digit += item}
        else { digit += item - 9 }
    }

    if digit % 10 != 0 {
        return false
    }

    true
}


fn main() {
    // True
    let a:bool = is_valid("234 567 891 234");
    println!("{}", a);
}

#[cfg(test)]
mod tests;
