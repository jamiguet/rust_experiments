use rprompt::prompt_reply;
use std::collections::HashMap;

fn add_entry_to_list(map: &mut HashMap<String, Vec<String>>, key: String, value: String) {
    map.entry(key).or_insert_with(Vec::new).push(value);
}

pub fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    println!("Enter commands in the format 'Add <value> to <key>'. Press Enter on an empty line to finish:");

    loop {
        // Prompt user for input
        match prompt_reply("Enter your command: ") {
            Ok(input) => {
                let input = input.trim();
                if input.is_empty() {
                    break;
                }

                // Parse the input in the "Add <value> to <key>" format
                if let Some((value, key)) = parse_input(input) {
                    add_entry_to_list(&mut map, key, value);
                } else {
                    println!(
                        "Invalid input format. Please enter in 'Add <value> to <key>' format."
                    );
                }
            }
            Err(_) => {
                println!("Error reading input. Try again.");
            }
        }
    }

    // Ask the user which department they want to see
    loop {
        match prompt_reply("Which department do you want to see? (Press Enter to exit): ") {
            Ok(department) => {
                let department = department.trim();
                if department.is_empty() {
                    break;
                }

                match map.get(department) {
                    Some(list) => {
                        println!("{}: {:?}", department, list);
                    }
                    None => {
                        println!("Department '{}' does not exist.", department);
                    }
                }
            }
            Err(_) => {
                println!("Error reading input. Try again.");
            }
        }
    }

    println!("Program terminated.");
}

fn parse_input(input: &str) -> Option<(String, String)> {
    if input.starts_with("Add ") && input.contains(" to ") {
        let parts: Vec<&str> = input[4..].splitn(2, " to ").collect();
        if parts.len() == 2 {
            return Some((parts[0].trim().to_string(), parts[1].trim().to_string()));
        }
    }
    None
}
