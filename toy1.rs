struct Message {
    from: String,
    to: String,
    body: String,
}

fn main() {
    let msg = Message {
        from: String::from("A"),
        to: String::from("B"),
        body: String::from(""),
    };

    println!("{} → {}: {}", msg.from, msg.to, msg.body);
}
