/*! check loading JSON data by rust
- use crate json
- use simple_logger
*/

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
//#![allow(unused_imports)]

extern crate log;
use log::{info, error, debug};
use simple_logger::SimpleLogger;

use substring::Substring; // substring function

fn main() {
    SimpleLogger::new().init().unwrap();
    
    info!("Start loading JSON data file...");
    let path = "/var/local/usersetting/gitdev/data/txn1.json".to_owned();
    let ju = jsonparse::jsonutils::JSONUtils::load_data(path).unwrap();
    info!("Json data length: {}", ju.clone().get_jdata_len());
    let ju = ju.clone().gen_all_path().unwrap();
    info!("Json data all array count: {}", ju.clone().get_all_arr_len());
    info!("Json data all path count: {}", ju.clone().get_all_path_len());
    /*
    let ju = ju.clone().gen_map().unwrap();
    info!("Json map with table number: {}", ju.clone().get_map_table_count());
    let path = "/var/local/usersetting/gitdev/data/rust_init.map".to_owned();
    ju.export_json_map(path.clone()).unwrap();
    info!("Map exported to {}", path);
    
    let path = "/var/local/usersetting/gitdev/data/rust_init.map".to_owned();
    let ju = ju.import_json_map(path).unwrap();
    info!("Json map with table number: {}", ju.clone().get_map_table_count());
    let csv_file = "/var/local/usersetting/gitdev/data/map_init.csv".to_owned();
    ju.export_map_csv(csv_file.clone()).unwrap();
    info!("Map exported to {}", csv_file);
    */
    let path = "/var/local/usersetting/gitdev/data/map_init.csv".to_owned();
    let ju = ju.import_map_csv(path).unwrap();
    info!("Json map with table number: {}", ju.clone().get_map_table_count());
    
    let json_file = "/var/local/usersetting/gitdev/data/rust_fromcsv.map".to_owned();
    ju.export_json_map(json_file.clone()).unwrap();
    info!("Map exported to {}", json_file);
    /*
    let path = "/var/local/usersetting/gitdev/data/t.csv".to_owned();
    let contents = std::fs::read_to_string(path).expect("Failed to read the file");
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.
        println!("{:?}", record);
    }
    */
}
