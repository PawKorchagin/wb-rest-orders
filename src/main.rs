// use std::{io, iter::StepBy, sync::{Arc, Mutex}};

use std::sync::Arc;

use tokio::sync::Mutex;

use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse,
     routing::get, 
    Json, 
    Router
};

use serde::{
    Serialize,
    Deserialize,
};

use serde_json::json;

use tokio_postgres::{error::Error as PostgresError, Client as PostgresClient, GenericClient, NoTls
};

use log::{error as cry, debug, info};
use std::collections::VecDeque;

#[tokio::main]
async fn main() {
    init_logging();

    let state = Arc::new(AppState::new(5000).await);

    let app = Router::new()
        .merge(handle_order())
        .with_state(state);

    info!("Listening on ");

    axum_server::bind(std::net::SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap()
}

fn init_logging() {
    log4rs::init_file("src/resources/logging/log_cfg.yaml",
        Default::default()).unwrap()
}

// fn create_routers() -> Router {
//     Router::new().route("/order", get(get_order).post(handle_order))
// }

// #[derive(Default)]
struct AppState {
    last_orders: Mutex<VecDeque<Order>>,
    max_capacity: usize,
    db_client: Mutex<PostgresClient>
}

type AppStateType = Arc<AppState>;

impl AppState {
    async fn new(capacity: usize) -> Self {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost user=admin dbname=postgres password=root",
            NoTls,
        )
        .await
        .expect("Failed to connect to PostgreSQL");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                cry!("Connection error: {}", e);
            }
        });
        

        AppState {
            last_orders: Mutex::new(VecDeque::new()),
            max_capacity: capacity,
            db_client: Mutex::new(client)
        } 
    }

    async fn add_order(&self, last_order: Order) -> Result<(), PostgresError> {

        let mut last_orders = self.last_orders.lock().await;

        debug!("There are {} orders in queue", last_orders.len());
        
        if last_orders.len() >= self.max_capacity {
            debug!("there are {} orders in queue, max_capacity is: {}", last_orders.len(), self.max_capacity);
            let client = self.db_client.lock().await;
            while !last_orders.is_empty() {
                if let Some(order) = last_orders.pop_front() {
                    client
                        .execute(
                            "INSERT INTO orders (order_uid, track_number, entry, locale, internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard)
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                            &[
                                &order.order_uid, &order.track_number, &order.entry, &order.locale, &order.internal_signature, 
                                &order.customer_id, &order.delivery_service, &order.shardkey, &order.sm_id, 
                                &order.date_created, &order.oof_shard,
                            ],
                        )
                        .await?;
            
                    client
                        .execute(
                            "INSERT INTO deliveries (order_uid, name, phone, zip, city, address, region, email)
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                            &[
                                &order.order_uid, &order.delivery.name, &order.delivery.phone, &order.delivery.zip, 
                                &order.delivery.city, &order.delivery.address, &order.delivery.region, &order.delivery.email,
                            ],
                        )
                        .await?;
                
                    client
                        .execute(
                            "INSERT INTO payments (transaction_id, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                            ON CONFLICT(transaction_id) DO NOTHING",
                            &[
                                &order.payment.transaction, &order.payment.request_id, &order.payment.currency,
                                &order.payment.provider, &order.payment.amount, &order.payment.payment_dt, 
                                &order.payment.bank, &order.payment.delivery_cost, &order.payment.goods_total, 
                                &order.payment.custom_fee,
                            ],
                        )
                        .await?;
                
                    for item in &order.items {
                        client
                            .execute(
                                "INSERT INTO items (order_uid, chrt_id, track_number, price, rid, name, sale, i_size, total_price, nm_id, brand, status)
                                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
                                &[
                                    &order.order_uid, &item.chrt_id, &item.track_number, &item.price, 
                                    &item.rid, &item.name, &item.sale, &item.size, &item.total_price, 
                                    &item.nm_id, &item.brand, &item.status,
                                ],
                            )
                            .await?;
                    }
                } else {
                    cry!("last_orders.pop_front() returned None");
                }
                debug!("clearing orders ended, there are {} orders in queue", last_orders.len());
            }
        }
        
        last_orders.push_back(last_order);
        Ok(())
    }

    // async fn get_orders(&self) -> Vec<Order> {
    //     let last_orders = self.last_orders.read().await;
    //     last_orders.iter().cloned().collect()
    // }

    async fn get_last_order(&self) -> Option<Order> {
        let last_orders = self.last_orders.lock().await;

        last_orders.back().cloned()
    }
}

fn handle_order() -> Router<AppStateType> {
    async fn send_order(
        State(state): State<AppStateType>,
        Json(order): Json<Order>
    ) -> impl IntoResponse {
        match state.add_order(order).await {
            Ok(_) => (StatusCode::OK, "Order received!").into_response(),
            Err(e) => {
                cry!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save order to database").into_response()
            }
        }
    }

    async fn get_order(State(state): State<AppStateType>) -> impl IntoResponse {
        // let last_order = state.last_order.read().await;
        // match &*last_order {
        //     Some(order) => Html(format!("Last order: {:?}", order)),
        //     None => Html(format!("No order received yet"))
        // }

        let pretty = match state.get_last_order().await {
            Some(order) => serde_json::to_string_pretty(&order).unwrap(),
            None => serde_json::to_string_pretty(&json!({"message": "No orders yet"})).unwrap()
        };

        (StatusCode::OK, pretty)
        // Json(state.read().await.last_order.clone())
        // "kek".into_response()
    }
    
    Router::new()
        .route("/order",
        get(get_order)
        .post(send_order))
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i64,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Item {
    chrt_id: i64,
    track_number: String,
    price: i32,
    rid: String,
    name: String,
    sale: i32,
    size: String,
    total_price: i32,
    nm_id: i64,
    brand: String,
    status: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
}
