mod server;

fn main() {
    let mut HttpServer = server::services::http_server::HttpServer {
        ip: String::from("0.0.0.0")
    };
    

    HttpServer.start();

    println!("Hello, world!");
}
