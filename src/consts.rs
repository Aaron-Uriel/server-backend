pub mod resquests {
    pub const Food: &str = "GET /food_list HTTP/1.1\r\n";
    pub const Variants: &str = "GET /variants_list HTTP/1.1\r\n";
    pub const Tables: &str = "GET /tables_table_list HTTP/1.1\r\n";
}

pub mod returns {
    pub const Ok: &str = "HTTP/1.1 200 OK";
    pub const NotFound: &str = "HTTP/1.1 404 NOT FOUND";
}

pub mod receives {
    pub const Order: &str = "";
}