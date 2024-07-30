use rand::Rng as _;

pub fn get_random_user_email() -> (String, String) {
    let suffix = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>();
    let username = format!("loco{}", suffix);
    let email = format!("{username}@loco.com");
    (username, email)
}
