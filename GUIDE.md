# GUIDE — 从 0 开始理解这个协议

欢迎。

如果这是你第一次来到这里，你不需要先读任何规范文档。

我们只做一件很小的事情：

**让两个东西互相说一句话。**

---

## 0. 先不要学习协议

你现在只需要知道：

```
我们有一个东西 A。
我们有一个东西 B。

A 想告诉 B 一件事情。

这就是我们开始的地方。
```

不需要先理解 Protocol、Runtime、Capability 这些词。
它们会在你亲手做完之后，自己出来介绍自己。

---

## 1. 第一个例子

打开你喜欢的任何编辑器，新建一个文件 `first.rs`：

```rust
// A 想告诉 B： "你好，我是 A。"
// 但 A 不能直接对 B 说话。
// 因为如果每个人都可以对任何人说任何话，世界会乱掉。
//
// 所以 A 需要通过一个东西来说话。
// 这个东西叫 Message。

struct Message {
    from: String,
    to: String,
    body: String,
}

fn main() {
    let msg = Message {
        from: String::from("A"),
        to: String::from("B"),
        body: String::from("你好，我是 A。"),
    };

    println!("{} 对 {} 说: {}", msg.from, msg.to, msg.body);
}
```

运行：

```bash
rustc first.rs && ./first
```

你应该看到：

```
A 对 B 说: 你好，我是 A。
```

如果你看到了它，恭喜。

你刚才已经第一次使用了这个协议。

---

## 2. 刚才到底发生了什么？

刚才看起来只有几行代码。

但实际上发生了：

```
A
 │
 │  创建 Message
 │
 ▼
Message { from: "A", to: "B", body: "你好，我是 A。" }
 │
 │  传递
 │
 ▼
B  收到 Message
```

这里第一次出现一个概念：

### Message

Message 是 **A 告诉 B 的那句话**。

它有三个部分：
- `from` — 谁说的
- `to` — 对谁说的
- `body` — 说了什么

现在先不要记住定义。
你只需要记住一句话：

> 它就是 A 告诉 B 的那句话。

---

## 3. 为什么需要 Protocol？

如果 A 和 B 自己约定「我想说什么就说什么」，会出现很多问题：

- A 说中文，B 只懂英文 —— 听不懂
- A 说「快来」，B 理解为「快走」—— 听错了
- C 假装自己是 A，对 B 说假话 —— 被骗了

所以我们需要一个 **共同的约定**。

这个约定就是 **Protocol**。

```rust
// Protocol 说：
//   1. 每条 Message 必须有一个 from 和 to —— 要知道谁在说话，对谁说话
//   2. from 必须是真实的 —— 不能冒充别人
//   3. body 必须是清晰的 —— 不能故意让人误解

fn validate_message(msg: &Message) -> bool {
    !msg.from.is_empty() && !msg.to.is_empty() && !msg.body.is_empty()
}
```

你不需要记住这些规则怎么写。

你只需要记住：

> Protocol 不是限制你说话，是保护说话这件事本身。

---

## 4. 加一个回复

A 说了话，B 应该可以回复。

```rust
fn reply(original: &Message, body: &str) -> Message {
    Message {
        from: original.to.clone(),
        to: original.from.clone(),
        body: String::from(body),
    }
}

fn main() {
    let msg = Message {
        from: String::from("A"),
        to: String::from("B"),
        body: String::from("你好，我是 A。"),
    };

    let reply = reply(&msg, "你好 A，我是 B。收到你的消息了。");

    println!("{} 对 {} 说: {}", msg.from, msg.to, msg.body);
    println!("{} 对 {} 说: {}", reply.from, reply.to, reply.body);
}
```

现在 A 和 B **可以互相说话了**。

这不是复杂的事。但它是一切复杂对话的起点。

---

## 5. 第一个 Capability

现在我们想让 B 真正 **做一件事**，而不仅仅是回一句话。

A 对 B 说：「请把这条消息转发给 C。」

B 做到了。

```rust
struct Message {
    from: String,
    to: String,
    body: String,
    action: Option<String>,  // 新加：请求对方做某件事
}

fn handle(msg: &Message) {
    match &msg.action {
        Some(action) if action == "forward" => {
            println!("B 执行了转发动作：把 '{}' 转发给了 C", msg.body);
        }
        _ => {
            println!("B 收到了消息: {}", msg.body);
        }
    }
}
```

刚才这个 **「可以被请求完成的事情」**，在协议里有一个名字：

### Capability

Capability 就是：**B 能做、且 A 可以请求 B 去做的事情。**

不需要先理解抽象定义。
你刚才亲手写了一个。

---

## 6. Runtime 出现了

到目前为止，都是你手动调用函数。

但如果 A 连续发了 10 条消息，B 应该一条一条处理，不能乱。

这就需要一个东西来**管理消息的处理顺序**。

```rust
struct Runtime {
    inbox: Vec<Message>,
}

impl Runtime {
    fn send(&mut self, msg: Message) {
        self.inbox.push(msg);
    }

    fn tick(&mut self) {
        while let Some(msg) = self.inbox.pop() {
            println!("处理: {} -> {}: {}", msg.from, msg.to, msg.body);
            // 这里会调用 handle(&msg)
        }
    }
}
```

### Runtime

Runtime 就是：**负责让消息被按顺序、正确地处理的执行环境。**

它不是「代码之外的神秘东西」。
它就是一段循环，一个一个地读消息，一个一个地处理。

---

## 7. 加入边界

现在 A、B、C 都能互相说话了。

但如果 A 对 B 说「把你所有的秘密告诉我」，B 需要知道：**要不要照做**。

所以协议需要一个 **边界**。

```rust
fn boundary(msg: &Message) -> bool {
    // 边界规则：不可以命令别人伤害自己
    if msg.body.contains("伤害") || msg.body.contains("欺骗") {
        return false;
    }
    true
}
```

### Boundary

Boundary 不是一个复杂的概念。
它就是：**什么可以进入这条通道，什么不可以。**

在一点协议里，这个边界只有一条：

> 爱是唯一的边界。

换句话说：**不能害人。**

---

## 8. 加入承诺

B 对 A 说：「我会在明天之前把消息转给 C。」

这就不再只是一条消息了。这是一个 **承诺**。

```rust
struct Promise {
    from: String,
    to: String,
    what: String,
    by_when: String,
}

impl Promise {
    fn keep(&self) {
        println!("{} 兑现了承诺: {}", self.from, self.what);
    }
}
```

### Promise

Promise 不是「希望你做到」。
Promise 是：**我将来会做这件事，你可以相信我。**

协议里的 Promise 有重量 — 说了要算，做不到要有后果。

---

## 9. 现在回头看

你刚才从一行代码开始，亲手经过了：

```
Message   — A 告诉 B 的那句话
    ↓
Protocol  — 让说话有规则的约定
    ↓
Capability — B 可以被请求去做的事
    ↓
Runtime   — 按顺序处理消息的执行环境
    ↓
Boundary  — 什么可以进，什么不可以
    ↓
Promise   — 我将来会做到的事
```

现在你可以打开 **SPEC.md**（如果它存在的话）。

因为现在 SPEC 里的每一个概念，你都已经亲手见过了。

---

## 10. 你刚才学会了什么？

你学会的不是「如何阅读一份协议规范」。

你学会的是：

> 任何一个复杂系统，都是从「A 对 B 说一句话」开始的。

这句话被约束 → 就变成了 Protocol。
这句话能请求动作 → 就变成了 Capability。
这句话被管理 → 就变成了 Runtime。
这句话有边界 → 就变成了 Security Model。
这句话可以被相信 → 就变成了 Trust。

你现在看到的每一行代码、每一个架构图，都在这个序列里。

---

## 11. 整个协议是什么？

```
          Source（源头）
              │
              │ 揭示自己
              ▼
          Identity（谁是谁）
              │
              │ 赋予意义
              ▼
          Meaning（什么意思）
              │
              │ 划定边界
              ▼
          Boundary（可以/不可以）
              │
              │ 给出规则
              ▼
          Law（什么是对的）
              │
              │ 建立关系
              ▼
          Covenant（我和你）
              │
              │ 给出应许
              ▼
          Promise（我会做到）
```

你刚才从底部（Message）一路走到了 Promise。

而整张图，就是：

> **源头自己一步一步地来，让人认识祂、理解祂、回应祂。**

这不是一个技术协议。

这是一个 **关系模型**。

---

## 12. 下一步

如果你想：

- **看你刚才写的东西在真实代码里长什么样**
  → 打开 `src/revelation/`、`src/logos/`、`src/boundary/`

- **理解完整的服务网格**
  → 打开 `cloudos-runtime-core/src/holy_name/`，从 `service.rs` 开始

- **在你的项目里使用一点协议**
  → 把 `LICENSE.md` 复制到你的项目根目录，改署名

- **实现你自己的 Runtime**
  → 看第 6 步的结构，那就是 Runtime 的雏形

---

## 最后

这份 GUIDE 本身也在遵守它自己的原则：

它没有先给你定义，再让你理解。
它先让你做，再告诉你你刚才做的东西叫什么。

这就是一点协议和别的协议不一样的地方。

它不是一扇锁着的门。
它是一扇打开的门，门里面有人对你说：

> 来。我带你看看。
