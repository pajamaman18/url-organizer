use serde::{Deserialize, Serialize};

use crate::url_struct::UrlData;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UrlPool {
    urls: Vec<UrlData>
}



impl UrlPool {
    pub fn get_all_urls(&self) -> Vec<&UrlData> {
        self.urls.iter().collect()
    }

    pub fn get_url_urls(&self, url_name: &str)-> Vec<&UrlData> {
        self.urls.iter().filter(|&u| u.has_url(url_name)).collect::<Vec<&UrlData>>()
    }

    pub fn get_name_urls(&self, name: &str)-> Vec<&UrlData> {
        self.urls.iter().filter(|&u| u.has_name(name)).collect::<Vec<&UrlData>>()
    }

    pub fn get_tag_urls(&self, tag_name: &str)-> Vec<&UrlData> {
        self.urls.iter().filter(|&u| u.contains_tag(tag_name)).collect::<Vec<&UrlData>>()
    }

    pub(crate) fn add_urls(&mut self, new_urls: &mut Vec<UrlData>){
        self.urls.append(new_urls)
    }

    fn create_direct_copy(url_list: Vec<UrlData>) -> UrlPool {
        let sublist_copy: Vec<UrlData> = url_list.iter().map(|u| u.clone()).collect();
        Self{
            urls: sublist_copy
        }
    }

    fn create_copy_from_reference(url_list: Vec<&UrlData>) -> UrlPool {
        let sublist_copy: Vec<UrlData> = url_list.iter().map(|&u| u.clone()).collect();
        Self{
            urls: sublist_copy
        }
    }

    pub fn new() -> Self {
        Self{
            urls: Vec::new()
        }
    }

}

impl PartialEq<Self> for UrlPool {
    fn eq(&self, other: &Self) -> bool {
        for i in 1..self.urls.len(){
            if i >= other.urls.len() || self.urls[i.clone()] != other.urls[i]{
                return false
            }
        }
        return true
    }
}


#[cfg(test)]
mod tests {
    use crate::url_struct;

    use super::*;

    fn setup() -> UrlPool {
        let mut pool = UrlPool::new();
        let mut url_list = Vec::new();
        url_struct::add_url_data("a", "a.com", "first half", &mut url_list);
        url_struct::add_url_data("b", "b.com", "first half", &mut url_list);
        url_struct::add_url_data("z", "z.com", "second half", &mut url_list);
        pool.add_urls(&mut url_list);
        pool
    }

    #[test]
    fn test_get_url_urls() {
        let url_pool = setup();
        let subset = url_pool.get_url_urls(".com");
        assert_eq!(url_pool.get_all_urls(), subset);
        let subset = url_pool.get_url_urls("a.com");
        assert_eq!(UrlPool::create_direct_copy(url_pool.urls[..1].to_vec()).get_all_urls(), subset);
        let subset = url_pool.get_url_urls("z.com");
        assert_eq!(UrlPool::create_direct_copy(url_pool.urls[2..].to_vec()).get_all_urls(), subset);
    }

    #[test]
    fn test_get_name_urls() {
        let url_pool = setup();
        let subset = url_pool.get_name_urls(".com");
        assert_eq!(UrlPool::new().get_all_urls(), subset);
        let subset = url_pool.get_name_urls("");
        assert_eq!(url_pool.get_all_urls(), subset);
        let subset = url_pool.get_name_urls("a");
        assert_eq!(UrlPool::create_direct_copy(url_pool.urls[..1].to_vec()).get_all_urls(), subset);
        let subset = url_pool.get_name_urls("z");
        assert_eq!(UrlPool::create_direct_copy(url_pool.urls[2..].to_vec()).get_all_urls(), subset);
    }

    #[test]
    fn test_get_tag_urls() {
        let url_pool = setup();
        let subset = url_pool.get_tag_urls(".com");
        assert_eq!(UrlPool::create_direct_copy(vec![]).get_all_urls(), subset);
        let subset = url_pool.get_tag_urls("first half");
        assert_eq!(UrlPool::create_direct_copy(url_pool.urls[..2].to_vec()).get_all_urls(), subset);
        let subset = url_pool.get_tag_urls("second half");
        assert_eq!(UrlPool::create_direct_copy(url_pool.urls[2..].to_vec()).get_all_urls(), subset);
    }
}