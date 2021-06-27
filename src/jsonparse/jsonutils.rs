/*! # JSON utils
 * Function **calculate_name_from_path** will compute name from path according to name pool
 * Function **calculate_table_seq_list_from_all_table_path** will compute sequence variable for sub-table
 * Function **json_to_path_array** will go through whole JSON to get all full paths, including array path
 * Struct JSONUtils have the following members:
 
   - Method **load_data** to load JSON data into memory
   - Method **get_jdata_len** check length of JSON data since jdata is private
   - Method **gen_all_path** collect all possible valid path along with valid array, calling function **json_to_path_array**
   - Method **get_all_arr_len** check length of all array, the number of the total tables will be this number plus 1 (root table)
   - Method **get_all_path_len** check lenght of all valid JSON path. All valid columns in numbers of tables
   - Method **gen_map** create JSON format map based on all_array and all_path
   - Method **get_map_table_count** check number of tables since map is private
   - Method **export_json_map** export map into text file with JSON format
 */
use substring::Substring;
use std::io::Write;
 /** # JSON Utilities
 * Create JSON format map based on JSON data
 * Parse JSON data into several csv format tables
 */
#[derive(Debug, Clone)]
pub struct JSONUtils {
    jdata        : json::JsonValue,  // json data
    all_full_path: Vec<String>,      // list of full paths from jdata
    all_arr_path : Vec<String>,      // list of paths for array, which will be table in database
    map          : Option<json::JsonValue>,  // JSON format map based on jdata
}

impl JSONUtils{
    /** # load JSON data
     * Read JSON data file, load into self.jdata
     * Use **json::parse** method
     */
    pub fn load_data(path: String) -> Result<Self, &'static str>{
        let contents = std::fs::read_to_string(path).expect("Failed to read the file");
        let v: json::JsonValue = match json::parse(&contents){
            Ok(res) => res,
            Err(err) => {
                error!("In JSONUtils::load_data, json::parse(): {}", err);
                return Err("JSONUtils::load_data Failed!");
            }
        };
        let all_path = Vec::new();
        let all_arr = Vec::new();
        Ok(JSONUtils{
            jdata: v,
            all_full_path: all_path,
            all_arr_path: all_arr,
            map: None,
        })
    }
    /** # Get jdata length
     * return length of jdata
     * This function is needed since **JSONUtils.jdata** is private
     */
    pub fn get_jdata_len(self) -> usize{
        return self.jdata.to_string().len()
    }
    /** # analyze JSON data jdata, which is array of JSON
     * Call **json_map_loop** to get two lists
     * sort full path list descending
     * sort array path list descending
     * Remove full path for array
     */
    pub fn gen_all_path(self) -> Result<Self, &'static str>{
        let crt_path : String = "".to_owned();
        let mut cum_array: Vec<String> = Vec::new();
        let mut cum_path: Vec<String> = Vec::new();
        for val in self.jdata.members() {
            match json_to_path_array(val.clone(), crt_path.clone(), &mut cum_array, &mut cum_path){
                Ok(res) => res,
                Err(err) => {
                    error!("Error in json_to_path_array(): {}", err);
                    return Err("JSONUtils::get_all_path failed.")
                }
            };
        }
        // store path in reverse way
        cum_array.sort();
        cum_array.reverse();
        cum_path.sort();
        cum_path.reverse();
        // debug!("DEBUG length of cum_path: {}", cum_path.len());
        // remove elements from cum_path if element in cum_array
        cum_path.retain(|elm|!cum_array.contains(&elm));
        // debug!("DEBUG now length of cum_path: {}", cum_path.len());
        Ok(JSONUtils{
            jdata: self.jdata,
            all_full_path: cum_path,
            all_arr_path: cum_array,
            map: None,
        })
    }
    /** # Get all_arr_path length
     * return length of all_arr_path
     */
    pub fn get_all_arr_len(self) -> usize{
        return self.all_arr_path.len()
    }
    /** # Get all_full_path length
     * return length of all_full_path
     */
    pub fn get_all_path_len(self) -> usize{
        return self.all_full_path.len()
    }
    /** # Create map from all_full_path and all_arr_path
     * Get path list for one table
     * Based on path to compute column name
     * Compute sequence variable for sub-tables
     * Call private functions **calculate_name_from_path** and **calculate_table_seq_list_from_all_table_path**
     */
    pub fn gen_map(self) -> Result<Self, &'static str>{

        // all paths, remove paths belong to created table
        let mut all_path = self.all_full_path.clone();

        // all paths for table
        let all_arr_path = self.all_arr_path.clone();

        // used table name for duplication check
        let mut tablename_pool : Vec<String> = Vec::new();

        // table vector
        let mut vtbl = json::JsonValue::new_array();

        // loop for each sub table
        // rest will be into root table
        for i in 0..self.all_arr_path.len(){
            // compute table name
            let tblpath = all_arr_path[i].clone();
            let tbl = calculate_name_from_path(tblpath.clone(), tablename_pool.clone());
            tablename_pool.push(tbl.clone());

            // compute all path belong to this table
            let tblpathlist : Vec<String> = all_path.clone()
                                        .into_iter()
                                        .filter(|x| x.contains(&tblpath))
                                        .collect();
            // remove paths belong to this table from all_path
            all_path.retain(|elm|!tblpathlist.contains(&elm));
            
            // used column name of this table for duplication check
            let mut columnname_pool : Vec<String> = Vec::new();

            // collection of columns in this table
            let mut columns = json::JsonValue::new_array();

            // loop for each column of this table
            for j in 0..tblpathlist.len() {
                // compute column name of this table
                let clmpath = tblpathlist[j].clone();
                let clm = calculate_name_from_path(clmpath.clone(), columnname_pool.clone());
                columnname_pool.push(clm.clone());
                // push to collection for this table
                // let relpath = clmpath.substring(tblpath.len()+1, clmpath.len());
                // debug!("{}({}) + {} = {} ({})", tblpath.clone(), tblpath.len(), relpath.clone(), clmpath.clone(), clmpath.len());
                let elm = json::object!{
                    "columnName": clm,
                    "fullPath": clmpath.clone(),
                    "relativePath": clmpath.substring(tblpath.len(), clmpath.len()), 
                };
                columns.push(elm).unwrap();
            }
            // debug!("The table '{}' has {} real columns.", tbl, columns.len());
            if columns.len() > 0 {
                // Since all these tables are sub-table, need to compute sequence variable
                let all_arr_path = self.all_arr_path.clone();
                let seqs : json::JsonValue = calculate_table_seq_list_from_all_table_path(tblpath.clone(), all_arr_path);
                let jtbl = json::object!{
                    "tableName": tbl,
                    "arrayPath": tblpath,
                    "seqList": seqs,
                    "columnList": columns,
                };
                vtbl.push(jtbl).unwrap();
            }
        }
        // rest of path will be in root table, similar to above
        // Do not need sequence variable since it is root table
        let tbl = "root".to_owned();
        let mut columnname_pool : Vec<String> = Vec::new();
        let mut columns = json::JsonValue::new_array();
        let tblpathlist = all_path;
        for j in 0..tblpathlist.len() {
            let clmpath = tblpathlist[j].clone();
            let clm = calculate_name_from_path(clmpath.clone(), columnname_pool.clone());
            columnname_pool.push(clm.clone());
            // push to collection for this table
            let elm = json::object!{
                "columnName": clm,
                "fullPath": clmpath.clone(),
                "relativePath": clmpath, 
            };
            columns.push(elm).unwrap();
        }
        let jtbl = json::object!{
            "tableName": tbl,
            "arrayPath": "".to_owned(),
            "columnList": columns,
        };
        vtbl.push(jtbl).unwrap();
        // return back
        Ok(JSONUtils{
            jdata: self.jdata,
            all_full_path: self.all_full_path,
            all_arr_path: self.all_arr_path,
            map: Some(json::object!(
                "tableList": vtbl
            )),
        })
    }
    /** # Get map table number
     * return table number in JSON map
     */
    pub fn get_map_table_count(self) -> usize{
        let map = match self.map {
            None => json::object!("tableList": []),
            Some(x) => x,
        };
        // debug!("{}", map.clone().dump());
        let tblarr = &map["tableList"];
        // debug!("{}", tblarr.clone().dump());
        return tblarr.len();
    }
    /** # export JSON map
     * into JSON format text file
     */
    pub fn export_json_map(self, path: String) -> Result<(), &'static str>{
        if self.clone().get_map_table_count() == 0 {
            error!("No map export, please create import map, or load dat to generate map first.");
            return Err("Failed JsonUtils::export_json_map.");
        }
        let map = match self.map {
            None => json::object!("tableList": []),
            Some(x) => x,
        };
        let str_map = format!("{:#}", map);

        let mut ofile = std::fs::File::create(&path)
                       .expect("unable to create file");
        ofile.write_all(str_map.as_bytes()).expect("unable to write");
        Ok(())
    }
}

/** # compute name from path according to namepool
 * String of path have separator '/'
 * The last field will be the name
 * If name already exists in name pool, create new one by appending uuid (random number)
 */
#[allow(dead_code)]
fn calculate_name_from_path(path: String,
                            namepool: Vec<String>
                           )
     -> String
{
    let v: Vec<&str> = path.split('/').collect();
    let mut name = v.last().unwrap().to_lowercase();
    while namepool.contains(&name){
        name = format!("{}_{}",
            v.last().unwrap().to_lowercase(),
            uuid::Uuid::new_v4().to_string(),
        );
    }
    return name;
}

/** # compute table sequence list from table path according to all table pathes
 * If this table path contains one of all table pathes, this table is parent table
 * Need sequence number upon this parent table
 */
#[allow(dead_code)]
fn calculate_table_seq_list_from_all_table_path(tablepath: String,
                                                alltablelist: Vec<String>,
    ) -> json::JsonValue {
    let mut v = json::JsonValue::new_array();
    for i in 0..alltablelist.len() {
        if tablepath.contains(&(alltablelist[i])) {
            let taglist : Vec<&str> = alltablelist[i].split('/').collect();
            let tag = taglist.last().unwrap();
            let elm = json::object!{
                "columnName": format!("seq_{}", tag),
                "arrayPath": alltablelist[i].clone()
            };
            v.push(elm).unwrap();
        }
    }
    return v;
}

/** # get JSON paths
 * Given JSON Value and current path, for each (key, value) pair
 * Add current path and key as the full path
 * Append the full path to cumulative path list if the full path is new in the cumulative path list
 * If value is object, call this function itself for the next level
 * When value is array
 * Append the full path to cumulative array list if the full path is new in the cumulative array list
 * Then loop the array
 * If element of array is object, call this function itself for the next level
 */
#[allow(dead_code)]
pub fn json_to_path_array(jv: json::JsonValue
    , crt_path: String
    , cum_array: &mut Vec<String>
    , cum_path: &mut Vec<String>
    ) -> Result<(), &'static str>{
        if jv.is_object() {
            for (key, value) in jv.entries() {
                let thispath = format!("{}/{}", crt_path.clone(), key);
                if value.is_string() | value.is_number() | value.is_boolean() { // ignore structural path
                    if ! cum_path.contains(&thispath) {
                        cum_path.push(thispath.clone());
                    }
                }
                else if value.is_array() {
                    json_to_path_array(value.to_owned(), thispath, cum_array, cum_path).unwrap();
                }
                else if value.is_object() {
                    json_to_path_array(value.to_owned(), thispath, cum_array, cum_path).unwrap();
                }
            }
        }
        else if jv.is_array() {
            if jv.members().len() > 0 { // ignore empty array
                if ! cum_array.contains(&crt_path) {
                    cum_array.push(crt_path.clone());
                }
                for jrec in jv.members() {
                    json_to_path_array(jrec.to_owned(), crt_path.clone(), cum_array, cum_path).unwrap();
                }
            }
        }
        Ok(())
    }