struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn print_field(user:User){
    println!("email:{}\nusername:{}\nactive:{}\nsign_in_count:{}",user.email,user.username,user.active,user.sign_in_count);
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    print_field(user1);
    print_field(user2);

}
