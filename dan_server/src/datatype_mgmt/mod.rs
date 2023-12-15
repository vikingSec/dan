
use poem_openapi::{ OpenApi, OpenApiService,payload::PlainText };
pub struct datatype_mgmt;

#[OpenApi]
impl datatype_mgmt {
    // TODO: Create CRUD for data types
    // Right now what we want to do is be able to manipulate data types for nodes
    #[oai(path="/datatype/get" ,method="get")]
    async fn datatype_get(&self) -> PlainText<&'static str> {
        return PlainText("Hello!");
    }


}
