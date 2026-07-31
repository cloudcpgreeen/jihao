use std::sync::mpsc;
use std::thread;

struct Message {
    from: String,
    to: String,
    body: String,
}

struct Mailbox {
    owner: String,
    messages: Vec<Message>,
    notify: mpsc::SyncSender<String>,
}

impl Mailbox {
    fn new(owner: &str, notify: mpsc::SyncSender<String>) -> Self {
        Mailbox { owner: String::from(owner), messages: Vec::new(), notify }
    }

    fn receive(&mut self, msg: Message) {
        let notice = format!("{} 收到了来自 {} 的消息", msg.to, msg.from);
        self.messages.push(msg);
        // 门铃响了，即使 B 不在，通知也会留在缓冲区
        match self.notify.try_send(notice) {
            Ok(()) => println!("  🔔 门铃响了"),
            Err(mpsc::TrySendError::Full(_)) => println!("  ⚠️ 通知满了，但消息已存入信箱"),
            Err(mpsc::TrySendError::Disconnected(_)) => println!("  ⚠️ B 的门铃坏了"),
        }
    }

    fn check(&self, unread_notices: Vec<String>) {
        if !unread_notices.is_empty() {
            println!("📋 B 不在时的通知:");
            for n in &unread_notices { println!("  {}", n); }
        }
        for msg in &self.messages {
            println!("  ✉  {} → {}: {}", msg.from, msg.to, msg.body);
        }
    }
}

fn main() {
    // 缓冲门铃——最多存 10 条通知，B 不在也能留着
    let (tx, rx) = mpsc::sync_channel::<String>(10);

    let mut b_box = Mailbox::new("B", tx);

    // A 留言了。但 B 不在家——没有人在听。
    println!("A 留言给 B，但 B 不在家...");
    b_box.receive(Message {
        from: String::from("A"), to: String::from("B"),
        body: String::from("你有空吗？"),
    });
    b_box.receive(Message {
        from: String::from("C"), to: String::from("B"),
        body: String::from("我帮你接孩子。"),
    });

    println!("\n--- 过了一会儿，B 回来了 ---");

    // B 回来，先查门铃有没有响过
    let missed: Vec<_> = rx.try_iter().collect();
    b_box.check(missed);
}
