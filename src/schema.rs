table! {
    clients (client_id) {
        client_id -> Unsigned<Integer>,
        table_id -> Unsigned<Tinyint>,
        arrival -> Timestamp,
    }
}

table! {
    foods (food_id) {
        food_id -> Unsigned<Smallint>,
        f_name -> Varchar,
        price -> Decimal,
        estimated_time_minutes -> Nullable<Unsigned<Tinyint>>,
        variant_group -> Unsigned<Tinyint>,
    }
}

table! {
    orders (order_id) {
        order_id -> Unsigned<Integer>,
        client_id -> Unsigned<Integer>,
        food_id -> Unsigned<Smallint>,
        food_amount -> Unsigned<Tinyint>,
        variant_id -> Tinyint,
        is_finished -> Bool,
    }
}

table! {
    tables (table_id) {
        table_id -> Unsigned<Tinyint>,
        is_occupied -> Bool,
    }
}

table! {
    variants (v_id) {
        v_id -> Tinyint,
        v_name -> Varchar,
        v_group -> Unsigned<Tinyint>,
    }
}

joinable!(clients -> tables (table_id));
joinable!(orders -> clients (client_id));
joinable!(orders -> foods (food_id));
joinable!(orders -> variants (variant_id));

allow_tables_to_appear_in_same_query!(
    clients,
    foods,
    orders,
    tables,
    variants,
);
