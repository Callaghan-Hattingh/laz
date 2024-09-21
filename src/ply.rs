use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek};

#[derive(Debug)]
struct PlyHeader {
    format: String,
    vertex_count: usize,
    properties: Vec<String>,
}

#[derive(Debug)]
struct Vertex {
    time: f64,
    x: f64,
    y: f64,
    z: f64,
    intensity: f32,
    ring: u8,
    return_num: u8,
    range: f32,
}

fn read_ply_header(file_path: &str) -> io::Result<(PlyHeader, u64)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut header = PlyHeader {
        format: String::new(),
        vertex_count: 0,
        properties: Vec::new(),
    };
    let mut header_size = 0;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        header_size += line.len() as u64 + 1; // Include newline character

        if line.starts_with("format") {
            header.format = line.to_string();
        } else if line.starts_with("element vertex") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 3 {
                header.vertex_count = parts[2].parse::<usize>().unwrap_or(0);
            }
        } else if line.starts_with("property") {
            header.properties.push(line.to_string());
        } else if line == "end_header" {
             // header_size += 1;
            break;
        }
    }

    Ok((header, header_size))
}

fn read_binary_data(
    file_path: &str,
    header: &PlyHeader,
    header_size: u64,
) -> io::Result<Vec<Vertex>> {
    let mut file = File::open(file_path)?;
    file.seek(io::SeekFrom::Start(header_size))?; // Move to the end of the header

    let mut vertices = Vec::new();

    for index in 0..header.vertex_count {
        let time = file.read_f64::<LittleEndian>()?;
        let x = file.read_f64::<LittleEndian>()?;
        let y = file.read_f64::<LittleEndian>()?;
        let z = file.read_f64::<LittleEndian>()?;
        let intensity = file.read_f32::<LittleEndian>()?;
        let ring = file.read_u8()?;
        let return_num = file.read_u8()?;
        let range = file.read_f32::<LittleEndian>()?;

        vertices.push(Vertex {
            time,
            x,
            y,
            z,
            intensity,
            ring,
            return_num,
            range,
        });

        if index > 20 {
            break;
        }
    }

    Ok(vertices)
}

fn main() {
    let file_path = "./map/global_georeferenced_wildcat.ply"; 

    let tuple_header = match read_ply_header(file_path) {
        Ok(header) => {
            println!("PLY Header: {:?}\n\n", header.0);
            for h in &header.0.properties {
                println!("{:?}", h);
            }
            println!("length of header: {}\n", header.1);
            header
        }
        Err(e) => {
            eprintln!("Failed to read PLY header: {}", e);
            return;
        }
    };

    match read_binary_data(file_path, &tuple_header.0, tuple_header.1) {
        Ok(vertices) => {
            for vertex in vertices {
                println!("{:?}\n", vertex);
            }
        }
        Err(e) => {
            eprintln!("Failed to read binary data: {}", e);
        }
    }
}
