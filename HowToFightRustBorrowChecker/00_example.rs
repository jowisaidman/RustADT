struct User {
    pub user_id: i32,
    pub posts: Vec<String>,
}

impl User {
    fn set_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }
}

fn main() {
    let mut user = User {
        user_id: 123,
        posts: vec!["Hello World!".to_string(), "This is a second post".to_string()]
    };

    let user_posts = user.posts;
    user.user_id = 456; // This works
    user.set_id(789) // This will result in a error, borrow of partially moved value
}