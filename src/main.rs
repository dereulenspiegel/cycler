extern crate cyclerlib;

use cyclerlib::{calculate_cycle_infra};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <path to osm pbf file>", args[0]);
        return
    }
    
    let pbf_path = &args[1];
    println!("Parsing {}", pbf_path);

    let r = std::fs::File::open(&std::path::Path::new(pbf_path)).unwrap();
    match calculate_cycle_infra(r) {
        Ok(res) => {
            println!("Track length {} km", res.track_length);
            println!("Lane length {} km", res.lane_length);
            println!("Cycle street length {} km", res.cycle_street_length);
            println!("Street length {} km", res.total_street_length);
        },
        Err(e) => println!("Error: {:?}", e),
    }
    
}
