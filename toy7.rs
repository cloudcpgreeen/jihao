use std::io::{self, Write};

/// A toy that doesn't give you an answer.
/// It gives you back your own question.

struct Wonder {
    /// What the child noticed while playing
    noticed: String,
    /// What the child wonders about
    question: String,
}

fn main() -> io::Result<()> {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  TOY 7 — 没有下一块积木");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("这个玩具很简单。\n");
    println!("你刚才玩了六个玩具。");
    println!("有的让你说了一句话。");
    println!("有的让你发现了一个能力。");
    println!("有的让你删掉了一段代码。");
    println!("有的让你接起了两条通路。");
    println!("有的让你造了一个信箱。");
    println!("有的让你装了一个门铃。\n");

    println!("现在，不看任何指南——\n");
    println!("只问你自己两个问题：\n");

    // Question 1: What did you notice?
    print!("① 你玩的时候，哪个瞬间让你停下来了？\n> ");
    io::stdout().flush()?;
    let mut noticed = String::new();
    io::stdin().read_line(&mut noticed)?;

    // Question 2: What do you wonder?
    print!("\n② 关于那个瞬间，你最想问的是什么？\n> ");
    io::stdout().flush()?;
    let mut question = String::new();
    io::stdin().read_line(&mut question)?;

    let wonder = Wonder {
        noticed: noticed.trim().to_string(),
        question: question.trim().to_string(),
    };

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  你留下了这个：");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  停下来的地方: {}", wonder.noticed);
    println!("  你想问的: {}", wonder.question);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if wonder.question.is_empty() {
        println!("你没有问题。也没关系。");
        println!("空白本身也是一块积木。\n");
    } else {
        println!("这就是 TOY 8 的设计稿。");
        println!("不是我们替你写的。");
        println!("是你自己问出来的。\n");
    }

    println!("第七格不是空的。");
    println!("它装着你自己的问题。\n");
    println!("下一块积木，等你准备好了再来搭。\n");

    Ok(())
}
