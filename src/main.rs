mod check_timestamps;
mod ply;
mod txt;

use check_timestamps::CheckTimestamps;
use ply::{create_new_ply_file, read_first_and_last_vertices, read_ply_header, Vertex};
use txt::{read_traj_txt_data, Trajectory};

fn read_traj() -> Option<Vec<Trajectory>> {
    let file_path_traj = "./map/global_georeferenced_wildcat_traj.txt";

    let traj = read_traj_txt_data(file_path_traj);

    Some(traj.unwrap())
}

fn read_first_and_last_ply() -> Option<(Vertex, Vertex)> {
    let file_path = "./map/global_georeferenced_wildcat.ply";

    Some(read_first_and_last_vertices(file_path).expect("Failed to read first and last vertices."))
}

fn create_new_ply(trajectories: Vec<Trajectory>) {
    let file_path = "./map/global_georeferenced_wildcat.ply";

    let (header, header_size) =
        read_ply_header(&file_path).expect("Error: read ply header failed.");

    let _ = create_new_ply_file(file_path, &header, header_size, trajectories);
}

fn main() {
    let traj = read_traj();
    let vert = read_first_and_last_ply();

    let i_t = traj.clone().unwrap().first().unwrap().clone();
    let f_t = traj.clone().unwrap().last().unwrap().clone();
    let i_v = vert.clone().unwrap().0;
    let f_v = vert.clone().unwrap().1;

    // validation
    let check = CheckTimestamps::new(i_t, f_t, i_v, f_v);
    check.validate_timestamp_range();

    create_new_ply(traj.unwrap())
}
