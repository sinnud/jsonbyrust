/*! # JSON parse tool 
 * Module **jsonutils** works for the following:
   - load JSON data
   - go through data to collect all path and array information
   - generate JSON format map
*/
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
// log to file
#[macro_use]
extern crate log;


pub mod jsonutils;
// pub mod sqltrait;
// pub mod postgresql;
// pub mod file_status;
// pub mod libmysql;