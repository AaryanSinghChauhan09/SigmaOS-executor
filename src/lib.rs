// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-executor: mint executor component model.
//!
//! Real implementation: executors, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Executor {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintExecutorError {
    EmptyExecutorName,
    DuplicateExecutor(String),
    EmptyEventSubject,
}

impl fmt::Display for MintExecutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExecutorName => write!(f, "executor name must not be empty"),
            Self::DuplicateExecutor(id) => write!(f, "duplicate executor {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintExecutorError {}

pub struct MintExecutor {
    pub name: String,
    executors: BTreeMap<String, Executor>,
    events: BTreeMap<String, Event>,
}

impl MintExecutor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            executors: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_executor(&mut self, item: Executor) -> Result<(), MintExecutorError> {
        if item.name.trim().is_empty() {
            return Err(MintExecutorError::EmptyExecutorName);
        }
        let id = item.id.clone();
        if self.executors.contains_key(&id) {
            return Err(MintExecutorError::DuplicateExecutor(id.clone()));
        }
        self.executors.insert(id, item);
        Ok(())
    }

    pub fn executor_count(&self) -> usize {
        self.executors.len()
    }

    pub fn get_executor(&self, id: &str) -> Option<&Executor> {
        self.executors.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintExecutorError> {
        if event.subject.trim().is_empty() {
            return Err(MintExecutorError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_executor(id: &str, name: &str) -> Executor {
        Executor {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintExecutor::new("MintExecutor");
        assert_eq!(m.name, "MintExecutor");
        assert_eq!(m.executor_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_executor() {
        let mut m = MintExecutor::new("MintExecutor");
        m.add_executor(sample_executor("a1", "Alpha")).unwrap();
        assert_eq!(m.executor_count(), 1);
        assert!(m.get_executor("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintExecutor::new("MintExecutor");
        assert!(m.add_executor(sample_executor("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintExecutor::new("MintExecutor");
        m.add_executor(sample_executor("x", "One")).unwrap();
        assert!(m.add_executor(sample_executor("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintExecutor::new("MintExecutor");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
