// ponytail: Mailbox = canonical fact store. Notification/Call/Attention are layers above it.
// Only what was discovered through TOY 6–7 play. Not a full message system.

/// Attention level — the message declares where it belongs. The system provides the room.
#[derive(Debug, Clone, PartialEq)]
pub enum Level {
    Archive, // happened, at rest — no attention needed
    Passive, // can wait quietly
    Active,  // needs attention
    Urgent,  // triggers Call when attention is full
}

#[derive(Debug, Clone)]
pub struct Message {
    pub from: String,
    pub body: String,
    pub level: Level,
}

/// Mailbox is the canonical fact store. Messages enter, never leave.
/// Attention buffer is limited — overflow triggers Call, not deletion.
pub struct Mailbox {
    pub owner: String,
    archive: Vec<Message>,
    attention: Vec<Message>,
    attention_max: usize,
    pub call_log: Vec<String>,
}

impl Mailbox {
    pub fn new(owner: &str, attention_max: usize) -> Self {
        Mailbox {
            owner: String::from(owner),
            archive: Vec::new(),
            attention: Vec::new(),
            attention_max,
            call_log: Vec::new(),
        }
    }

    pub fn receive(&mut self, msg: Message) {
        self.archive.push(msg.clone());

        match msg.level {
            Level::Archive => {} // no attention needed, stays in archive only
            Level::Passive | Level::Active | Level::Urgent => {
                self.attention.push(msg.clone());
                if self.attention.len() > self.attention_max {
                    // Urgent messages trigger Call when attention overflows
                    if msg.level == Level::Urgent {
                        self.call_log.push(format!(
                            "⏰ {} 需要你注意！积压 {} 条",
                            self.owner,
                            self.attention.len()
                        ));
                    }
                }
            }
        }
    }

    /// B processes N attention items. They leave attention but stay in archive.
    pub fn acknowledge(&mut self, count: usize) -> Vec<Message> {
        let n = count.min(self.attention.len());
        self.attention.drain(..n).collect()
    }

    pub fn attention_count(&self) -> usize {
        self.attention.len()
    }

    pub fn archive_count(&self) -> usize {
        self.archive.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(from: &str, body: &str, level: Level) -> Message {
        Message { from: String::from(from), body: String::from(body), level }
    }

    /// Test 1: Notification fails — fact still exists in archive.
    /// Proves: Notification can be lost, Fact cannot.
    #[test]
    fn notification_fails_fact_survives() {
        let mut mb = Mailbox::new("B", 3);
        mb.receive(msg("A", "一条消息", Level::Active));

        // Attention has it
        assert_eq!(mb.attention_count(), 1);
        // Archive has it too — independently
        assert_eq!(mb.archive_count(), 1);

        // Even if we acknowledge (notification gone), fact remains
        mb.acknowledge(1);
        assert_eq!(mb.attention_count(), 0);
        assert_eq!(mb.archive_count(), 1); // fact survives
    }

    /// Test 2: Attention buffer full — facts still in archive.
    /// Proves: Buffer overflow ≠ data loss.
    #[test]
    fn attention_full_fact_survives() {
        let mut mb = Mailbox::new("B", 2);
        mb.receive(msg("A", "第1条", Level::Active));
        mb.receive(msg("C", "第2条", Level::Active));
        mb.receive(msg("D", "第3条", Level::Active)); // overflows attention

        assert_eq!(mb.attention_count(), 3); // all kept, exceeded max
        assert_eq!(mb.archive_count(), 3);   // nothing lost
    }

    /// Test 3: Archive-level messages don't consume attention.
    /// Proves: Low-attention messages have their own home.
    #[test]
    fn low_attention_skips_attention_buffer() {
        let mut mb = Mailbox::new("B", 3);
        mb.receive(msg("A", "天气不错", Level::Archive));
        mb.receive(msg("C", "转发文章", Level::Archive));

        assert_eq!(mb.attention_count(), 0); // never entered attention
        assert_eq!(mb.archive_count(), 2);   // still in canonical store
    }

    /// Test 4: Urgent message triggers Call when attention is full.
    /// Proves: Call is the overflow valve, not deletion.
    #[test]
    fn urgent_triggers_call_when_full() {
        let mut mb = Mailbox::new("B", 2);
        mb.receive(msg("A", "普通1", Level::Active));
        mb.receive(msg("C", "普通2", Level::Active));
        assert_eq!(mb.call_log.len(), 0);

        // Urgent arrives when attention is already full
        mb.receive(msg("王超", "服务器挂了", Level::Urgent));
        assert_eq!(mb.call_log.len(), 1);
        assert!(mb.call_log[0].contains("需要你注意"));

        // Archive still has everything
        assert_eq!(mb.archive_count(), 3);
    }

    /// Test 5: All facts recoverable from archive.
    /// Proves: Nothing is ever truly lost — every fact can be rediscovered.
    #[test]
    fn all_facts_recoverable() {
        let mut mb = Mailbox::new("B", 3);
        mb.receive(msg("A", "日常", Level::Archive));
        mb.receive(msg("C", "提醒", Level::Active));
        mb.receive(msg("D", "紧急", Level::Urgent));

        // Process all attention
        mb.acknowledge(10);
        assert_eq!(mb.attention_count(), 0);

        // Archive still has everything — every fact recoverable
        assert_eq!(mb.archive_count(), 3);
    }
}
