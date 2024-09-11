use axum::{
    extract::State, 
    response::IntoResponse, 
    Json, 
    Router, 
    http::StatusCode, 
    routing::get
};
use crate::state::AppStateType;
use crate::order::Order;
use serde_json::json;
use log::error as cry;

/// Creates a router that handles order-related HTTP requests.
///
/// # Routes:
/// - `GET /order`: Retrieves the last order from the server's in-memory queue.
/// - `POST /order`: Accepts a new order and adds it to the server's in-memory queue.
///
/// This function sets up two routes: one for fetching the most recent order (GET),
/// and one for submitting a new order (POST). Orders are processed and saved to the database
/// if needed.
pub fn handle_order() -> Router<AppStateType> {
    
    /// Handles the `POST /order` route to accept a new order. The order is passed in as a JSON payload.
    ///
    /// # Parameters:
    /// - `state`: Shared application state (`AppStateType`) containing the in-memory queue and database client.
    /// - `order`: The new `Order` submitted by the client.
    ///
    /// # Returns:
    /// - `StatusCode::OK` with a success message if the order is added successfully.
    /// - `StatusCode::INTERNAL_SERVER_ERROR` if an error occurs while saving the order to the database.
    async fn send_order(State(state): State<AppStateType>, Json(order): Json<Order>) -> impl IntoResponse {
        match state.add_order(order).await {
            Ok(_) => (StatusCode::OK, "Order received!").into_response(),
            Err(e) => {
                cry!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save order to database").into_response()
            }
        }
    }

    /// Handles the `GET /order` route to fetch the last order from the in-memory queue.
    ///
    /// # Parameters:
    /// - `state`: Shared application state (`AppStateType`) containing the in-memory queue and database client.
    ///
    /// # Returns:
    /// - `StatusCode::OK` and a pretty-printed JSON representation of the last order, if one exists.
    /// - If no orders are available, a message indicating that no orders have been received yet.
    async fn get_order(State(state): State<AppStateType>) -> impl IntoResponse {
        let pretty = match state.get_last_order().await {
            Some(order) => serde_json::to_string_pretty(&order).unwrap(),
            None => serde_json::to_string_pretty(&json!({"message": "No orders yet"})).unwrap(),
        };
        (StatusCode::OK, pretty)
    }

    // Create the router with the defined routes
    Router::new()
        .route("/order", get(get_order).post(send_order))
}
