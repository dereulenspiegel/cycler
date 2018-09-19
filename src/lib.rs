extern crate osmpbfreader;
extern crate haversine;

use osmpbfreader::{OsmPbfReader, OsmObj, Node, Way, OsmId};
use haversine::{distance, Location};

use std::collections::BTreeMap;
use std::convert::From;

const COORD_FACTOR : f64 = 10000000.0;

static STREET_TAG_VALS : [&str; 13]= ["motorway", "trunk", "pimary", "secondary", "tertiary", 
    "unclassified", "residential", "motorway_link", "trunk_link", "primary_link", "secondary_link",
    "tertiary_link", "road"];

pub struct InfraStats {
    pub track_length: f64,
    pub lane_length: f64,

    pub street_length: f64,
}

pub fn calculate_cycle_infra(pbf_path : &str) -> Result<InfraStats, osmpbfreader::Error> {
    let r = std::fs::File::open(&std::path::Path::new(pbf_path))?;
    let mut pbf = OsmPbfReader::new(r);

    let mut track_length = 0_f64;
    let mut lane_length = 0_f64;
    let mut street_length = 0_f64;

    let mut cycle_objs = pbf.get_objs_and_deps(is_cycle_obj)?;
    pbf.rewind()?;

    for obj in pbf.iter().map(Result::unwrap) {
        match obj {
            OsmObj::Way(w) => {
                if is_cycle_lane(&w) {
                    let length = calculate_length(&mut cycle_objs, &w);
                    lane_length += length;
                } else if is_cycle_track(&w) {
                    let length = calculate_length(&mut cycle_objs, &w);
                    track_length += length;
                }
                
            },
            _ => continue,
        }
    }

    pbf.rewind()?;

    let mut street_objs = pbf.get_objs_and_deps(is_street)?;
    pbf.rewind()?;

    for obj in pbf.iter().map(Result::unwrap) {
        match obj {
            OsmObj::Way(w) => {
                if is_way_street(&w) {
                    let length = calculate_length(&mut street_objs, &w);
                    street_length += length;
                }
            },
            _=> continue,
        }
    }

    return Ok(InfraStats{
        track_length: track_length,
        lane_length: lane_length,
        street_length: street_length,
    })
}

fn calculate_length(obj_map: &mut BTreeMap<OsmId, OsmObj>, w :&Way) -> f64 {
    let mut length = 0_f64;
    let mut last_node : Option<&Node> = None;
    for node_id in &w.nodes {
        let obj = obj_map.get(&OsmId::from(*node_id));
        if let Some(ln) = last_node {

            if let Some(o) = obj {
                match o {
                    OsmObj::Node(n) => {
                        length += distance(Location{
                            latitude: (ln.decimicro_lat as f64) / COORD_FACTOR,
                            longitude: (ln.decimicro_lon as f64) / COORD_FACTOR,
                            },
                            Location{
                            latitude: (n.decimicro_lat as f64) / COORD_FACTOR,
                            longitude: (n.decimicro_lon  as f64) / COORD_FACTOR,
                            },
                            haversine::Units::Kilometers);
                            last_node = Some(n);
                    },
                    _ => continue,
                }
                
            }
            
            
        } else {
            if let Some(o) = obj {
                 match o {
                    OsmObj::Node(n) => last_node = Some(n),
                    _ => continue,
                }
            }
        }
    }
    return length;
}

fn is_way_street(w: &Way) -> bool {
    for val in STREET_TAG_VALS.iter() {
        if w.tags.contains("highway", val) {
            return true;
        }
    }
    return false;
}

fn is_street(obj: &OsmObj) -> bool {
    match obj {
        OsmObj::Way(w) => {
            return is_way_street(&w);
        },
        _ => false
    }
}

fn is_cycle_obj(obj: &OsmObj) -> bool {
    match obj {
        OsmObj::Way(w) => {
            return is_cycle_lane(&w) || is_cycle_track(&w);
        },
        _ => return false,
    }
}

fn is_cycle_track(w: &Way) -> bool {


    let mut highway_val: String = "".to_string();
    let mut bicycle_val: String = "".to_string();
    let mut oneway_val: String = "".to_string();
    let mut cycleway_val: String = "".to_string();

    for (key, element) in w.tags.iter() {
        if key.starts_with("cycleway") {
            cycleway_val = element.to_string();
            continue;
        }
        match key.as_ref() {
            "highway" => highway_val = element.to_string(),
            "bicycle" => bicycle_val = element.to_string(),
            "oneway" => oneway_val = element.to_string(),
            
            _=> continue,
        }
    }

    if highway_val == "cycleway" && !oneway_val.is_empty() {
        return true
    }

    if !highway_val.is_empty() {
        if bicycle_val == "use_sidepath" {
            return true
        }
        if cycleway_val == "track" {
            return true
        }
    }
    return false
}

fn is_cycle_lane(w: &Way) -> bool {
    let mut highway_val: String = "".to_string();
    let mut cycleway_val: String = "".to_string();

    for (key, element) in w.tags.iter() {
        if key.starts_with("cycleway") {
            cycleway_val = element.to_string();
            continue;
        }
        match key.as_ref() {
            "highway" => highway_val = element.to_string(),
            
            _=> continue,
        }
    }

    if !highway_val.is_empty() {
        if !cycleway_val.is_empty() {
            return true
        }
    }

    return false
}