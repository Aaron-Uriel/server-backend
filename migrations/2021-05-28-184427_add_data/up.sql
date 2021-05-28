INSERT INTO variants (v_name, v_group)
VALUES ("bistec", 1),
       ("chorizo", 1),
       ("pastor", 1),
       ("", 0);

INSERT INTO foods (f_name, price, estimated_time_minutes, variant_group)
VALUES ("Taco", 10, 5, 1),
       ("Quesadilla", 20, 10, 1),
       ("Volcán", 15, 7, 0);

INSERT INTO tables (table_id, is_occupied)
VALUES (1, FALSE),
       (2, FALSE),
       (3, FALSE),
       (4, FALSE),
       (5, FALSE),
       (6, FALSE);