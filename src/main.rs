use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct PlyHeader {
    format: String,
    elements: Vec<String>,
    properties: Vec<String>,
}

fn read_ply_header(file_path: &str) -> io::Result<PlyHeader> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut header = PlyHeader {
        format: String::new(),
        elements: Vec::new(),
        properties: Vec::new(),
    };

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with("format") {
            header.format = line.to_string();
        } else if line.starts_with("element") {
            header.elements.push(line.to_string());
        } else if line.starts_with("property") {
            header.properties.push(line.to_string());
        } else if line == "end_header" {
            // End of the header section
            break;
        }
    }

    Ok(header)
}

fn main() {
    let file_path = "example.ply"; // Replace with your .ply file path
    match read_ply_header(file_path) {
        Ok(header) => println!("PLY Header: {:?}", header),
        Err(e) => eprintln!("Failed to read PLY header: {}", e),
    }
}
