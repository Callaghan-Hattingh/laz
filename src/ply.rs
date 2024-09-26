use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek};

#[derive(Debug)]
pub struct PlyHeader {
    pub format: String,
    pub vertex_count: usize,
    pub properties: Vec<String>,
}

#[derive(Debug)]
pub struct Vertex {
    pub time: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub intensity: f32,
    pub ring: u8,
    pub return_num: u8,
    pub range: f32,
}

pub fn read_ply_header(file_path: &str) -> io::Result<(PlyHeader, u64)> {
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

pub fn read_binary_data(
    file_path: &str,
    header: &PlyHeader,
    header_size: u64,
) -> io::Result<Vec<Vertex>> {
    let mut file = File::open(file_path)?;
    file.seek(io::SeekFrom::Start(header_size))?; // Move to the end of the header

    let mut vertices = Vec::new();

    for index in 0..header.vertex_count {

        if index > 2 && index < header.vertex_count - 5 {
            continue;
        }

        println!("index: {}", index);

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
    }

    Ok(vertices)
}
