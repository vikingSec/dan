
use poem_openapi::{ OpenApi, OpenApiService,payload::PlainText };

use crate::types::{typefile, typelist};

use self::types::{datatype_get, datatype_get_ok, datatype_get_err, datatype_put, datatype_put_ok, datatype_put_err};
pub struct datatype_mgmt;
mod types;


#[OpenApi]
impl datatype_mgmt {
    // TODO: Create CRUD for data types
    // Right now what we want to do is be able to manipulate data types for nodes
    #[oai(path="/datatype/get" ,method="get")]
    async fn datatype_get(&self) -> datatype_get {

        let fileloc = "./datatypes/datatypes.json";
        let readstr = std::fs::read_to_string(fileloc.to_string()).unwrap();
        let tf : typelist = serde_json::from_str(&readstr).unwrap(); 
 
        return datatype_get::Ok(poem_openapi::payload::Json(datatype_get_ok {
            message: "it's all good".to_string(),
            types: tf       
       })) 
    }

    // TODO: Create a POST for data types
    // Let's just add a new file and abstract the datatype read away from one file and into
    // multiple files
    #[oai(path="/datatype/put", method="post")]
    async fn datatype_post(&self) -> datatype_put {
        return datatype_put::Ok(poem_openapi::payload::Json(datatype_put_ok {
            message: "yeet".to_string()
        }))
    }


}
