struct Message {
    from: String,
    to: String,
    body: String,
}

fn deliver(msg: &Message) {
    println!("{} → {}: {}", msg.from, msg.to, msg.body);
}

fn main() {
    let messages = vec![
        Message { from: String::from("A"), to: String::from("B"), body: String::from("第一条") },
        Message { from: String::from("A"), to: String::from("B"), body: String::from("第二条") },
        Message { from: String::from("A"), to: String::from("B"), body: String::from("第三条") },
    ];
}
