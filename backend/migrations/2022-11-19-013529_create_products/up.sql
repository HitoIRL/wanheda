CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    price FLOAT4 NOT NULL,
    image VARCHAR NOT NULL
)