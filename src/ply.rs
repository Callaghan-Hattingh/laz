use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

/// Represents the header of a PLY file.
#[derive(Debug)]
pub struct PlyHeader {
    pub format: String,
    pub vertex_count: usize,
    pub properties: Vec<String>,
}

/// Represents a vertex in the PLY file.
// #[repr(C)]
#[repr(packed)] // is the data packed or has it got padding.
#[derive(Debug, Clone)]
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

impl Vertex {
    /// Reads a single vertex from a binary reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a binary reader.
    ///
    /// # Returns
    ///
    /// A `Vertex` instance populated with the read data.
    pub fn read_from<R: ReadBytesExt>(reader: &mut R) -> io::Result<Self> {
        let time = reader.read_f64::<LittleEndian>()?;
        let x = reader.read_f64::<LittleEndian>()?;
        let y = reader.read_f64::<LittleEndian>()?;
        let z = reader.read_f64::<LittleEndian>()?;
        let intensity = reader.read_f32::<LittleEndian>()?;
        let ring = reader.read_u8()?;
        let return_num = reader.read_u8()?;
        let range = reader.read_f32::<LittleEndian>()?;

        Ok(Vertex {
            time,
            x,
            y,
            z,
            intensity,
            ring,
            return_num,
            range,
        })
    }
}

/// Reads the header of a PLY file.
///
/// # Arguments
///
/// * `file_path` - The path to the PLY file.
///
/// # Returns
///
/// A `PlyHeader` struct and the size of the header in bytes.
pub fn read_ply_header<P: AsRef<Path>>(file_path: P) -> io::Result<(PlyHeader, u64)> {
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
            break;
        }
    }

    Ok((header, header_size))
}

/// Reads a specified number of vertices from the beginning of the binary data.
///
/// # Arguments
///
/// * `file_path` - The path to the PLY file.
/// * `header` - A reference to the `PlyHeader`.
/// * `header_size` - The size of the header in bytes.
/// * `count` - The number of vertices to read from the start.
///
/// # Returns
///
/// A vector of `Vertex` structs representing the first `count` vertices.
pub fn read_first_vertices<P: AsRef<Path>>(
    file_path: P,
    header: &PlyHeader,
    header_size: u64,
    count: usize,
) -> io::Result<Vec<Vertex>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::Start(header_size))?;

    let vertices_to_read = count.min(header.vertex_count);
    let mut vertices = Vec::with_capacity(vertices_to_read);

    for _ in 0..vertices_to_read {
        let vertex = Vertex::read_from(&mut reader)?;
        vertices.push(vertex);
    }

    Ok(vertices)
}

/// Reads a specified number of vertices from the end of the binary data.
///
/// # Arguments
///
/// * `file_path` - The path to the PLY file.
/// * `header` - A reference to the `PlyHeader`.
/// * `header_size` - The size of the header in bytes.
/// * `count` - The number of vertices to read from the end.
///
/// # Returns
///
/// A vector of `Vertex` structs representing the last `count` vertices.
pub fn read_last_vertices<P: AsRef<Path>>(
    file_path: P,
    header: &PlyHeader,
    header_size: u64,
    count: usize,
) -> io::Result<Vec<Vertex>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let total_vertices = header.vertex_count;
    let vertices_to_read = count.min(total_vertices);
    let vertex_size = std::mem::size_of::<Vertex>() as u64;
    let mut vertices = Vec::with_capacity(vertices_to_read);

    // Calculate the position to seek to for the last `vertices_to_read` vertices
    // Note: This assumes fixed-size binary data for vertices.
    println!("header_size {}\n total_vertices {}\n  vertices_to_read {}\n vertex_size {}\n", header_size, total_vertices, vertices_to_read, vertex_size);
    let seek_position =
        header_size + (total_vertices as u64 - vertices_to_read as u64) * vertex_size;

    reader.seek(SeekFrom::Start(seek_position))?;

    for _ in (total_vertices - vertices_to_read)..total_vertices {
        let vertex = Vertex::read_from(&mut reader)?;
        vertices.push(vertex);
    }

    Ok(vertices)
}

/// Example function that reads both first and last vertices based on specified counts.
///
/// # Arguments
///
/// * `file_path` - The path to the PLY file.
///
/// # Returns
///
/// A tuple containing vectors of the first and last `Vertex` structs.
pub fn read_first_and_last_vertices<P: AsRef<Path>>(
    file_path: P,
) -> io::Result<(Vertex, Vertex)> {
    let (header, header_size) = read_ply_header(&file_path)?;

    let mut first_vertex = read_first_vertices(&file_path, &header, header_size, 1)?;
    let mut last_vertex = read_last_vertices(&file_path, &header, header_size, 1)?;

    Ok((first_vertex.pop().unwrap(), last_vertex.pop().unwrap()))
}
