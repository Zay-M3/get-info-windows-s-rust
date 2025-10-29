use std::io;


use crate::utils::interfase::*;

/// The function `request_input_ticket` in Rust prompts the user for input and returns the trimmed input
/// as a String.
/// 
/// Returns:
/// 
/// The function `request_input_ticket()` is returning a `String` value.
pub fn request_input_ticket() -> String {
    println!("Requesting input what do you need...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let winput = input.trim().to_string();

    return winput
}


/// The function `ask_paths_to_check` reads user input for paths separated by commas and returns them as
/// a vector of strings.
/// 
/// Returns:
/// 
/// A vector of strings containing the paths entered by the user, after splitting and trimming them.
pub fn ask_paths_to_check() -> Vec<String> {
    println!("Enter paths to check (separated by commas): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let paths: Vec<String> = input
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    paths
}

pub fn check_path_exists(paths: Vec<String>) -> Vec<AplicacionInfo> {
    let mut aplicaciones = Vec::new();
    
    for path in paths {
        let existe = std::path::Path::new(&path).exists();
        let elementos = if existe {
            if let Ok(entries) = std::fs::read_dir(&path) {
                let files: Vec<_> = entries.filter_map(Result::ok).collect();
                Some(files.len())
            } else {
                None
            }
        } else {
            None
        };
        
        aplicaciones.push(AplicacionInfo {
            ruta: path.to_string(),
            existe,
            elementos,
        });
    }

    aplicaciones
}