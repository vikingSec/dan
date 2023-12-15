use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
struct Api;
mod types;
mod datatype_mgmt;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        let fileloc = "./datatypes/datatypes.json";
        let readstr = std::fs::read_to_string(fileloc.to_string()).unwrap();
        let tf : types::typelist = serde_json::from_str(&readstr).unwrap(); 
        println!("{}",tf.types[0].name);

        PlainText("Hello World")
    }
}

#[tokio::main]
async fn main() {
    let api_service =
        OpenApiService::new((Api,datatype_mgmt::datatype_mgmt), "Hello World", "1.0").server("http://localhost:3001");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3001"))
        .run(app)
        .await;
}
