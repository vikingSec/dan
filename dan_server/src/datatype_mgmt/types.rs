use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::{Deserialize, Serialize};
use crate::types as typebase;
use poem_openapi::Enum;
#[derive(ApiResponse)]
pub enum datatype_get {
    #[oai(status = 200)]
    Ok(Json<datatype_get_ok>),
    /// Returns when the BadRequest.
    #[oai(status = 400)]
    Err(Json<datatype_get_err>),
}

#[derive(Object, Serialize, Deserialize)]
pub struct datatype_get_ok {
    pub message : String,
    pub types : typebase::typelist
}

#[derive(Object)]
pub struct datatype_get_err {
    pub message: String
}


// TODO: Create a struct to hold the put request
#[derive(Object, Serialize, Deserialize)]
pub struct datatype_put_req {
    pub username: String
}


#[derive(ApiResponse)]
pub enum datatype_put {
    #[oai(status = 200)]
    Ok(Json<datatype_put_ok>),
    /// Returns when the BadRequest.
    #[oai(status = 400)]
    Err(Json<datatype_put_err>),
}

#[derive(Object, Serialize, Deserialize)]
pub struct datatype_put_ok {
    pub message : String,
}

#[derive(Object)]
pub struct datatype_put_err {
    pub message: String
}
