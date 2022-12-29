use web_server;

fn main() {

web_server::new()
   .get("/", Box::new(|request: web_server::Request, mut response: web_server::Response|
        "Hello World!".into()))
   .launch(80)
   .unwrap();
}
