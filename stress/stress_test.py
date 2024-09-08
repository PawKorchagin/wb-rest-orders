import asyncio
import aiohttp
import uuid
import time

# API endpoint
url = "http://localhost:3000/order"

# Define the POST request payload template (JSON data)
template_data = {
  "order_uid": "b563feb7b2b84a7test",
  "track_number": "WBILMTESTTRACK",
  "entry": "WBIL",
  "delivery": {
    "name": "Test Testov",
    "phone": "+9720000000",
    "zip": "2639809",
    "city": "Kiryat Mozkin",
    "address": "Ploshad Mira 15",
    "region": "Kraiot",
    "email": "test@gmail.com"
  },
  "payment": {
    "transaction": "b563feb7b2b84b6test",
    "request_id": "",
    "currency": "USD",
    "provider": "wbpay",
    "amount": 1817,
    "payment_dt": 1637907727,
    "bank": "alpha",
    "delivery_cost": 1500,
    "goods_total": 317,
    "custom_fee": 0
  },
  "items": [
    {
      "chrt_id": 9934930,
      "track_number": "WBILMTESTTRACK",
      "price": 453,
      "rid": "ab4219087a764ae0btest",
      "name": "Mascaras",
      "sale": 30,
      "size": "0",
      "total_price": 317,
      "nm_id": 2389212,
      "brand": "Vivienne Sabo",
      "status": 202
    }
  ],
  "locale": "en",
  "internal_signature": "",
  "customer_id": "test",
  "delivery_service": "meest",
  "shardkey": "9",
  "sm_id": 99,
  "date_created": "2021-11-26T06:22:19Z",
  "oof_shard": "1"
}

# Function to generate unique payload with different order_uid
def generate_unique_data():
    unique_order_uid = str(uuid.uuid4())  # Generate a unique order UID
    data = template_data.copy()
    data["payment"]["transaction"] = str(uuid.uuid4())
    data["order_uid"] = unique_order_uid
    return data

# Function to send a POST request
async def send_request(session, data):
    try:
        async with session.post(url, json=data) as response:
            if response.status != 200:
                print(f"Request failed with status code {response.status}")
            # else:
                # print(f"Request succeeded for order_uid: {data['order_uid']}")
    except Exception as e:
        print(f"Request failed: {e}")

# Function to run multiple requests in parallel
async def run_parallel_requests(num_requests):
    async with aiohttp.ClientSession() as session:
        tasks = []
        for _ in range(num_requests):
            data = generate_unique_data()
            task = asyncio.create_task(send_request(session, data))
            tasks.append(task)

        # Run all tasks concurrently
        await asyncio.gather(*tasks)

# Entry point to run the event loop
max_delta = None
def main():
    num_requests = 10000
    start_time = time.time()
    asyncio.run(run_parallel_requests(num_requests))
    end_time = time.time()
    print(f"Sent {num_requests} requests in {end_time - start_time:.2f} seconds")
    # if max_delta is None or max_delta < end_time - start_time:
        # max_delta = end_time - start_time

if __name__ == "__main__":
    main()
