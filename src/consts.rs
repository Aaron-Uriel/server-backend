pub mod resquests {
    pub const Food: &str = "GET /food_list HTTPS/1.1";
    pub const Variants: &str = "GET /variants_list HTTPS/1.1";
    pub const Tables: &str = "GET /tables_table_list HTTPS/1.1";
}

pub mod returns {
    pub const Ok: &str = "HTTP/1.1 200 OK\r\n\r\n";
    pub const NotFound: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
}