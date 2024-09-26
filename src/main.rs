mod txt;
mod ply;

use txt::read_traj_txt_data;
use ply::{read_ply_header, read_binary_data};

fn read_traj() {
    let file_path_traj = "./map/global_georeferenced_wildcat_traj.txt";

    let traj = read_traj_txt_data(file_path_traj);

    println!("There are {} trajectories in the file.", traj.unwrap().len());
}

fn read_ply() {
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

fn main () {
    read_traj();
    read_ply();
}