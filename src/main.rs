extern crate cyclerlib;

use cyclerlib::{calculate_cycle_infra};

fn main() {
    match calculate_cycle_infra("osm_data/Dortmund.osm.pbf") {
        Ok(res) => {
            println!("Track length {} km", res.track_length);
            println!("Lane length {} km", res.lane_length);
            println!("Street length {} km", res.street_length);
        },
        Err(e) => println!("Error: {:?}", e),
    }
    
}




