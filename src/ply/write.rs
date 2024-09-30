use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;


/// Represents a write vertex in the PLY file.
#[repr(packed)] // is the data packed or has it got padding.
#[derive(Debug, Clone)]
pub struct WriteVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub time: f64,
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub intensity: f32,
    pub ring: u8,
    pub return_num: u8,
    pub range: f32,
}

impl WriteVertex {
    /// Writes the write vertex data to the provided writer in binary little endian format.
    fn write_vertex<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_f64::<LittleEndian>(self.x)?;
        writer.write_f64::<LittleEndian>(self.y)?;
        writer.write_f64::<LittleEndian>(self.z)?;
        writer.write_f64::<LittleEndian>(self.time)?;
        writer.write_f64::<LittleEndian>(self.nx)?;
        writer.write_f64::<LittleEndian>(self.ny)?;
        writer.write_f64::<LittleEndian>(self.nz)?;
        writer.write_f32::<LittleEndian>(self.intensity)?;
        writer.write_u8(self.ring)?;
        writer.write_u8(self.return_num)?;
        writer.write_f32::<LittleEndian>(self.range)?;
        Ok(())
    }
    
}



