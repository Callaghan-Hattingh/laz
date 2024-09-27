mod check_timestamps;
mod ply;
mod txt;

use check_timestamps::CheckTimestamps;
use ply::{read_first_and_last_vertices, read_ply_header, Vertex};
use txt::{read_traj_txt_data, Trajectory};

fn read_traj() -> Option<Vec<Trajectory>>{
    let file_path_traj = "./map/global_georeferenced_wildcat_traj.txt";

    let traj = read_traj_txt_data(file_path_traj);

    Some(traj.unwrap())
}

fn read_first_and_last_ply() -> Option<(Vertex, Vertex)> {
    let file_path = "./map/global_georeferenced_wildcat.ply";

    Some(read_first_and_last_vertices(file_path).expect("Failed to read first and last vertices."))
}

fn main() {
    let traj = read_traj();
    let vert = read_first_and_last_ply();

    let i_t = traj.clone().unwrap().first().unwrap().clone();
    let f_t = traj.unwrap().last().unwrap().clone();
    let i_v = vert.clone().unwrap().0;
    let f_v = vert.unwrap().1;

    let check = CheckTimestamps::new(i_t, f_t, i_v, f_v);
    check.validate_timestamp_range();
    
}
