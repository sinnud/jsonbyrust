/*! check loading JSON data by rust

- use crate serde_json
- use simple_logger

*/

extern crate log;
use log::{info, error, debug};
use simple_logger::SimpleLogger;

use serde_json::Value;
use std::collections::HashMap;
type JsonMap = HashMap<String, serde_json::Value>;

fn main(){
    SimpleLogger::new().init().unwrap();

    let path = "/var/local/usersetting/gitdev/data/txn1.json".to_owned();
    let ju = myjson::jsonparse::JSONUtils::load_data(path).unwrap();
    info!("Data length: {}", ju.clone().get_jdata_len());
    let ju = ju.gen_all_path().unwrap();
    info!("All path length: {}", ju.clone().get_all_path_len());
    info!("Array length: {}", ju.clone().get_all_arr_len());

    let ju = ju.gen_map().unwrap();
    info!("All path length: {}", ju.clone().get_all_path_len());

    // let name = match jsonload(){
    //     Ok(res) => res,
    //     Err(err) => {
    //         error!("main with Error: {}", err);
    //         std::process::exit(1);
    //     }
    // };
    // info!("Result: {}", name);
}

#[allow(dead_code)]
fn jsonload()  -> Result<String, &'static str> {
    /*/ Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"[
        {
            "name": "John Doe",
            "age": 43,
            "School": {"name": "Potorla", "address": "100 Main St., Noland, Invisible"},
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        },
        {
            "name": "Luke Dule",
            "age": 44,
            "School": {"name": "Potorla", "address": "101 Main St., Noland2, Invisible2"}
        }]
        "#;

    // Parse the string of data into serde_json::Value.
    let v: Value = match serde_json::from_str(data){
        Ok(res) => res,
        Err(err) => {
            println!("In main, serde_json::from_str(): {}", err);
            return Err("main Failed!");
        }
    };
    */
    let path = "/var/local/usersetting/gitdev/data/txn8698.json";
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    debug!("Start loading JSON file: {}...", path);
    let v: Value = match serde_json::from_reader(reader){
        Ok(res) => res,
        Err(err) => {
            error!("In main, serde_json::from_file(): {}", err);
            return Err("main Failed!");
        }
    };
        
    debug!("Start analizing JSON data...");
    let crt_path : String = "".to_owned();
    let mut cum_array: Vec<String> = Vec::new();
    let mut cum_path: Vec<String> = Vec::new();
    if v.is_object() {
        info!("Value is object!!! '{}'", v);
        let v_map : JsonMap = serde_json::from_value(v.clone()).unwrap();
        match myjson::jsonparse::json_map_loop(v_map.clone(), crt_path, &mut cum_array, &mut cum_path){
            Ok(res) => res,
            Err(err) => {
                println!("Error in jsonload(): {}", err);
                return Err("See above.")
            }
        };
    }
    else if v.is_array(){
        let value: Vec<Value> = serde_json::from_value(v).unwrap();
        for val in value {
            let v_map : JsonMap = serde_json::from_value(val).unwrap();
            let cp = crt_path.clone();
            match myjson::jsonparse::json_map_loop(v_map, cp, &mut cum_array, &mut cum_path){
                Ok(res) => res,
                Err(err) => {
                    error!("Error in jsonload(): {}", err);
                    return Err("See above.")
                }
            };
        }
    }

    info!("After json_map_loop, we have {} paths and {} arrays", cum_path.len(), cum_array.len());
    for (i, p) in cum_array.iter().enumerate() {
        info!("The {}-th path is '{}'", i, p);
    }

    // Access parts of the data by indexing with square brackets.
    // println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    // let rst : String = serde_json::to_string(&v["name"]).unwrap();
    let rst : String = "OK".to_owned();

    Ok(rst)
}

#[cfg(test)]
mod tests {

    // lets pull our add_one function into scope
    // try move this definition to lib folder `ex`
    use super::*;

    // now let's pull in our lab tools into scope
    // to test our function
    use laboratory::{describe, expect, LabResult, NullState};

    /** # unit test function start from here
    // From Rust's perspective we will only define
    // one test, but inside this test we can define
    // however many tests we need.
    *
    #[test]
    fn suite() -> LabResult {

        // let's describe what our add_one() function will do.
        // The describe function takes a closure as its second
        // argument. And that closure also takes an argument which
        // we will call "suite". The argument is the suite's context
        // and it allows for extensive customizations. The context struct
        // comes with a method called it() and using this method we can
        // define a test.
        describe("jsonload()", |suite| {

            // when describing what it should do, feel free to be
            // as expressive as you would like.
            suite.it("should return 1 when passed 0", |_| {

                // here we will use the default expect function
                // that comes with laboratory.
                // We expect the result of add_one(0) to equal 1
                expect(jsonload().unwrap()).to_equal("John Doe".to_owned())

            })

            // just as a sanity check, let's add a second test
            .it("should return 2 when passed 1", |_| {

                expect(jsonload().unwrap()).to_equal("John Doe".to_owned())

            });

        }).state(NullState).milis().run()

    }
    */
    #[test]
    fn suite1() -> LabResult {
        describe("jsonload()", |suite1| {
            suite1.it("test json load", |_| {
                expect(jsonload()
                .unwrap()).to_equal("OK".to_owned())
            });
        }).state(NullState).milis().run()

    }
}