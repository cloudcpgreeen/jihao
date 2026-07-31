struct Message {
    from: String,
    to: String,
    body: String,
    action: Option<String>,
}

fn forward(msg: &Message) -> Message {
    Message {
        from: msg.from.clone(),  // 保留原始发送者
        to: String::from("C"),
        body: format!("[B 转发自 {}] {}", msg.from, msg.body),
        action: None,
    }
}

fn reply(msg: &Message, answer: &str) -> Message {
    Message {
        from: msg.to.clone(),
        to: msg.from.clone(),  // 回复给原始发送者
        body: String::from(answer),
        action: None,
    }
}

fn main() {
    let original = Message {
        from: String::from("A"),
        to: String::from("B"),
        body: String::from("请转给 C"),
        action: Some(String::from("forward")),
    };

    let forwarded = forward(&original);
    println!("{} → {}: {}", forwarded.from, forwarded.to, forwarded.body);

    let answer = reply(&forwarded, "收到。谢谢 A。");
    println!("{} → {}: {}", answer.from, answer.to, answer.body);
}
