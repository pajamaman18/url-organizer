pub(crate) use crate::url_struct;
use crate::url_struct::UrlData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlPool {
    urls: Vec<UrlData>
}



impl UrlPool {
    pub fn get_url_urls(self, url_name: &String) -> UrlPool {
        let filtered_urls: Vec<&UrlData> = self.urls.iter().filter(|&u| u.has_url(url_name)).collect::<Vec<&UrlData>>();
        Self::create_copy(filtered_urls)

    }

    pub(crate) fn add_urls(&mut self, &mut new_urls: Vec<UrlData>){
        self.urls.append(new_urls)
    }

    fn create_copy(url_list: Vec<UrlData>) -> UrlPool {
        Self{
            urls: url_list
        }
    }

    pub fn new() -> Self {
        Self{
            urls: Vec::new()
        }
    }

}