use crate::event::InternalEvent;

/// Interface for filtering an `InternalEvent`.
pub(crate) trait Filter: Send + Sync + 'static {
    /// Returns whether the given event fulfills the filter.
    fn eval(&self, event: &InternalEvent) -> bool;
}

#[cfg(unix)]
#[derive(Debug, Clone)]
pub(crate) struct CursorPositionFilter;

#[cfg(unix)]
impl Filter for CursorPositionFilter {
    fn eval(&self, event: &InternalEvent) -> bool {
        matches!(*event, InternalEvent::CursorPosition(_, _))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EventFilter;

impl Filter for EventFilter {
    #[cfg(unix)]
    fn eval(&self, event: &InternalEvent) -> bool {
        matches!(*event, InternalEvent::Event(_))
    }

    #[cfg(windows)]
    fn eval(&self, _: &InternalEvent) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InternalEventFilter;

impl Filter for InternalEventFilter {
    fn eval(&self, _: &InternalEvent) -> bool {
        true
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::{
        super::Event, CursorPositionFilter, EventFilter, Filter, InternalEvent, InternalEventFilter,
    };

    #[test]
    fn test_cursor_position_filter_filters_cursor_position() {
        assert!(!CursorPositionFilter.eval(&InternalEvent::Event(Event::Resize(10, 10))));
        assert!(CursorPositionFilter.eval(&InternalEvent::CursorPosition(0, 0)));
    }

    #[test]
    fn test_event_filter_filters_events() {
        assert!(EventFilter.eval(&InternalEvent::Event(Event::Resize(10, 10))));
        assert!(!EventFilter.eval(&InternalEvent::CursorPosition(0, 0)));
    }

    #[test]
    fn test_event_filter_filters_internal_events() {
        assert!(InternalEventFilter.eval(&InternalEvent::Event(Event::Resize(10, 10))));
        assert!(InternalEventFilter.eval(&InternalEvent::CursorPosition(0, 0)));
    }
}
