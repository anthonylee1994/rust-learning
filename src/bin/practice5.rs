fn main() {
    for input in ["25", "abc"] {
        match parse_age(input) {
            Ok(x) => println!("{} -> {}", input, x),
            Err(e) => println!("{} -> parse error: {}", input, e),
        }
    }
}

fn parse_age(s: &str) -> Result<u32, std::num::ParseIntError> {
    let age = s.parse::<u32>()?;
    Ok(age)
}
