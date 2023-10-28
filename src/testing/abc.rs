pub mod file_a {
    #[get("/a")]
    pub fn file_a_func() -> &'static str {
        "This is A function"
    }
}
