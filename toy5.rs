struct Message {
    from: String,
    to: String,
    body: String,
}

struct Mailbox {
    owner: String,
    messages: Vec<Message>,
}

impl Mailbox {
    fn new(owner: &str) -> Self {
        Mailbox { owner: String::from(owner), messages: Vec::new() }
    }

    fn receive(&mut self, msg: Message) {
        println!("  ✉  {} 的消息存入 {} 信箱", msg.from, self.owner);
        self.messages.push(msg);
    }

    fn check(&self) {
        println!("📬 {} 的信箱里有 {} 条消息", self.owner, self.messages.len());
        for msg in &self.messages {
            println!("  {} → {}: {}", msg.from, msg.to, msg.body);
        }
    }

    fn take(&mut self) -> Vec<Message> {
        let taken: Vec<_> = self.messages.drain(..).collect();
        println!("📭 {} 取走了 {} 条消息", self.owner, taken.len());
        taken
    }
}

fn main() {
    let mut b_box = Mailbox::new("B");
    let mut c_box = Mailbox::new("C");

    // 没有 Runtime 在跑。消息直接放进信箱。
    b_box.receive(Message {
        from: String::from("A"), to: String::from("B"),
        body: String::from("你有空吗？"),
    });
    b_box.receive(Message {
        from: String::from("C"), to: String::from("B"),
        body: String::from("收到。谢谢 A。"),
    });
    c_box.receive(Message {
        from: String::from("A"), to: String::from("C"),
        body: String::from("请转给 C"),
    });

    println!("--- 检查信箱 ---");
    b_box.check();
    c_box.check();

    println!("--- B 来取信 ---");
    b_box.take();

    println!("--- 取完之后 ---");
    b_box.check();
}
