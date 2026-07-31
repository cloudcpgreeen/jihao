struct Message {
    from: String,
    to: String,
    body: String,
    action: Option<String>,
}

fn handle(msg: &Message) {
    match &msg.action {
        Some(cmd) => println!("B 执行了: {}", cmd),
        None => println!("B 收到了: {}", msg.body),
    }
}

fn main() {
    let msg = Message {
        from: String::from("A"),
        to: String::from("B"),
        body: String::from("飞起来"),
        action: Some(String::from("fly")),
    };

    handle(&msg);
}
