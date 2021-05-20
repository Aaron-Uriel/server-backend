CREATE TABLE IF NOT EXISTS orders (
    order_id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    client_id INTEGER UNSIGNED NOT NULL,
    food_id INTEGER UNSIGNED NOT NULL,
    food_amount INTEGER UNSIGNED NOT NULL,
    variant_id VARCHAR(2) NOT NULL,
    is_finished BOOLEAN DEFAULT FALSE NOT NULL,
    CONSTRAINT `client_name_fk`
        FOREIGN KEY (client_id) REFERENCES clients (client_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT `food_id_fk`
        FOREIGN KEY (food_id) REFERENCES foods (food_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT `variant_id_fk`
        FOREIGN KEY (variant_id) REFERENCES variants (v_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)