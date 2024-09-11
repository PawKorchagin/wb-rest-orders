use serde::{Serialize, Deserialize};

/// Represents the delivery details for an order.
///
/// This structure contains information related to the recipient's delivery address, 
/// contact information, and location details (such as the city and region).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Delivery {
    /// Name of the recipient.
    pub name: String,
    /// Phone number of the recipient.
    pub phone: String,
    /// Postal code of the recipient's address.
    pub zip: String,
    /// City of the recipient.
    pub city: String,
    /// Full address of the recipient.
    pub address: String,
    /// Region or state where the recipient is located.
    pub region: String,
    /// Email address of the recipient.
    pub email: String,
}

/// Represents payment details associated with an order.
///
/// This structure contains all information related to the payment for an order, 
/// including transaction ID, amount, payment date, and currency.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Payment {
    /// Unique transaction identifier.
    pub transaction: String,
    /// Request ID associated with the payment.
    pub request_id: String,
    /// Currency in which the payment was made.
    pub currency: String,
    /// Payment provider (e.g., Visa, MasterCard, PayPal).
    pub provider: String,
    /// Total amount paid for the order.
    pub amount: i32,
    /// Date and time of the payment (in Unix timestamp format).
    pub payment_dt: i64,
    /// Bank through which the payment was processed.
    pub bank: String,
    /// Cost of delivery for the order.
    pub delivery_cost: i32,
    /// Total cost of the goods in the order.
    pub goods_total: i32,
    /// Custom fee applied to the order, if applicable.
    pub custom_fee: i64,
}

/// Represents an item in an order.
///
/// This structure contains details for individual items included in an order, such as
/// the item's ID, price, and other related information.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Item {
    /// Unique identifier for the item (e.g., product code).
    pub chrt_id: i64,
    /// Tracking number for the item shipment.
    pub track_number: String,
    /// Price of the item.
    pub price: i32,
    /// RID (Retailer Identifier) for the item.
    pub rid: String,
    /// Name or description of the item.
    pub name: String,
    /// Discount or sale amount applied to the item.
    pub sale: i32,
    /// Size of the item (e.g., S, M, L).
    pub size: String,
    /// Total price for the item after applying discounts.
    pub total_price: i32,
    /// Unique NM (nomenclature) ID for the item.
    pub nm_id: i64,
    /// Brand of the item.
    pub brand: String,
    /// Status of the item (e.g., in stock, shipped, delivered).
    pub status: i64,
}

/// Represents an entire order, including delivery, payment, and item details.
///
/// The `Order` structure contains the full order information such as unique identifiers,
/// delivery and payment data, the list of items in the order, and other metadata.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Order {
    /// Unique identifier for the order.
    pub order_uid: String,
    /// Tracking number for the entire order.
    pub track_number: String,
    /// Entry point for the order (e.g., marketplace, online store).
    pub entry: String,
    /// Delivery details for the order (see `Delivery`).
    pub delivery: Delivery,
    /// Payment details for the order (see `Payment`).
    pub payment: Payment,
    /// List of items in the order (see `Item`).
    pub items: Vec<Item>,
    /// Locale for the order (e.g., en_US, fr_FR).
    pub locale: String,
    /// Internal signature or reference for the order.
    pub internal_signature: String,
    /// Unique customer identifier.
    pub customer_id: String,
    /// Delivery service used for the order (e.g., UPS, FedEx).
    pub delivery_service: String,
    /// Shard key used for distributed storage.
    pub shardkey: String,
    /// SM (Sales Manager) identifier associated with the order.
    pub sm_id: i32,
    /// Date and time when the order was created.
    pub date_created: String,
    /// Out of order shard key.
    pub oof_shard: String,
}
