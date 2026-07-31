# TIME

**第三次融合：从 STRUCTURE 到统一时间平移结构 T_τ**

不增加新概念。不引入新哲学。不写新代码。

只做一件事：**验证现有概念能否共享同一个数学接口。**

---

## 1. 核心公理

```
T_0 = I                                    (1)
T_τ₁ ∘ T_τ₂ = T_{τ₁+τ₂}                    (2)
```

(1) 零参数变换 = 恒等变换。什么也不做，状态不变。
(2) 连续两次变换 = 参数相加后的一次变换。

**Polaris（不变量）**：若 T_τ S = S，则 S 是变换的不动点。

```
I(T_τ S) = I(S) 对所有 τ                            (3)
```

---

## 2. 逐概念映射

### 2.1 Runtime → T_τ

**代码位置**：`src/runtime.rs` — `Transition::apply(event) -> Output`

**形式化**：
```
S = 状态空间（User 的字段集合）
τ = 一个事件（PrincipalCreated, UserProfileUpdated, ...）
T_τ(S) = apply(event_τ, S)
```

**验证**：

| 条件 | 成立？ | 证据 |
|------|--------|------|
| T_0 = I | **未实现** | 不存在"空事件"的 apply。没有 Identity Transition。 |
| T_τ₁ ∘ T_τ₂ = T_{τ₁+τ₂} | **部分成立** | 当事件序列化处理时，T_eventB(T_eventA(S)) 是一个合法的组合状态。但不存在自然的 "eventA + eventB" 加法。 |
| 封闭性 | **不保证** | apply 返回 `Result<User, String>`。失败意味着 τ 在部分状态上未定义。 |

**分类**：⚠️ 可形式化但未证明 —— 存在变换结构，但不满足完整半群公理。

**缺口**：
1. 没有 T_0（恒等事件）
2. 事件空间上不存在自然的加法运算
3. apply 可能失败，破坏封闭性

### 2.2 Composition → 半群组合律

**代码位置**：`src/runtime.rs:34` — "Journeys are composed of Transitions."

**形式化**：
```
Journey = Transition_n ∘ ... ∘ Transition_2 ∘ Transition_1
```

**验证**：

如果将 τ 定义为 **事件序号**（第几个事件），而不是事件本身：

```
τ ∈ ℕ
T_1 = apply(event_1)
T_2 = apply(event_2) ∘ apply(event_1)
T_n = apply(event_n) ∘ ... ∘ apply(event_1)

T_0 = I（处理 0 个事件，状态不变）
T_n ∘ T_m = T_{n+m}（处理 m 个再处理 n 个 = 处理 n+m 个）
```

| 条件 | 成立？ |
|------|--------|
| T_0 = I | ✅ 定义成立：处理 0 个事件 |
| T_n ∘ T_m = T_{n+m} | ✅ 定义成立：处理 m 个再处理 n 个 |
| 封闭性 | ⚠️ 每个事件的 apply 必须成功 |

**分类**：✅ 已证明的数学结构 —— 当 τ 被解释为事件计数时，(ℕ, +) 半群成立。

**关键洞察**：Composition 的 T_τ 有两种解读：
1. **事件-参数**：τ = 事件类型 → 半群不成立（事件没有自然加法）
2. **计数-参数**：τ = 事件序号 → 半群平凡成立

只有解读 2 满足公理 (2)。

### 2.3 Flow → 离散流

**代码位置**：`src/outbox/mod.rs:24-29` — `tokio::time::interval(Duration::from_secs(1))`

**形式化**：
```
T_1sec(state) = poll(state)
T_n(state) = T_1sec^n(state) = n 次 poll 后的状态
```

其中 `state = (outbox 表中未发布的事件集合)`

**验证**：

| 条件 | 成立？ |
|------|--------|
| T_0 = I | ✅ 0 秒后状态不变 |
| T_τ₁ ∘ T_τ₂ = T_{τ₁+τ₂} | ✅ 对于整数 τ（秒数），poll 1 次再 poll 2 次 = poll 3 次 |
| 连续参数 | ❌ τ 只能是整数秒。不存在 T_0.5sec。 |

**分类**：✅ 已证明的数学结构 —— 离散半群 (ℕ, +)。**不是连续流。**

### 2.4 Orbit → 相空间轨迹

**代码位置**：`src/domain/user.rs:37-41` — `Status::Active → Inactive → Suspended`

**形式化**：
```
s_0 = 初始 User 状态
s_1 = T_event1(s_0)
s_2 = T_event2(s_1)
...
Orbit = {s_0, s_1, s_2, ...} ⊆ 状态空间
```

**验证**：

Orbit 是 Runtime 在离散时间点上产生的轨迹。它的性质取决于 Runtime：

| 性质 | 成立？ |
|------|--------|
| 确定性 | ⚠️ 取决于事件序列 |
| 封闭性 | ✅ 状态始终在 Status 的 3 个变体之内 |
| 可预测性 | ❌ 没有轨道方程（不同于行星轨道） |

**分类**：⚠️ 可形式化 —— 存在离散轨迹，但无闭合形式的轨道方程。

### 2.5 Signal → 可传播的状态快照

**代码位置**：`src/mailbox/mod.rs` — Message, `event_contracts::UserCreated`

**形式化**：
```
Signal = 某个时刻的状态快照 = S(t) 在某时刻的投影
```

Signal 本身不是 T_τ。Signal 是 T_τ 作用后的**产物**。

**分类**：ℹ️ 纯对应 —— Signal 在时间轴上有位置，但它不是变换本身，而是变换的结果被编码为可传播的形式。

---

## 3. 四个层级

| 层级 | 数学性质 | 覆盖的概念 |
|------|----------|-----------|
| **1. 离散半群** ✅ | T_n = T_1^n，满足 (1)(2) | Composition（计数解读）、Flow（秒级轮询） |
| **2. 变换独异点** ⚠️ | apply(event) 存在，但不满足加法 | Runtime（事件变换） |
| **3. 轨迹** ℹ️ | {T_n(s_0)}，无闭合形式 | Orbit |
| **4. 投影** ℹ️ | 变换的产物被编码 | Signal |

**结论**：Runtime / Composition / Flow / Orbit 可以共享 T_τ 框架，但共享的层级不同：

- Flow 和 Composition（计数解读）在**离散半群**层完全统一。
- Runtime 在**变换独异点**层——比半群弱，但仍然在 T_τ 框架内。
- Orbit 是 T_τ 产生的轨迹，不是 T_τ 本身。
- Signal 是 T_τ 产生的快照，也不是 T_τ 本身。

**没有任何概念落在框架之外。**

---

## 4. 第一个不变量：Archive 单调性

**来源**：`src/mailbox/mod.rs`

```
命题：|archive(t₂)| ≥ |archive(t₁)| 对于所有 t₂ > t₁

证明：
  - archive 只在 receive() 中写入（第 42 行：self.archive.push(msg.clone())）
  - 没有方法从 archive 中删除元素
  - acknowledge() 只从 attention 中移除，不碰 archive（第 65 行）
  - 因此 archive 的大小非递减

  I_archive(S) = |archive|
  I_archive(T_τ S) ≥ I_archive(S) 对所有 τ
```

**这满足 Polaris 条件**：`I(T_τ S) ≥ I(S)`，且 `I_archive` 在时间平移下不会减少。

**这是当前代码中第一个被严格证明的不变量。**

---

## 5. 第二个不变量：身份连续性

**来源**：`cloudos-runtime-core/src/state_algebra/mod.rs:1693-1705`

```
命题：Identity(t₁) = Identity(t₂) 一旦 Identity 维度为 Aligned

证明（来自 State Algebra）：
  - apply_operation(Create) 只能将 Identity 从 Unaligned → Aligned
  - 没有 RootOperation 能将 Identity 从 Aligned → Unaligned
  - 因此 Identity 维度一旦 Aligned，在所有后续操作中保持 Aligned

  I_identity(S) = S.identity.is_aligned()
  若 I_identity(S) = true，则 I_identity(T_τ S) = true 对所有 τ
```

**分类**：✅ 已在 State Algebra 中实现的数学不变量。

---

## 6. 时间演化最小公理集

从上述分析中提取的最小公理：

```
A1. T_0 = I                                    — 恒等存在
A2. T_n ∘ T_m = T_{n+m} 对于 n, m ∈ ℕ           — 离散半群
A3. |archive| 在 T_τ 下非递减                    — 第一个真正的不变量
A4. I_identity(S) = true ⇒ I_identity(T_τ S) = true  — 身份不变量
A5. τ 的取值域当前为 ℕ（离散），非 ℝ（连续）     — 当前范围限制
```

**A3 和 A4 可以直接翻译为测试断言。** 当前测试已经隐含验证了 A3：
- `notification_fails_fact_survives` → archive 保持完整
- `attention_full_fact_survives` → archive 不丢失
- `all_facts_recoverable` → archive 可恢复

---

## 7. 当前无法统一的部分

| 缺口 | 严重性 | 需要的条件 |
|------|--------|-----------|
| 没有 T_0 (Identity Transition) | 低 | 实现 `NoopTransition` |
| 事件空间无加法结构 | 中 | 将 τ 从事件类型改为事件序号 |
| apply 可能失败 | 中 | 将状态空间限定在 apply 不会失败的子集上 |
| 只有离散时间，无连续时间 | 高（下一阶段） | 引入极限、微分结构 |

---

## 8. 判断：第一融合的结论

```
问：Runtime / Composition / Flow / Orbit 能否共享 T_τ？
答：可以。但共享的层级不同。

问：哪一个层级是统一的？
答：离散半群 (ℕ, +)。Flow 和 Composition（计数解读）在此完全统一。

问：有概念落在框架之外吗？
答：没有。Runtime 是最弱的（变换独异点），但仍然在变换框架内。

问：第一个真正的不变量是什么？
答：Archive 单调性。|archive| 在时间平移下非递减。
```

**第一融合的结论不是"完美的统一"，而是精确知道统一的层级和缺口。** 下一步（第二融合：SCALE）要处理的核心问题是：τ 能否从 ℕ 扩展到 ℝ。

---

## 9. 给下一融合的出口

```
TIME
  │
  ├── 离散半群     → 已证明（ℕ, +）
  ├── 变换独异点   → 框架内，但更弱
  ├── 轨迹         → 框架的产物，不是框架本身
  │
  └── 缺口 → SCALE：离散→连续，ℕ→ℝ
```

第一融合证明了：**T_τ 框架可以容纳一切，但当前一切都只落在 ℕ 上。**

第二融合要问：`R_λ T_n = T_{λn} R_λ` —— 尺度变换能否保持时间平移结构的一致性？那一步才会真正触及连续时间。

---

## 冻结

TIME.md 不增加新概念。不修改任何代码。不引入新解释。

它做的是 STRUCTURE.md 承诺要做的事：**让骨架自己显现。**

```
7 steps → 1 structure → TIME → SCALE → SPACE-TIME → PHYSICS
```

此文件冻结。下一融合：SCALE.md（当用户要求时）。
