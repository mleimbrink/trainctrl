use crate::track_node::TrackNode;

pub struct RailStation {}

impl RailStation {
    pub fn new() -> Self {
        Self {}
    }
}

impl TrackNode for RailStation {}