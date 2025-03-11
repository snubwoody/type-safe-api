mod lang;
pub use lang::*;
use axum::{body::Body, extract::Request, response::Response};
use futures_util::future::BoxFuture;
use http::StatusCode;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct ValidationLayer{
	checksum: String
}

impl ValidationLayer{
	pub fn new(checksum:&str) -> Self{
		Self{
			checksum: String::from(checksum)
		}
	}
}

impl<S> Layer<S> for ValidationLayer{
	type Service = SchemaMiddleware<S>;
	fn layer(&self, inner: S) -> Self::Service {
		SchemaMiddleware{
			inner,
			checksum: self.checksum.clone()
		}
	}
}

#[derive(Clone)]
pub struct SchemaMiddleware<S>{
	inner: S,
	checksum: String
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
		match req.headers().get("Api-Schema-Checksum"){
			Some(value) => {
				let checksum = value.to_str().unwrap();
				if checksum != self.checksum{
					let response  = Response::builder()
						.status(StatusCode::UNPROCESSABLE_ENTITY)
						.body(Body::empty())
						.unwrap();

					return Box::pin(async move{Ok(response)});
				}
			},
			None => {
				let response  = Response::builder()
					.status(StatusCode::BAD_REQUEST)
					.body(Body::empty())
					.unwrap();

				return Box::pin(async move{Ok(response)});
			}
		}

		let future = self.inner.call(req);
		
		Box::pin(async move {
			let resposnse = future.await?;
			Ok(resposnse)
		})
	}
	
}

#[cfg(test)]
mod tests{
    use axum::{body::Body, routing::get, Router};
	use super::*;

	#[tokio::test]
	async fn api_validation(){

		let mut router = Router::new()
			.route("/", get(||async {"Hello world"}))
			.layer(ValidationLayer::new("1234abc"));

		let request = Request::builder()
			.header("Api-Schema-Checksum", "1234abc")
			.uri("/")
			.body(Body::empty())
			.unwrap();

		let response = router.call(request).await.unwrap();

		assert_eq!(response.status(),StatusCode::OK);
	}

	#[tokio::test]
	async fn invalid_checksum(){

		let mut router = Router::new()
			.route("/", get(||async {"Hello world"}))
			.layer(ValidationLayer::new("hi"));

		let request = Request::builder()
			.header("Api-Schema-Checksum", "kdkfs")
			.uri("/")
			.body(Body::empty())
			.unwrap();

		let response = router.call(request).await.unwrap();

		assert_eq!(response.status(),StatusCode::UNPROCESSABLE_ENTITY);
	}

	#[tokio::test]
	async fn missing_header(){
		let mut router = Router::new()
			.route("/", get(||async {"Hello world"}))
			.layer(ValidationLayer::new("hi"));

		let request = Request::builder()
			.uri("/")
			.body(Body::empty())
			.unwrap();

		let response = router.call(request).await.unwrap();

		assert_eq!(response.status(),StatusCode::BAD_REQUEST);
	}
}


