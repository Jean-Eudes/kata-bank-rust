CREATE TABLE bank_account
(
    id             SERIAL PRIMARY KEY,
    account_number VARCHAR(256) UNIQUE NOT NULL,
    initial_amount INTEGER             NOT NULL
);

CREATE TABLE transaction
(
    id              SERIAL PRIMARY KEY,
    type            VARCHAR(255)             NOT NULL,
    amount          INTEGER                  NOT NULL,
    date            TIMESTAMP WITH TIME ZONE NOT NULL,
    bank_account_id INT REFERENCES bank_account (id)
);

CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    login    VARCHAR(256) UNIQUE NOT NULL,
    password VARCHAR(256)        NOT NULL
);
