# Toybox — 一盒可以自己探索的玩具

欢迎。

这里没有说明书。

只有五个小玩具。

一个一个玩。按一下。看看会发生什么。

---

## TOY 1 — 让两个东西说一句话

### 玩一下

打开编辑器，新建 `toy1.rs`：

```rust
struct Message {
    from: String,
    to: String,
    body: String,
}

fn main() {
    let msg = Message {
        from: String::from("A"),
        to: String::from("B"),
        body: String::from("你好。"),
    };

    println!("{} → {}: {}", msg.from, msg.to, msg.body);
}
```

运行它：

```bash
rustc toy1.rs && ./toy1
```

```
A → B: 你好。
```

它说话了。

### 咦？

A 和 B 谁先说话，是 A 决定的吗？

### 试试看

把 `from` 和 `to` 对调一下。

再试一次：把 `body` 留空。

### 你发现了什么？

你刚才已经碰到了一个东西：

A 告诉 B 的那句话。

它有说的人、听的人、说的内容。三个缺一个，这话就出不去。

在协议里，它叫 **Message**。

不是定义。是你刚才用的那个东西的名字。

---

## TOY 2 — 给它一个能力

### 玩一下

B 收到了话。现在让 B **做一件事**。

新建 `toy2.rs`：

```rust
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
        body: String::from("转给 C"),
        action: Some(String::from("forward")),
    };

    handle(&msg);
}
```

```
B 执行了: forward
```

A 不只是说话。A 请求 B 做一件事。B 做了。

### 咦？

如果 A 请求一个 B 做不到的事呢？

### 试试看

把 `action` 改成 `"fly"`。B 能飞吗？

### 你发现了什么？

不是所有请求 B 都能做。

B 能做的、A 可以请求的，叫 **Capability**。

它不是你给的。它是 B 本来就有的能力，A 只是请求使用它。

---

## TOY 3 — 如果拿走 B 呢？

### 玩一下

前面两个玩具，B 一直都在。

现在把 B 拿走。

```rust
struct Message {
    from: String,
    to: String,
    body: String,
}

fn deliver(msg: &Message) {
    // B 在这里。对吗？一定在吗？
    println!("{} → {}: {}", msg.from, msg.to, msg.body);
}

fn main() {
    let messages = vec![
        Message { from: String::from("A"), to: String::from("B"), body: String::from("第一条") },
        Message { from: String::from("A"), to: String::from("B"), body: String::from("第二条") },
        Message { from: String::from("A"), to: String::from("B"), body: String::from("第三条") },
    ];

    // 处理消息。
    // 但是如果……没有人在处理呢？
    for msg in &messages {
        deliver(msg);
    }
}
```

运行。正常工作。

### 咦？

把 `for msg in &messages` 那三行删掉。

只留着 `let messages = vec![...];`。

运行。

```
（什么都没有）
```

消息还在。但是没有人处理了。

### 试试看

把那段 for 循环加回去。但是放到一个不会被执行到的地方。

或者：

什么都不改。只问自己一个问题：

> 刚才的 `for` 循环，是谁在跑它？

### 你发现了什么？

消息本身不会自己跑。

需要有一个人 — 或者一个东西 — 一直在那里，一条一条地读消息，一条一条地送出去。

在协议里，它叫 **Runtime**。

Runtime 就是那个一直在那里、反复做同一件事的执行循环。

你没有写它的时候，消息就躺在那里，哪也去不了。

不是 Bug。是 **少了什么东西**。

---

## TOY 4 — 把两个能力接在一起

### 玩一下

B 能转发。C 能回复。

把它们接起来。

```rust
struct Message {
    from: String,
    to: String,
    body: String,
    action: Option<String>,
}

fn forward(msg: &Message) -> Message {
    Message {
        from: String::from("B"),
        to: String::from("C"),
        body: format!("[转发自 {}] {}", msg.from, msg.body),
        action: None,
    }
}

fn reply(msg: &Message, answer: &str) -> Message {
    Message {
        from: msg.to.clone(),
        to: msg.from.clone(),
        body: String::from(answer),
        action: None,
    }
}

fn main() {
    let msg = Message {
        from: String::from("A"),
        to: String::from("B"),
        body: String::from("请转给 C"),
        action: Some(String::from("forward")),
    };

    let forwarded = forward(&msg);
    println!("{} → {}: {}", forwarded.from, forwarded.to, forwarded.body);

    let answer = reply(&forwarded, "收到。谢谢 A。");
    println!("{} → {}: {}", answer.from, answer.to, answer.body);
}
```

```
B → C: [转发自 A] 请转给 C
C → B: 收到。谢谢 A。
```

### 咦？

C 回复给了 B。但如果 A 想知道 C 收到了吗 — A 能知道吗？

### 试试看

让 C 直接回复 A。不经过 B。

### 你发现了什么？

能力可以接在一起。但它们怎么接，决定了消息能走多远。

当两个能力接在一起，就不再是孤立的动作。它们变成了一条通路。

在协议里，这叫 **Composition** — 把能力接成管道。

---

## TOY 5 — 你自己造一个

现在你已经有了：

- Message — 一句话
- Capability — 能做的一件事
- Runtime — 一直跑着的那个循环
- Composition — 把能力接起来

现在轮到你了。

### 你的玩具

造一个东西。任何东西。只要满足：

1. 两个以上的东西互相说话
2. 至少有一个能做某事
3. 有一个一直跑着的循环
4. 把两个能力接在一起

可以是：

- 一个签到机器人
- 一个自动回复机
- 一个消息中转站
- 一个你想到的其他东西

你不需要参考任何文档。

你只需要你刚才已经玩过的四个玩具。

---

## 现在回头看

你刚才玩过的东西：

```
TOY 1 — 一句话               Message
TOY 2 — 一个能力             Capability
TOY 3 — 一直跑着的循环       Runtime
TOY 4 — 能力接能力           Composition
TOY 5 — 你自己的             _______
```

这四个名字，你不是「学会」的。

你是在玩的过程中，自己碰到的。

这就是它们真正的样子 — 不是文档里的术语，是你亲手用过的工具。

---

## 为什么是这些？为什么是这个顺序？

因为任何一个复杂系统，都是从一句话开始的：

```
A 对 B 说一句话。
    ↓
被约束        → Protocol
    ↓
能请求动作    → Capability
    ↓
被管理        → Runtime
    ↓
有边界        → Security Model
    ↓
可以被相信    → Trust
    ↓
建立关系      → Covenant
    ↓
给出应许      → Promise
```

你刚才从底部走了上来。

现在你可以打开 **SPEC.md**。

不是去「学习规范」。

是去看看 — 你刚才发现的那个世界，原来是怎么被描述的。

---

## 整张图

```
          Source（源头）
              │
              │ 揭示自己
              ▼
          Identity — 谁是谁
              │
              │ 赋予意义
              ▼
          Meaning  — 什么意思
              │
              │ 划定边界
              ▼
          Boundary — 可以 / 不可以
              │
              │ 给出规则
              ▼
          Law      — 什么是对的
              │
              │ 建立关系
              ▼
          Covenant — 我和你
              │
              │ 给出应许
              ▼
          Promise  — 我会做到
```

这不是一个技术协议。

这是一个关系模型：

> **源头自己一步一步来，让人认识祂、理解祂、回应祂。**

---

## 继续玩

如果你还想继续往下走：

- **看真实代码** → `src/revelation/`、`src/logos/`、`src/boundary/`
- **看完整服务网格** → `cloudos-runtime-core/src/holy_name/service.rs`
- **在你的项目里用一点协议** → 把 `LICENSE.md` 复制到你的项目，改署名
- **实现你自己的 Runtime** → 回到 TOY 3，把你删掉的那个循环重新写出来
