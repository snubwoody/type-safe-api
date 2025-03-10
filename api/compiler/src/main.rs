use std::fs;
use axum::{extract::Request, middleware::Next, response::Response};
pub use code_generation::generate_code;
use core_types::ApiSchema;
use futures_util::future::BoxFuture;
use http::StatusCode;
use tower::{Layer, Service, ServiceBuilder};

mod api{
	use crate::generate_code;
	generate_code!("../schema.yml");
}

fn main() {
	let contents = fs::read_to_string("../schema.yml").unwrap();
    let schema: ApiSchema = serde_yaml::from_str(&contents).unwrap();
}

async fn schema_validation(request: Request,next: Next) -> Response{
	let checksum = request.headers().get("Api-Schema-Checksum");
	
	let response = next.run(request).await;
	
	response
}

pub struct ValidationLayer;

impl ValidationLayer{
	pub fn new() -> Self{
		Self
	}
}

impl<S> Layer<S> for ValidationLayer{
	type Service = SchemaMiddleware<S>;
	fn layer(&self, inner: S) -> Self::Service {
		SchemaMiddleware{
			inner
		}
	}
}


pub struct SchemaMiddleware<S>{
	inner: S
}

impl<S> Service<Request> for SchemaMiddleware<S>  
where 
	S: Service<Request,Response = Response> + Send + 'static,
	S::Future: Send + 'static
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = BoxFuture<'static,Result<Self::Response,Self::Error>>;

	fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, req: Request) -> Self::Future {
		println!("{:#?}",req);

		let future = self.inner.call(req);
		
		Box::pin(async move {
			let resposnse = future.await?;
			Ok(resposnse)
		})
	}
	
}


