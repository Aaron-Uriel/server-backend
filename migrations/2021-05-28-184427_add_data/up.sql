INSERT INTO variants (v_name)
VALUES ("Bistec"),
       ("Chorizo"),
       ("Pastor");

INSERT INTO foods (f_name, price, estimated_time_minutes, has_variants)
VALUES ("Taco", 10, 5, TRUE),
       ("Quesadilla", 20, 10, TRUE),
       ("Volcán", 15, 7, FALSE);

INSERT INTO tables (table_id, is_occupied)
VALUES (1, FALSE),
       (2, FALSE),
       (3, FALSE),
       (4, FALSE),
       (5, FALSE),
       (6, FALSE);