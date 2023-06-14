use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlData {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}


/// creates url object from given arguments and appends it to given vector, it will only update the tags list if the url is in the list
///
/// # Arguments
///
/// * `name`: the name of the url
/// * `url`: the url of the link
/// * `tag_name`: tag of the url
/// * `existing_urls`: list of url structs
///
/// returns: ()
///
/// # Examples
///
/// ```
/// let mut url_list = Vec::new();
/// add_url_data("google", "https://www.google.com", "search", url_list);
/// assert_eq!(url_list, [("google", "https://www.google.com", ["search"])])
/// ```
pub(crate) fn add_url_data(name: String, url: String, tag_name: String, existing_urls: &mut Vec<UrlData>) {
    let mut new_url_data = UrlData {
        name,
        url,
        tags: HashSet::new()
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
    pub(crate) fn add_tag(&mut self, tag: String) {
        self.tags.push(tag)
    }

    pub(crate) fn contains_tag(self, tag: &String) -> bool {
        self.tags.contains(tag)
    }

    pub(crate) fn has_url(&self, url: &String) -> bool {
        self.url.contains(url)
    }

    pub(crate) fn has_name(self, name: &String) -> bool {
        self.name.contains(name)
    }
}


impl PartialEq<Self> for UrlData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.url == other.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> UrlData{
        let mut url = UrlData {
            name: "google".to_string(),
            url: "www.google.com".to_string(),
            tags: HashSet::new(),
        };
        url.tags.insert("search".to_string());
        url
    }

    #[test]
    fn test_has_url(){
        let url = setup();
        assert!(url.clone().has_url(&"google".to_string()));
        assert!(url.clone().has_url(&"www".to_string()));
        assert!(url.clone().has_url(&"www.google.com".to_string()));
        assert!(!url.has_url(&"facebook".to_string()));
    }

    #[test]
    fn test_has_name(){
        let url = setup();
        assert!(url.clone().has_name(&"google".to_string()));
        assert!(!url.clone().has_name(&"www".to_string()));
        assert!(url.clone().has_name(&"go".to_string()));
        assert!(!url.has_name(&"facebook".to_string()));
    }

    #[test]
    fn test_contains_tag(){
        let mut url = setup();
        assert!(!url.clone().contains_tag(&"google".to_string()));
        assert!(!url.clone().contains_tag(&"www".to_string()));
        assert!(url.clone().contains_tag(&"search".to_string()));
        assert!(!url.clone().contains_tag(&"facebook".to_string()));
        url.add_tag("corpo".to_string());
        assert!(url.contains_tag(&"corpo".to_string()));
    }
}