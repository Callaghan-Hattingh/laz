use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
struct Trajectory {
    time: f64,
    x: f64,
    y: f64,
    z: f64,
    qw: f64,
    qx: f64,
    qy: f64,
    qz: f64,
}

impl Trajectory {
    fn from_line(line: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 8 {
            return Err(format!(
                "Expected 8 fields, but found {} in line: {}",
                parts.len(),
                line
            ));
        }

        let time = parts[0].parse::<f64>().map_err(|e| e.to_string())?;
        let x = parts[1].parse::<f64>().map_err(|e| e.to_string())?;
        let y = parts[2].parse::<f64>().map_err(|e| e.to_string())?;
        let z = parts[3].parse::<f64>().map_err(|e| e.to_string())?;
        let qw = parts[4].parse::<f64>().map_err(|e| e.to_string())?;
        let qx = parts[5].parse::<f64>().map_err(|e| e.to_string())?;
        let qy = parts[6].parse::<f64>().map_err(|e| e.to_string())?;
        let qz = parts[7].parse::<f64>().map_err(|e| e.to_string())?;

        Ok(Trajectory{
            time,
            x,
            y,
            z,
            qw,
            qx,
            qy,
            qz,
        })
    }
    
}

fn read_traj_txt_data(file_path: &str) -> io::Result<Vec<Trajectory>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut trajectories = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line_number > 1 {
            let traj = match Trajectory::from_line(line) { 
                Ok(trajectory) => trajectory,
                Err(e) => {
                    eprintln!("Failed to read TXT error: {}", e);
                    continue;
                }
            };
            trajectories.push(traj);
        }

    }

    Ok(trajectories)

}

fn main() {
    let file_path = "./map/global_georeferenced_wildcat_traj.txt";

    let traj = read_traj_txt_data(file_path);

    println!("There are {} trajectories in the file.", traj.unwrap().len());
}