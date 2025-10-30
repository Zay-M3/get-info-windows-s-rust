use std::io;

use crate::utils::interfase::*;
use crate::api::send_info::*;

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

/// The function `check_path_exists` takes a vector of file paths, checks if each path exists, counts
/// the number of elements in the path if it exists, and returns a vector of `AplicacionInfo` structs
/// containing information about each path.
/// 
/// Arguments:
/// 
/// * `paths`: The function `check_path_exists` takes a vector of strings `paths` as input. Each string
/// in the vector represents a file path that you want to check for existence and retrieve information
/// about its elements.
/// 
/// Returns:
/// 
/// The function `check_path_exists` returns a vector of `AplicacionInfo` structs, which contain
/// information about each path in the input vector `paths`.
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

/// The function `parse_to_json` takes a `SystemReport` struct reference and converts it to a
/// pretty-printed JSON string, returning a `Result` with either the JSON string or a
/// `serde_json::Error`.
/// 
/// Arguments:
/// 
/// * `report`: The `report` parameter in the `parse_to_json` function is of type `SystemReport`, which
/// is presumably a custom struct or type that contains information about a system. The function takes a
/// reference to this `SystemReport` as input and attempts to serialize it into a JSON string using
/// `serde
/// 
/// Returns:
/// 
/// The function `parse_to_json` returns a `Result` containing a `String` if the serialization of the
/// `SystemReport` struct to JSON is successful. If an error occurs during the serialization process, it
/// returns a `serde_json::Error`.
pub fn parse_to_json(report: &SystemReport) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string_pretty(report)?;
    Ok(json)
}


/// The function `print_and_send_json` prints a JSON representation of a `SystemReport` and sends the
/// information to a remote server.
/// 
/// Arguments:
/// 
/// * `report`: The `print_and_send_json` function takes a reference to a `SystemReport` struct as
/// input. This function first tries to parse the `SystemReport` into a JSON format. If successful, it
/// prints the JSON data and then proceeds to send the information to a remote server.
pub fn print_and_send_json(report: &SystemReport) {
    match parse_to_json(report) {
        Ok(json) => {
            let mut input = String::new();
            println!("Press y to print json or any other key to skip...");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim().eq_ignore_ascii_case("y") {
                println!("{}", json);
            }

            let mut input = String::new();

            println!("Press y to send info to remote server or any other key to skip...");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if !input.trim().eq_ignore_ascii_case("y") {
                return;
            }

            // ============= ENVIAR INFO A SERVIDOR REMOTO =============
            let info = Info {
                id: 1,
                name: "SistemaReporte".into(),
                active: true,
                winput: request_input_ticket(),
            };

            // Use a new runtime only when needed
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            match rt.block_on(send_info(&info)) {
                Ok(status) => println!("Información enviada con estado: {}", status),
                Err(e) => eprintln!("Error al enviar información: {}", e),
            }
        },
        Err(e) => eprintln!("Error generating JSON: {}", e),
    }
}