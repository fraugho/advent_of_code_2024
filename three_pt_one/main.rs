use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("corrupted.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut sum: u32 = 0;
    let mut i = 0;
    let bytes = content.as_bytes();

    while i < bytes.len() {
        // Look for "mul("
        if i + 3 < bytes.len() && 
        bytes[i] == b'm' && 
        bytes[i + 1] == b'u' && 
        bytes[i + 2] == b'l' && 
        bytes[i + 3] == b'(' {

            i += 4; // Skip past "mul("
            let mut num1 = 0u32;

            // Parse first number
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                num1 = num1 * 10 + (bytes[i] - b'0') as u32;
                i += 1;
            }

            // Must find comma next
            if i < bytes.len() && bytes[i] == b',' {
                i += 1;
                let mut num2 = 0u32;

                // Parse second number
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    num2 = num2 * 10 + (bytes[i] - b'0') as u32;
                    i += 1;
                }

                // Must end with closing parenthesis
                if i < bytes.len() && bytes[i] == b')' {
                    sum += num1 * num2;
                }
            }
        }
        i += 1;
    }

    println!("sum {}", sum);
    Ok(())
}
