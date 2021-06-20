use serde_json::Value;
use std::collections::HashMap;
type JsonMap = HashMap<String, serde_json::Value>;

/** # get JSON paths
 * Given JSON map and current path, for each (key, value) pair
 * Add current path and key as the full path
 * Append the full path to cumulative path list if the full path is new in the cumulative path list
 * If value is object, call this function itself for the next level
 * When value is array
 * Append the full path to cumulative array list if the full path is new in the cumulative array list
 * Then loop the array
 * If element of array is object, call this function itself for the next level
 */
#[allow(dead_code)]
pub fn json_map_loop(jm: JsonMap
    , crt_path: String
    , cum_array: &mut Vec<String>
    , cum_path: &mut Vec<String>
    ) -> Result<String, &'static str>{
        for (key, value) in jm.iter() {
            let thispath = format!("{}/{}", crt_path, key);
            if ! cum_path.contains(&thispath) {
                let path = thispath.clone();
                cum_path.push(path);
            }
            if value.is_array(){
                if ! cum_array.contains(&thispath) {
                    let path = thispath.clone();
                    cum_array.push(path);
                }
                // println!("The key '{}' has array '{}'.", key, value);
                let v: Vec<Value> = serde_json::from_value(value.clone()).unwrap();
                for val in v {
                    if val.is_array() {
                        println!("Value is array!!! '{}'", thispath);
                        return Err("Data format is wrong, should be object.");
                    }
                    else if val.is_object() {
                        // println!("Value is object!!! '{}'", val);
                        let v_map : JsonMap = serde_json::from_value(val).unwrap();
                        let crt_path = thispath.clone();
                        json_map_loop(v_map, crt_path, cum_array, cum_path).unwrap();
                    }
                    else {
                        println!("Value is '{}' at '{}'", val, thispath);
                        return Err("Data format is wrong, should be object.");
                    }
                }
            }
            else if value.is_object(){
                // println!("The key '{}' has object '{}'.", key, value);
                let v_map : JsonMap = serde_json::from_value(value.clone()).unwrap();
                let crt_path = thispath.clone();
                json_map_loop(v_map, crt_path, cum_array, cum_path).unwrap();
            }
            // else {
            //     println!("The key '{}' has value '{}'.", key, value);
            // }
        };
        return Ok("OK".to_owned())
    }