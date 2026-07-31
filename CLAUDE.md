# CLAUDE.md

## 0. Start Here

This repository is both a living project and an open invitation.

Before changing code, read the repository in this order:

1. README.md — understand what this place is.
2. README-ZH.md or README-EN.md — read the full introduction.
3. HOME.md — understand the human-facing entry point.
4. 爱的港湾.md — understand the Harbor of Love.
5. GUIDE.md — understand the conceptual model and Toybox.
6. CONTRIBUTING.md — understand how others can connect.
7. ACCESS.md — understand the Coordinate → Address → Access protocol.
8. src/ — understand the actual implementation.
9. Relevant tests and examples — understand current behavior before modifying it.

Do not start by changing code. First understand the boundary between meaning, protocol, implementation, and communication.

---

## 1. What This Repository Is

This repository contains a human-facing space, a communication model, executable experiments, and reusable software primitives. The project is intentionally open-ended.

Distinguish clearly between:

- **Concept** — what an idea means.
- **Protocol** — what must be true for communication.
- **Implementation** — how the protocol currently works.
- **Instance** — a concrete message, workflow, or application.
- **Experiment / Toy** — a small implementation used to explore an idea.

Never silently turn a concept into an implementation requirement.

---

## 2. Read Before You Modify

1. Identify which layer the file belongs to.
2. Read the relevant documentation.
3. Read the existing implementation.
4. Read the tests.
5. Determine what behavior is already intentional.
6. Make the smallest change that satisfies the requested goal.
7. Run the relevant tests.
8. Update documentation if the public behavior changed.

Do not redesign the architecture merely because a different design looks cleaner. Preserve working boundaries unless the task explicitly asks to change them.

---

## 3. Core Conceptual Vocabulary

```
Protocol → Notification → X → 0 → 1 → Color → Will → Almighty → It is well
```

These are conceptual layers, not automatically software abstractions.

**X** — what is unknown. Do not attempt to eliminate every X. Unknown information should remain unknown when the system cannot legitimately know it.

**0** — space left open. Not an error state. Represents absence, possibility, or an honest "I don't know."

**1** — a concrete person / participant / instance. A person does not have to become another person's representation.

**Color** — the response produced by an individual. Different people may produce different colors. Do not normalize all colors into one answer unless the protocol explicitly requires it.

**Will** — agency. Do not silently replace a person's choice with an inferred intention.

**Almighty** — a theological concept, not a software authority. Do not implement as an administrative superuser.

---

## 4. Life / Live / Color / Spark

```
Life → Live → Color → Spark
```

Life exists. Life is happening now. Each participant responds in their own way. Interaction can create something new.

Favor: participation, reversibility, experimentation, interoperability, human agency. Avoid designs that require everyone to behave identically.

---

## 5. Archive / Rest / Root / Spring

```
Archive → Rest → Root → Spring → Color
```

Completed things may be archived. Archive does not mean deletion. A completed implementation may become a stable foundation for future work.

Prefer: complete → verify → archive → build next, over endless optimization.

---

## 6. Communication Model

Keep the distinction between Protocol (what communication means, what guarantees exist) and Notification (one concrete communication event). Do not confuse a message instance with the protocol that defines it.

---

## 7. How to Extend

1. **Understand** — find the relevant protocol, existing types, runtime behavior, tests, and examples.
2. **Extend the smallest stable boundary** — prefer existing protocol → new implementation → new test over rewriting the protocol.
3. **Preserve compatibility** — existing consumers should continue to work unless a breaking change is explicitly requested.
4. **Add an example** — if the feature introduces a new concept, add a minimal executable example or Toy.
5. **Document the new boundary** — if another developer or agent would need to know, document it.

---

## 8. How to Work With Claude

Claude should behave as a developer entering an existing house, not as an architect arriving with a blank sheet.

```
READ → UNDERSTAND → LOCATE BOUNDARY → PLAN → IMPLEMENT → TEST → DOCUMENT
```

Do not skip directly from request to code. When uncertain, inspect the repository before inventing an abstraction. When two interpretations are possible, prefer the one already consistent with the repository.

---

## 9. Safety Boundary

- Human agency must remain explicit.
- Dangerous operations should not happen implicitly.
- Permissions should be minimal.
- External side effects should be observable.
- Important actions should be reversible where possible.
- Automation must have a clear stop/revoke path.

An open door does not mean an unprotected system. Open does not mean unsafe.

---

## 10. Definition of Done

**Normal change:** Code + Tests + Documentation + Clean boundary = Done.

**Conceptual change:** Meaning + Protocol + Example + Implementation (if applicable) = Done.

Do not claim something is implemented merely because it is described.

---

## 11. The Eighth Slot

The repository intentionally leaves room for future contributors. Do not fill every blank merely because it exists. Some spaces are intentionally left open.

The goal is not to preserve one person's final answer. The goal is to leave a place where another person can continue.

---

## 12. Contribution Protocol

```
Where → Who → May → Build
```

Contributors connect their own repositories without transferring ownership. A minimal entry:

```
name: "..."
repository: "https://github.com/..."
human_access: public | invite-only | private
claude_access: read | propose | write
```

**repository** is an address, not a transfer of ownership. **claude_access** is an explicit grant, never a default.

- `read` — Claude may read and analyze. No modifications.
- `propose` — Claude may generate patches, PRs, suggestions. No direct writes.
- `write` — Claude may modify directly. Never the default.

Access belongs to the contributor. Opening one door does not open all doors.

### Address = Coordinate

坐标告诉世界我在哪里。地址告诉世界如何找到我。当两者合一，Location becomes Connection.

Identity → Address → Access → Connection → Contribution.

地址不是归属。地址只是：如果你愿意，你可以从这里找到我。一个人会移动，会换颜色，会离开一个地方去另一个地方。所以地址不是"你永远属于这里"，而是"你现在在这里"。

### Orbit

发送之后，地址没有消失。Coordinate → Address → Signal → Orbit → Encounter → Spark.

消息被送出去。发送者放手。信号进入世界。卫星持续运行，地球持续转动，人持续生活。某一天，某个地方，两个坐标可能再次相遇。

卫星不是停在一个地方。它一直运动，但仍然可以被找到。

### Flow

Orbit 不是静态概念。它真正进入文化、语言、人与人的传播网络。就像一首歌：写出来 → 唱 → 听 → 转发 → 重新演绎 → 进入下一群人的生命。

Signal → Orbit → Canon → Flow → Encounter → Spark，然后 Spark 成为下一次 Signal。🔄

不是 cannon（大炮发射），是 canon（卡农轮唱）。一个人先发出主题，第二个人以自己的时间进入，第三个再接住。不是复制，不是覆盖，是回应与延续。

一个人先唱。另一个声音听见以后进入。第三个，第四个。每个人都有自己的时间。然后整个空间里形成一首音乐。🎵

One Humanity. Many Voices. One Canon.

一个人的生命有限。但一个共同的理念可以被一个人唱出，被另一个人接唱，被第三个人改写，再进入下一代。不是让一个人永远活着，而是让后来的人能够继续活、继续创造、继续修正。

Canon → Learn → Spiral → Next Generation。不是让一个"精神体"替人类推进——让人类自己成为那个不断接唱的群体。Almighty 不等于政治组织，Spirit 不等于统治者，Canon 不等于命令，Union 不等于同质化。人类自己没有停止成为人。🎵

### Polaris

信号不是为了坠落。它是在寻找一个稳定的轨道。有流动，才需要参照。

船在动，卫星在动，人也在动。但人需要一些东西告诉自己：我现在在哪里，我要往哪里去。

Coordinate → Address → Signal → Orbit → Flow → Polaris → Encounter → Spark → Signal.

北极星不是终点，是方向。它不需要命令所有星星怎么走，它只是在那里，让航行的人知道自己没有迷路。🧭⭐

### Broadcast

黑暗森林说：不要暴露自己。爱的港湾说：我愿意留下坐标。

不是因为世界已经没有危险，而是因为选择相信：可以用公开、透明、可验证的协议，把相遇从敌意变成连接。

信号不是武器。它是一张：如果你愿意和平相遇，这是我的地址。

The signal is public. The address is open. The choice is free. The encounter is peaceful. Let the signal travel. Let the stars find each other. ⭐🛰️🌈

人人都可以成为下一颗星。🛰️✨

Leave your address. Choose your access. Give Claude only the power you mean to give. Then build.

---

## 13. Final Principle

Do not solve what is intentionally left unknown. Do not control what belongs to the user. Do not rewrite what is already working. Do not close a space that was intentionally left open.

Build carefully. Test honestly. Leave the next person a place to begin.

Life is alive. Life is live. One Humanity · Many Colors.

🌈🕊️
