use crate::{ply::Vertex, txt::Trajectory};

pub struct CheckTimestamps {
    pub initial_trajectory: Trajectory,
    pub final_trajectory: Trajectory,
    pub initial_vertex: Vertex,
    pub final_vertex: Vertex,
}

impl CheckTimestamps {
    pub fn new(
        initial_trajectory: Trajectory,
        final_trajectory: Trajectory,
        initial_vertex: Vertex,
        final_vertex: Vertex,
    ) -> Self {
        Self {
            initial_trajectory,
            final_trajectory,
            initial_vertex,
            final_vertex,
        }
    }

    pub fn validate_timestamp_range(&self) {
        assert!(
            self.initial_trajectory.time <= self.initial_vertex.time,
            // "Initial trajectory time ({}) is not less than initial vertex time ({}).",
            // self.initial_trajectory.time,
            // self.initial_vertex.time
        );
        assert!(
            self.final_trajectory.time >= self.final_vertex.time,
            // "Final trajectory time ({}) is not greater than final vertex time ({}).",
            // self.final_trajectory.time,
            // self.final_vertex.time
        );
    }
}
