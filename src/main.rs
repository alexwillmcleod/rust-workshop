use axum::{
  extract::State,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::{get, post},
  Json, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
enum CoffeeType {
  FlatWhite,
  Espresso,
  Frappe,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
enum Milk {
  Oat,
  Soy,
  Pea,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
struct Coffee {
  coffee_type: CoffeeType,
  milk_type: Option<Milk>,
  sugar_count: u8,
  is_decaf: bool,
}

trait Decaffeinatable {
  fn remove_caffeine(&mut self);
}

impl Decaffeinatable for Coffee {
  fn remove_caffeine(&mut self) {
    self.is_decaf = true;
  }
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  let app = Router::new()
    .route("/", get(root))
    .route("/order", post(order))
    .route("/orders", get(orders))
    .with_state(Arc::new(Mutex::new(Vec::<Coffee>::new())));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
  "Hello, World"
}

async fn orders(State(orders): State<Arc<Mutex<Vec<Coffee>>>>) -> Json<Vec<Coffee>> {
  let coffees = orders.lock().await.clone();
  Json(coffees)
}

async fn order(
  State(orders): State<Arc<Mutex<Vec<Coffee>>>>,
  Json(body): Json<Coffee>,
) -> StatusCode {
  orders.lock().await.push(body);
  StatusCode::CREATED
}
