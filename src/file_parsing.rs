use crate::url_struct::UrlData;
use std::fs;
use std::io::read_to_string;
use glob::glob;
use serde_json;
use crate::url_pool::UrlPool;
use crate::url_struct;


pub fn save_to_file(filename: &str, url_data: &UrlPool) -> std::io::Result<()> {
    let contents = serde_json::to_string(&url_data).unwrap();
    // println!("{:?}", contents);
    fs::write("src/parsed_data/".to_string() + filename + ".json", contents)
}

pub fn parse_random_file(filename: &str) -> Option<UrlPool> {
    let extension = filename.rsplit('.').last()?;
    match extension {
        "json" => {
          read_from_parsed_file(filename)
        },
        "txt" => {
            parse_files_into_data(filename)
        }
        _ => None
    }
}

pub fn read_from_parsed_file(filename: &str) -> Option<UrlPool>{
    let byte_data = fs::read("src/parsed_data/".to_string() + filename).expect("file reading went wrong");
    let s = match String::from_utf8(byte_data){
        Ok(v) => v,
        Err(e) => panic!("not valid UTF-8 in file: {}", e)
    };
    serde_json::from_str(&s).ok()
}

/// reads data from a file in the format:
///     url name
///     actual url
/// and turns it into a
///
/// # Arguments
///
/// * `path`: folder containing all .txt files with the formatted data
///
/// returns: Vec<UrlData, Global>
///
/// # Examples
///
/// ```
///
/// ```
pub fn parse_files_into_data(path: &str) -> Option<UrlPool>{
    let mut url_pool: UrlPool = UrlPool::new();
    // search for all files in folder
    let dir_path = glob(&*(path.to_string() + "*")).expect("glob didn't find file");
    // loop over every found file
    for glob_path in dir_path{
        match glob_path {
            Ok (path) => {
                // extract name of file
                let filename:String = path.clone().display().to_string().split('/').map(|s| s.to_string()).collect::<Vec<String>>().last().unwrap().to_string();
                let tag_name = &filename[0..filename.len()-4];
                // println!("{:?}", string_name);
                // read file
                let data = fs::read(path).expect("should have been able to read file");
                // turn it into string
                match String::from_utf8(data) {
                    Ok(v) => {
                        // if string is empty -> do nothing
                        if !v.is_empty() {
                            // split string on newlines
                            let split_strings :Vec<String> = v.split('\n').map(|s| s.to_string()).collect();
                            let mut parsed_urls: Vec<UrlData> = Vec::new();
                            // run through list with steps of 2 for both name and url
                            for i in (0..split_strings.len()).step_by(2) {
                                // add new url to list
                                url_struct::add_url_data(&split_strings[i].clone(), &split_strings[&i + 1].clone(), &tag_name.to_string(), &mut parsed_urls)
                            }
                            url_pool.add_urls(&mut parsed_urls)
                        }else{
                            println!("file on: {} is empty", filename);
                        }
                        // url_data.push((split_strings[i.clone()].clone(), split_strings[i+1].clone()));
                    }
                    Err(e) => panic!("invalid string sequence {}", e)
                };
            }
            Err(e) => panic!("path borked: {:?}", e)
        }
    }
    return Some(url_pool);
}