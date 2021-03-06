CREATE TABLE IF NOT EXISTS clients (
    client_id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    table_id TINYINT UNSIGNED,
    arrival TIMESTAMP NOT NULL DEFAULT UTC_TIMESTAMP(),
    CONSTRAINT `table_id_fk`
        FOREIGN KEY (table_id) REFERENCES tables (table_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)