use crate::url_pool::UrlPool;

mod url_pool;
mod file_parsing;
mod url_struct;

pub fn main(){
    let parsed_urls :UrlPool = file_parsing::parse_files_into_data("src/url_files/".to_string());
    file_parsing::save_to_file("test-file", &parsed_urls).expect("saving to file errored");
    // println!("{:?}", read_from_parsed_file("test-file"))
    get_url(&"google".to_string(), &parsed_urls);
    println!("{:?}", parsed_urls);
}
