use las::Reader;

fn main() {
    read_num_point_las();
}

fn read_num_point_las() {
    let path = std::env::args()
        .nth(1)
        .expect("Must provide a path to a las file");
    println!("{:?}", path);
    let mut reader = Reader::from_path(path).expect("Unable to open reader");
    let npoints = reader
        .points()
        .map(|p| p.expect("Unable to read point"))
        .count();
    println!("Number of points: {}", npoints);
}