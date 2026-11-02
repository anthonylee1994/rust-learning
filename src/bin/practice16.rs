#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

fn find_user(users: &[User], name: &str) -> Option<User> {
    users.iter().find(|x| x.name == name).cloned()
}

fn main() {
    let user1 = User {
        name: "John".to_string(),
        age: 20,
    };
    let user2 = User {
        name: "Mary".to_string(),
        age: 20,
    };
    println!("{:?}", user1);
    println!("{:#?}", user2);

    // PartialEq: compare field by field. Same data means equal.
    let user1_copy = user1.clone();
    println!("user1 == user1_copy: {}", user1 == user1_copy);
    println!("user1 == user2: {}", user1 == user2);

    // `users` takes ownership of user1 and user2, so compare before this line.
    let users = [user1, user2];

    // if let: only care about the Some case, ignore None.
    if let Some(u) = find_user(&users, "John") {
        println!("Found: {:?}", u);
    }

    // map + unwrap_or: get the age, or fall back to 0 when not found.
    let found_age = find_user(&users, "John").map(|u| u.age).unwrap_or(0);
    let missing_age = find_user(&users, "Nobody").map(|u| u.age).unwrap_or(0);
    println!("John age: {}", found_age);
    println!("Nobody age: {}", missing_age);
}
