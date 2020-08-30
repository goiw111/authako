use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use openssl::sha;
use std::io::Read;

use crate::Authako;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Resource{
    name:       String,
    permission: u8,
}

impl Resource {
    pub fn new(name:    String, permission: u8)
        -> Resource {Resource {name,permission,}}
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Authorization {
    id:         u32,
    user_id:    u32,
    resources:  Vec<Resource>,
}

impl Authorization {

    pub fn new(id:  u32, user_id:   u32, resources:  Vec<Resource>)
        -> Authorization {
            Authorization {
                id,
                user_id,
                resources,
            }
    }

    pub fn get_resource(&self, name: &String) 
        -> Result<&Resource, &'static str> {
        for i in self.resources.iter() {
           if i.name.eq(name){
              return Ok(i);
           }
        }
        Err("you don't have authorization for that resource")
    }

    pub fn get_token(&self)
        -> [u8; 32] {
            let mut hasher = sha::Sha256::new();
             hasher.update(&serde_json::to_string(self).unwrap().as_bytes());
             let now = SystemTime::now();
             hasher.update(&now.duration_since(UNIX_EPOCH)
                 .expect("Time went backwards")
                 .as_secs_f64()
                 .to_le_bytes());
             hasher.finish()
    }

    pub fn default(authini: &Authako) -> Authorization {
        let mut data = String::new();
        let mut file = authini.authfile.as_ref().unwrap();
        file.read_to_string(&mut data).unwrap();
        let default: Vec<Resource>  = serde_json::from_str(&mut data).unwrap();
        Authorization {
            id:0,user_id:0,
            resources: default,
        }
    }
}
