use std::fs;

use glob::glob;
use serde_json;

use crate::url_struct::UrlData;

mod url_struct;

fn main(){
    let parsed_urls :Vec<UrlData> = parse_files_into_data("src/url_files/".to_string());
    save_to_file("test-file", parsed_urls).expect("saving to file errored");
    println!("{:?}", read_from_parsed_file("test-file"))

}

fn save_to_file(filename: &str, url_data: Vec<UrlData>) -> std::io::Result<()> {
    let contents = serde_json::to_string(&url_data).unwrap();
    // println!("{:?}", contents);
    fs::write("src/parsed_data/".to_string() + filename, contents)
}

fn read_from_parsed_file(filename: &str) -> Vec<UrlData>{
    let byte_data = fs::read("src/parsed_data/".to_string() + filename).expect("file reading went wrong");
    let s = match String::from_utf8(byte_data){
        Ok(v) => v,
        Err(e) => panic!("not valid UTF-8 in file: {}", e)
    };
    let out = serde_json::from_str(&s).unwrap();
    return out;
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
fn parse_files_into_data(path: String) -> Vec<UrlData>{
    let mut url_data: Vec<UrlData> = Vec::new();
    let dir_path = glob(&*(path + "*")).expect("glob didn't find file");
    for glob_path in dir_path{
        match glob_path {
            Ok (path) => {
                let filename:String = path.clone().display().to_string().split('/').map(|s| s.to_string()).collect::<Vec<String>>().last().unwrap().to_string();
                let tag_name = &filename[0..filename.len()-4];
                // println!("{:?}", string_name);
                let data = fs::read(path).expect("should have been able to read file");

                match String::from_utf8(data) {
                    Ok(v) => {
                        if !v.is_empty() {
                            let split_strings :Vec<String> = v.split('\n').map(|s| s.to_string()).collect();
                            for i in (0..split_strings.len()).step_by(2) {
                                url_struct::add_url_data(split_strings[i].clone(), split_strings[&i + 1].clone(), tag_name.to_string(), &mut url_data)
                            }
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
    return url_data;
}

