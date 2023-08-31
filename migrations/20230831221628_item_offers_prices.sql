-- Add migration script here
CREATE TABLE IF NOT EXISTS item_offers_prices (
    offer_id BIGINT NOT NULL,
    FOREIGN KEY (offer_id) REFERENCES item_offers (id) ON DELETE CASCADE,
    currency_id BIGINT NOT NULL,
    FOREIGN KEY (currency_id) REFERENCES currencies (id) ON DELETE CASCADE,
    price BIGINT NOT NULL,

    PRIMARY KEY (offer_id, currency_id)
);
