use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlData {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}


pub fn add_url_data(name: String, url: String, tag_name: String, existing_urls: &mut Vec<UrlData>) {
    let mut new_url_data = UrlData {
        name,
        url,
        tags: Vec::new()
    };
    for url in existing_urls.iter_mut() {
        if new_url_data == *url {
            url.add_tag(tag_name);
            return;
        }
    }
    new_url_data.add_tag(tag_name);
    existing_urls.push(new_url_data);

}

impl UrlData {
    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag)
    }
}


impl PartialEq<Self> for UrlData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.url == other.url
    }
}
