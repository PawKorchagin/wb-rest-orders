CREATE TABLE IF NOT EXISTS orders(
   order_uid            VARCHAR NOT NULL PRIMARY KEY,
   track_number         VARCHAR,
   entry                VARCHAR,
   locale               VARCHAR,
   internal_signature   VARCHAR,
   customer_id          VARCHAR,
   delivery_service     VARCHAR,
   shardkey             VARCHAR, -- ?
   sm_id                BIGINT,
   date_created         VARCHAR, -- TODO TIMESTAMP
   oof_shard            VARCHAR
);

CREATE TABLE IF NOT EXISTS deliveries(
    order_uid   VARCHAR NOT NULL PRIMARY KEY,
    name        VARCHAR,
    phone       VARCHAR,
    zip         VARCHAR,
    city        VARCHAR,
    address     VARCHAR,
    region      VARCHAR,
    email       VARCHAR,
    FOREIGN KEY (order_uid) REFERENCES orders (order_uid)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS items(
    order_uid       VARCHAR NOT NULL,
    chrt_id         BIGINT,
    track_number    VARCHAR,
    price           BIGINT,
    rid             VARCHAR,
    name            VARCHAR,
    sale            BIGINT,
    i_size          VARCHAR,
    total_price     BIGINT,
    nm_id           BIGINT,
    brand           VARCHAR,
    status          BIGINT,
    FOREIGN KEY (order_uid) REFERENCES orders (order_uid)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS payments(
    transaction_id  VARCHAR NOT NULL PRIMARY KEY,
    request_id      VARCHAR,
    currency        VARCHAR,
    provider        VARCHAR,
    amount          BIGINT,
    payment_dt      BIGINT,
    bank            VARCHAR,
    delivery_cost   BIGINT,
    goods_total     BIGINT,
    custom_fee      BIGINT,
    FOREIGN KEY (transaction_id) REFERENCES orders (order_uid)
        ON DELETE CASCADE
);
