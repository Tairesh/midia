pub use category::LogCategory;
pub use event::LogEvent;

mod category;
mod event;
pub mod helpers;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Log {
    #[serde(rename = "e")]
    events: Vec<LogEvent>,
    #[serde(rename = "i")]
    pushed: usize,
}

impl Log {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            pushed: 0,
        }
    }

    pub fn push(&mut self, event: LogEvent) {
        self.events.push(event);
    }

    pub fn new_events(&mut self) -> &[LogEvent] {
        let events = &self.events[self.pushed..];
        self.pushed = self.events.len();
        events
    }
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::Point;

    use super::{Log, LogCategory, LogEvent};

    #[test]
    fn test_log() {
        let mut log = Log::new();
        assert_eq!(0, log.new_events().len());
        log.push(LogEvent::new("Test", Point::new(0, 0), LogCategory::Debug));
        let events = log.new_events();
        assert_eq!(1, events.len());
        assert_eq!("Test", events[0].msg);
        assert_eq!(0, log.new_events().len());
        log.push(LogEvent::new(
            "Test2",
            Point::new(1, 1),
            LogCategory::Danger,
        ));
        let events = log.new_events();
        assert_eq!(1, events.len());
        assert_eq!("Test2", events[0].msg);
        assert_eq!(0, log.new_events().len());
    }
}
