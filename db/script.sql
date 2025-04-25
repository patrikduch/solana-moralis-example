CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE crypto_prices (
  asset_name TEXT NOT NULL,
  price NUMERIC NOT NULL,
  ts TIMESTAMPTZ NOT NULL
);

SELECT create_hypertable('crypto_prices', 'ts');