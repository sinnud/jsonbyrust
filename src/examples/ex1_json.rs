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
    let ju = ju.clone().gen_map().unwrap();
    info!("Json map with table number: {}", ju.clone().get_map_table_count());
    let path = "/var/local/usersetting/gitdev/data/rust_init.map".to_owned();
    ju.export_json_map(path.clone()).unwrap();
    info!("Map exported to {}", path);
}
