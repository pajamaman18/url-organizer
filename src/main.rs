use std::fs;
use glob::glob;

fn main(){
    parse_files_into_data("src/url_files/".to_string());
}

fn parse_files_into_data(path: String) -> Vec<(String, String)> {
    let dir_path = glob(path.as_str() + "*").expect("glob didn't find file");
    let mut url_data = Vec::new();
    for glob_path in dir_path{
        match glob_path {
            Ok (path) => {
                let data = fs::read(path).expect("should have been able to read file");

                match String::from_utf8(data) {
                    Ok(v) => {
                        let split_strings :Vec<String> = v.split('\n').map(|s| s.to_string()).collect();
                        for i in (0..split_strings.len()).step_by(2){
                            url_data.push((split_strings[i.clone()].clone(), split_strings[i+1].clone()));
                        }
                    }
                    Err(e) => panic!("invalid string sequence {}", e)
                };
            }
            Err(e) => panic!("path borked: {:?}", e)
        }
    }
    println!("{:?}", url_data);
    return url_data;
}

