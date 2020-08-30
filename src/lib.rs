use std::{fs::File, collections::HashMap};

pub use crate::permission::*;
pub use crate::authorization::*;

pub mod permission;
pub mod authorization;

pub struct Authako {
    authfile:   Option<File>,
    cachesys:   Option<HashMap<[u8; 32], Authorization>>,
}

impl  Authako {
    pub fn new() -> Authako {
        Authako {authfile: None, cachesys: None}
    }
    pub fn init (&mut self, filename: &str)
        -> std::io::Result<()> {
            let file = File::open(filename).unwrap();
            self.authfile = Some(file);
            self.cachesys = Some(HashMap::new());
            Ok(())
    }
    pub fn login(&mut self, loger: Authorization) 
        -> Option<[u8; 32]> {
            let token = loger.get_token();
            let hachm = self.cachesys.as_mut().unwrap();
            match hachm.get(&token) {
                Some(_) => None,
                None    => {hachm.insert(token,loger); Some(token)},
            }
    }
    pub fn logout(&mut self,token: &[u8; 32]) 
        -> Option<()> {
        let hachm = self.cachesys.as_mut().unwrap();
        match hachm.get(token) {
            Some(_v) => {
                match hachm.remove(token) {
                    Some(_v) => Some(()),
                    None    => None,
                }
            },
            None    => None,
        }
    }
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let _t = Authako::new();
		let  p = Permission::Create | Permission::Update;
		let  r1= Resource::new("useres",p);
		let  r2= Resource::new("useres",p);
		let _a = Authorization::new(0,0,vec![r1,r2]);
                let _s = match _a.get_resource("useres") {
                    Some(_p) => _p.have_right(p),
                    None     => false,
                };
                assert_eq!(_s,true);
	}
}
