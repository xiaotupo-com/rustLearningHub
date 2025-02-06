fn main() {
    let user = User::new(
        "xiaotupo".to_owned(),        //
        "xiaotupo@qd.com".to_owned(), //
    );

    println!("{:#?}", user);
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new(username: String, email: String) -> Self {
        Self {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}
