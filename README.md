# Overview

`solana-moralis-example` a minimal, production-ready example showcasing how to integrate Solana blockchain with Moralis API to fetch token prices, wallet balances, and perform real-time on-chain interactions. This project demonstrates best practices for using Moralis APIs with Solana.

## Features


- **Actix-Web** for building RESTful APIs
- **SQLx** for asynchronous PostgreSQL access
- **CQRS** pattern with separated command and query handlers
- **Docker + Docker Compose** for containerized development
- **GitHub Actions CI** with offline SQLx query validation and test runner


## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/patrikduch/solana-moralis-example.git
cd solana-moralis-example
```

### 2. Set Up Environment Variables

Copy the example `.env.example` file and configure it:

```sh
cp .env.example .env
```

Modify the `.env` file with your database credentials and other configurations.


### 3. Build and Run the Application

```sh
cargo build
cargo run
```


### 4. Dockerization

```sh
docker-compose up -d
```

### 5.📌 Example Usage – User API

#### ➕ Create a New User

```sh
curl -X POST http://localhost/api/users \
     -H "Content-Type: application/json" \
     -d '{"name": "Charlie", "email": "charlie@example.com"}'
```


#### 📥 Get All Users

```sh
curl http://localhost/api/users
```

#### 📄 Get User by ID

```sh
curl http://localhost/api/users/1
```

#### 🔄 Update a User

```sh
curl -X PUT http://localhost/api/users/1 \
     -H "Content-Type: application/json" \
     -d '{"name": "Updated Charlie", "email": "new-charlie@example.com"}'
```


### 6. Example Usage – Moralis Token Price API

#### Get SOL Price (USD)

Returns the current price of SOL in USD.

```sh
curl http://localhost/api/moralis/sol-price
```

Example response:

```json
{
  "usd_price": 152.45
}
```

---

#### Get Token Price by Address

Returns the current price of a token (by its Solana address) in USD.

```sh
curl http://localhost/api/moralis/token-price/<TOKEN_ADDRESS>
```

Replace `<TOKEN_ADDRESS>` with the actual token address. For example:

```sh
curl http://localhost/api/moralis/token-price/Es9vMFrzaCERoXcspv3z5kzpGbBCtF1PphhrybX9t9dX
```

Example response:

```json
{
  "usd_price": 1.00
}
```