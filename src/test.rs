use axum::{routing::get, Router};

fn main() {
  let _app: Router = axum::router::Router::new().route("/foo", post(post_foo));
}

struct PostFooRequestBody {
  pub username: String
}

pub struct PostFooResponseBody {
  pub id: String
}

async fn post_foo(request_body: PostFooRequestBody) -> PostFooResponseBody {

}
