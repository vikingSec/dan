use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct typefile {
    pub name : String 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct typelist {
   pub types : Vec<typefile> 
}
