pub mod abc;

pub mod testing_world {
    use diesel::RunQueryDsl;
    use rocket::serde::json::Json;

    use crate::{blog_posts, BlogPost, Db};

    #[get("/")]
    pub fn index() -> &'static str {
        "Hello, world!"
    }

    #[get("/blog")]
    pub async fn get_all_blog_posts(connection: Db) -> Json<Vec<BlogPost>> {
        connection
            .run(|c| blog_posts::table.load(c))
            .await
            .map(Json)
            .expect("Failed to fetch blog posts")
    }
}
