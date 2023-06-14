use crate::url_pool::UrlPool;

mod url_pool;
mod file_parsing;
mod url_struct;

pub fn main(){
    let all_urls:UrlPool = file_parsing::parse_files_into_data("src/url_files/");
    file_parsing::save_to_file("test-file", &all_urls).expect("saving to file errored");
    // println!("{:?}", read_from_parsed_file("test-file"))
    let parsed_urls = all_urls.get_url_urls("google");
    println!("{:?}", parsed_urls);

}
