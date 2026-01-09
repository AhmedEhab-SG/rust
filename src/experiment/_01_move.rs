#[allow(unused_imports)]
use crate::Runable;

use super::Trial;

#[derive(Debug)]
struct User {
    full_name: String,
    email: String,
}

impl Trial {
    pub fn move_test() {
        let text = "test".to_string();

        let closure = |x: &String| println!("{}", x);

        closure(&text);

        let user_1 = User {
            full_name: "Ahmed Ehab".to_string(),
            email: "test@gmail.com".to_string(),
        };

        let user_2 = User {
            full_name: "Awww test".to_string(),
            email: "test1@gmail.com".to_string(),
        };

        let itr = vec![user_1, user_2];

        let is_user_found = itr
            .iter()
            .find(|u| u.full_name.contains("shmed") && u.email.contains("@"))
            .is_some();

        dbg!(is_user_found);
    }
}

impl Runable for Trial {
    fn run() {
        Self::move_test()
    }
}
