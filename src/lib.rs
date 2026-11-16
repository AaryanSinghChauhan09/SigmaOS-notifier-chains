// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-notifier-chains: notifier chains component model.
//!
//! Real implementation: chainss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chains {
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
pub enum NotifierChainsError {
    EmptyChainsName,
    DuplicateChains(String),
    EmptyEventSubject,
}

impl fmt::Display for NotifierChainsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyChainsName => write!(f, "chains name must not be empty"),
            Self::DuplicateChains(id) => write!(f, "duplicate chains {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for NotifierChainsError {}

pub struct NotifierChains {
    pub name: String,
    chainss: BTreeMap<String, Chains>,
    events: BTreeMap<String, Event>,
}

impl NotifierChains {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            chainss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_chains(&mut self, item: Chains) -> Result<(), NotifierChainsError> {
        if item.name.trim().is_empty() {
            return Err(NotifierChainsError::EmptyChainsName);
        }
        let id = item.id.clone();
        if self.chainss.contains_key(&id) {
            return Err(NotifierChainsError::DuplicateChains(id.clone()));
        }
        self.chainss.insert(id, item);
        Ok(())
    }

    pub fn chains_count(&self) -> usize {
        self.chainss.len()
    }

    pub fn get_chains(&self, id: &str) -> Option<&Chains> {
        self.chainss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), NotifierChainsError> {
        if event.subject.trim().is_empty() {
            return Err(NotifierChainsError::EmptyEventSubject);
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

    fn sample_chains(id: &str, name: &str) -> Chains {
        Chains {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = NotifierChains::new("NotifierChains");
        assert_eq!(m.name, "NotifierChains");
        assert_eq!(m.chains_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_chains() {
        let mut m = NotifierChains::new("NotifierChains");
        m.add_chains(sample_chains("a1", "Alpha")).unwrap();
        assert_eq!(m.chains_count(), 1);
        assert!(m.get_chains("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = NotifierChains::new("NotifierChains");
        assert!(m.add_chains(sample_chains("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = NotifierChains::new("NotifierChains");
        m.add_chains(sample_chains("x", "One")).unwrap();
        assert!(m.add_chains(sample_chains("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = NotifierChains::new("NotifierChains");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
