use crate::track_node::TrackNode;

pub struct RailSwitch {}

impl RailSwitch {
    pub fn new() -> Self {
        Self {}
    }
}

impl TrackNode for RailSwitch {}