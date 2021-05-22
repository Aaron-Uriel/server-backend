table! {
    clients (client_id) {
        client_id -> Unsigned<Integer>,
        client_name -> Varchar,
        arrival -> Timestamp,
    }
}

table! {
    foods (food_id) {
        food_id -> Unsigned<Integer>,
        f_name -> Varchar,
        price -> Decimal,
        estimated_time_minutes -> Nullable<Unsigned<Tinyint>>,
        variant_group -> Unsigned<Integer>,
    }
}

table! {
    orders (order_id) {
        order_id -> Unsigned<Integer>,
        client_id -> Unsigned<Integer>,
        food_id -> Unsigned<Integer>,
        food_amount -> Unsigned<Integer>,
        variant_id -> Varchar,
        is_finished -> Bool,
    }
}

table! {
    variants (v_id) {
        v_id -> Varchar,
        v_name -> Varchar,
        v_group -> Unsigned<Integer>,
    }
}

table! {
    workers (worker_id) {
        worker_id -> Unsigned<Integer>,
        w_name -> Varchar,
        w_password -> Binary,
    }
}

joinable!(orders -> clients (client_id));
joinable!(orders -> foods (food_id));
joinable!(orders -> variants (variant_id));

allow_tables_to_appear_in_same_query!(
    clients,
    foods,
    orders,
    variants,
    workers,
);
