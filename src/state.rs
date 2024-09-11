use tokio_postgres::{Client as PostgresClient, error::Error as PostgresError, NoTls};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::VecDeque;
use crate::order::Order;
use log::{debug, error as cry};

/// Application state shared across HTTP handlers, including the order queue and database client.
/// - `last_orders`: A runtime queue holding the most recent orders.
/// - `max_capacity`: Maximum size of the `last_orders` queue before flushing orders to the database.
/// - `db_client`: A database client for interacting with PostgreSQL.
pub struct AppState {
    last_orders: Mutex<VecDeque<Order>>,
    max_capacity: usize,
    db_client: Mutex<PostgresClient>,
}

/// A shared reference to `AppState`, wrapped in an `Arc` for safe concurrent access.
pub type AppStateType = Arc<AppState>;

impl AppState {
    /// Creates a new `AppState` instance with a given cache capacity and database connection parameters.
    /// Spawns a separate task to maintain the database connection.
    ///
    /// # Parameters
    /// - `capacity`: Maximum number of orders to store in memory before persisting to the database.
    /// - `host`: Database host address.
    /// - `username`: Username for connecting to the database.
    /// - `dbname`: The name of the database.
    /// - `password`: Password for the database connection.
    ///
    /// # Returns
    /// An instance of `AppState` with initialized database connection and empty order queue.
    pub async fn new(capacity: usize, host: &str, username: &str, dbname: &str, password: &str) -> Self {
        if capacity == 0 {
            panic!("Cache size can't be zero");
        }

        let connection_string = format!("host={host} user={username} dbname={dbname} password={password}");
        
        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
            .await
            .expect("Failed to connect to PostgreSQL");

        // Spawn a task to handle the database connection.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                cry!("Connection error: {}", e);
            }
        });

        AppState {
            last_orders: Mutex::new(VecDeque::new()),
            max_capacity: capacity,
            db_client: Mutex::new(client),
        }
    }

    /// Adds a new order to the in-memory queue. If the queue exceeds its maximum capacity, 
    /// orders will be persisted to the database.
    ///
    /// # Parameters
    /// - `last_order`: The `Order` to be added to the queue.
    ///
    /// # Returns
    /// `Ok(())` if the operation succeeds, or a `PostgresError` if a database error occurs.
    pub async fn add_order(&self, last_order: Order) -> Result<(), PostgresError> {
        let mut last_orders = self.last_orders.lock().await;

        debug!("There are {} orders in queue", last_orders.len());
        
        // If the queue reaches the maximum capacity, flush the orders to the database.
        if last_orders.len() >= self.max_capacity {
            debug!("Queue is full ({} orders). Flushing to the database.", self.max_capacity);
            let client = self.db_client.lock().await;
            while let Some(order) = last_orders.pop_front() {
                Self::save_to_db(&client, &order).await?;
            }
            debug!("Flushed all orders to the database.");
        }
        
        last_orders.push_back(last_order);
        Ok(())
    }

    /// Saves a given `Order` to the database, including related tables such as `deliveries`, `payments`, and `items`.
    ///
    /// # Parameters
    /// - `client`: A reference to the `PostgresClient` used for database operations.
    /// - `order`: The `Order` to be persisted.
    ///
    /// # Returns
    /// `Ok(0)` on success, or a `PostgresError` if a database operation fails.
    async fn save_to_db(client: &PostgresClient, order: &Order) -> Result<(), PostgresError> {
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
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
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

        Ok(())
    }

    /// Retrieves the most recent order from the in-memory queue.
    ///
    /// # Returns
    /// An `Option<Order>` containing the last order, or `None` if the queue is empty.
    pub async fn get_last_order(&self) -> Option<Order> {
        let last_orders = self.last_orders.lock().await;

        last_orders.back().cloned()
    }
}
