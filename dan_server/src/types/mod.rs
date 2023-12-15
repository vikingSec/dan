use serde::{Deserialize, Serialize};
use poem_openapi::Object;
#[derive(Object, Debug, Deserialize, Serialize)]
pub struct typefile {
    pub name : String 
}

#[derive(Object, Debug, Deserialize, Serialize)]
pub struct typelist {
   pub types : Vec<typefile> 
}
