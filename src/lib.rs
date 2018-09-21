extern crate osmpbfreader;
extern crate haversine;

use osmpbfreader::{OsmPbfReader, OsmObj, Node, Way, OsmId, Tags, Relation};
use haversine::{distance, Location};

use std::collections::BTreeMap;
use std::convert::From;
use std::io::{Read, Seek};

const COORD_FACTOR : f64 = 10000000.0;

static STREET_TAG_VALS : [&str; 13]= ["motorway", "trunk", "pimary", "secondary", "tertiary", 
    "unclassified", "residential", "motorway_link", "trunk_link", "primary_link", "secondary_link",
    "tertiary_link", "road"];

/// Infrastructure statistics of the street infrastructure of an area. 
/// This provides the total length publicly accessible streets excluding 
/// "Spielstraßen" etc. The goal is the have the total length of streets where
/// cars have priority over other means of transportation.
/// If a street provides infrastructure for bicycles the length of this infrastructure is
/// added to these statistics.
pub struct InfraStats {
    pub track_length: f64,
    pub lane_length: f64,
    pub cycle_street_length: f64,

    pub total_street_length: f64,
}
/// This takes a parameter providing Read and Seek for OSM pbf file. It then selects
/// all relations describing streets. For each relations the length of every contained
/// Way is summarized. Additionally it is checked for every Way if there are tags describing 
/// the existence of bicycle infrastructure. If there are tags describing bicycle infrastructure
/// the length of the way is added to either track_length, lane_length or cycle_street_length.
pub fn calculate_cycle_infra<R>(r: R) -> Result<InfraStats, osmpbfreader::Error> where 
    R: Read + Seek {
    let mut pbf = OsmPbfReader::new(r);
    let mut result = InfraStats{
        track_length: 0.0,
        lane_length: 0.0,
        cycle_street_length: 0.0,
        total_street_length: 0.0,
    };

    let street_objs = pbf.get_objs_and_deps(is_street)?;

    for (_id, obj) in street_objs.iter() {
        if is_street(&obj) {
            match obj {
                OsmObj::Relation(r) => {
                    if r.tags.contains("type", "street") || r.tags.contains("type", "associatedStreet"){
                        calculate_relation_length(&street_objs, &r, &mut result);
                    }
                },
                _ => continue,
            }
        }
        
    }

    return Ok(result)
}

fn calculate_relation_length(obj_map: &BTreeMap<OsmId, OsmObj>, r: &Relation, result: &mut InfraStats) {

    for node_ref in r.refs.iter() {
        match obj_map.get(&node_ref.member) {
            Some(node) => {
                match node {
                    OsmObj::Way(w) => {
                        let l = calculate_way_length(obj_map, &w);
                        if has_street_tags(&w.tags){
                            result.total_street_length += l
                        }

                        if is_cycle_street(&w) {
                            result.cycle_street_length += l;
                        } else if is_cycle_lane(&w) {
                            result.lane_length += l;
                        } else if is_cycle_track(&w) {
                            result.track_length += l;
                        }
                    },
                    _ => continue,
                }
            },
            None => continue,
        }
    }
}

fn calculate_way_length(obj_map: &BTreeMap<OsmId, OsmObj>, w :&Way) -> f64 {
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

fn has_street_tags(tags: &Tags) -> bool {
    for val in STREET_TAG_VALS.iter() {
        if tags.contains("highway", val) {
            return true;
        }
    }
    return false;
}

fn is_street(obj: &OsmObj) -> bool {
    match obj {
        OsmObj::Relation(r) => {
            return r.tags.contains("type","street") || r.tags.contains("type","associatedStreet");
        },
        _ => false
    }
}

fn is_cycle_street(w: &Way) -> bool {
    return w.tags.contains("highway","cycleway") && w.tags.contains("bicycle_road", "yes")
}

fn is_cycle_track(w: &Way) -> bool {
    return w.tags.contains("cycleway", "track") || w.tags.contains("cycleway", "opposite_track")
}

fn is_cycle_lane(w: &Way) -> bool {
    return w.tags.contains("cycleway", "lane") || w.tags.contains("cycleway", "opposite_lane")
}