use axum::{routing::get, Router};
use compiler::ValidationLayer;


#[tokio::main]
async fn main(){
	let router = Router::new()
		.route("/", get(||async {"Hello world"}))
		.layer(ValidationLayer::new("123abc"));
	
	println!("Starting sever on port 1000");
	let listener = tokio::net::TcpListener::bind("0.0.0.0:1000")
		.await
		.expect("Failed to start server");
	
	axum::serve(listener, router)
		.await
		.unwrap();
}