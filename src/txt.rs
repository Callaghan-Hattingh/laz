use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Trajectory {
    pub time: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub qw: f64,
    pub qx: f64,
    pub qy: f64,
    pub qz: f64,
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

pub fn read_traj_txt_data(file_path: &str) -> io::Result<Vec<Trajectory>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut trajectories = Vec::new();
    let mut traj_old: Option<Trajectory> = None;

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

            if line_number > 2 && traj_old.is_some() {
                assert!(traj.time >= traj_old.unwrap().time);
            }

            trajectories.push(traj.clone());

            traj_old = Some(traj);
        }
    }

    Ok(trajectories)

}
