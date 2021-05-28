CREATE TABLE IF NOT EXISTS foods (
    food_id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT,
    f_name VARCHAR(25) NOT NULL,
    price DECIMAL(6, 2) NOT NULL,
    estimated_time_minutes TINYINT UNSIGNED DEFAULT NULL,
    has_variants BOOLEAN NOT NULL,
    PRIMARY KEY (food_id)
)