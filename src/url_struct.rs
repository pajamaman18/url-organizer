use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlData {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}


pub fn build_url_data(n: String, u: String, t: Vec<String>) -> UrlData {
    UrlData {
        name: n,
        url: u,
        tags: t,
    }
}

impl UrlData {
    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag)
    }

    fn calculate_url_hash<UrlData: Hash>(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }
}



impl Hash for UrlData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.url.hash(state);
    }
}

// impl AsRef<[u8]> for UrlData{
//     fn as_ref(&self) -> &[u8] {
//         let encoded = serde_json::to_string(&self).unwrap();
//         encoded.as_bytes()
//     }
// }
