use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::{NoContext, Timestamp, Uuid};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Default)]
pub enum Impact {
    #[default]
    NEGLIGIBLE,
    MINOR,
    MODERATE,
    SIGNIFICANT,
    SEVERE,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Default)]
pub enum Urgency {
    #[default]
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Default)]
pub enum Priority {
    #[default]
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

#[derive(Debug, Clone, Default)]
pub struct Source {
    pub system: String,
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub enum Value {
    #[default]
    None,
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

#[derive(Debug, Clone)]
pub struct Event {
    pub id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub source: Source,

    pub impact: Impact,
    pub priority: Priority,
    pub urgency: Urgency,

    pub received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,

    pub fields: HashMap<String, Value>,
}

pub struct EventBuilder {
    correlation_id: Option<Uuid>,
    source: Source,
    fields: HashMap<String, Value>,
    priority: Priority,
    impact: Impact,
    urgency: Urgency,
    received_at: Option<DateTime<Utc>>,
}

impl EventBuilder {
    pub fn new(source: Source) -> Self {
        Self {
            correlation_id: None,
            source: source,
            fields: HashMap::new(),
            priority: Priority::default(),
            impact: Impact::default(),
            urgency: Urgency::default(),
            received_at: None,
        }
    }

    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_source(mut self, source: Source) -> Self {
        self.source = source;
        self
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_impact(mut self, impact: Impact) -> Self {
        self.impact = impact;
        self
    }

    pub fn with_urgency(mut self, urgency: Urgency) -> Self {
        self.urgency = urgency;
        self
    }

    pub fn with_field(mut self, key: &str, value: Value) -> Self {
        self.fields.insert(key.to_string(), value);
        self
    }

    pub fn with_text_field(mut self, key: &str, value: &str) -> Self {
        self.with_field(key, Value::String(value.to_string()))
    }

    pub fn with_int_field(mut self, key: &str, value: i64) -> Self {
        self.with_field(key, Value::Int(value))
    }

    pub fn with_float_field(mut self, key: &str, value: f64) -> Self {
        self.with_field(key, Value::Float(value))
    }

    pub fn with_bool_field(mut self, key: &str, value: bool) -> Self {
        self.with_field(key, Value::Bool(value))
    }

    pub fn with_list_field(mut self, key: &str, value: Vec<Value>) -> Self {
        self.with_field(key, Value::List(value))
    }

    pub fn with_map_field(mut self, key: &str, value: HashMap<String, Value>) -> Self {
        self.with_field(key, Value::Map(value))
    }

    pub fn build(self) -> Event {
        let now = Utc::now();
        let event_id = Uuid::new_v7(Timestamp::from_unix(
            NoContext,
            now.timestamp() as u64,
            now.timestamp_subsec_nanos(),
        ));
        let received_at = self.received_at.unwrap_or(now);
        Event {
            id: event_id,
            correlation_id: self.correlation_id,
            source: self.source,
            created_at: now,
            received_at,
            fields: self.fields,
            priority: self.priority,
            impact: self.impact,
            urgency: self.urgency,
            resolved_at: None,
        }
    }
}

impl From<Source> for EventBuilder {
    fn from(source: Source) -> Self {
        Self::new(source)
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::str::FromStr;
    use uuid::Uuid;

    // ========== Impact Tests ==========
    #[test]
    fn test_impact_default() {
        let impact = Impact::default();
        assert_eq!(impact, Impact::NEGLIGIBLE);
    }

    #[test]
    fn test_impact_variants() {
        assert_eq!(Impact::NEGLIGIBLE, Impact::NEGLIGIBLE);
        assert_eq!(Impact::MINOR, Impact::MINOR);
        assert_eq!(Impact::MODERATE, Impact::MODERATE);
        assert_eq!(Impact::SIGNIFICANT, Impact::SIGNIFICANT);
        assert_eq!(Impact::SEVERE, Impact::SEVERE);
    }

    #[test]
    fn test_impact_clone_copy() {
        let impact = Impact::SEVERE;
        let cloned = impact.clone();
        let copied = impact;

        assert_eq!(impact, cloned);
        assert_eq!(impact, copied);
    }

    #[test]
    fn test_impact_debug() {
        let impact = Impact::MODERATE;
        let debug_str = format!("{:?}", impact);
        assert_eq!(debug_str, "MODERATE");
    }

    // ========== Urgency Tests ==========
    #[test]
    fn test_urgency_default() {
        let urgency = Urgency::default();
        assert_eq!(urgency, Urgency::LOW);
    }

    #[test]
    fn test_urgency_variants() {
        assert_eq!(Urgency::LOW, Urgency::LOW);
        assert_eq!(Urgency::MEDIUM, Urgency::MEDIUM);
        assert_eq!(Urgency::HIGH, Urgency::HIGH);
        assert_eq!(Urgency::CRITICAL, Urgency::CRITICAL);
    }

    #[test]
    fn test_urgency_clone_copy() {
        let urgency = Urgency::CRITICAL;
        let cloned = urgency.clone();
        let copied = urgency;

        assert_eq!(urgency, cloned);
        assert_eq!(urgency, copied);
    }

    #[test]
    fn test_urgency_debug() {
        let urgency = Urgency::HIGH;
        let debug_str = format!("{:?}", urgency);
        assert_eq!(debug_str, "HIGH");
    }

    // ========== Priority Tests ==========
    #[test]
    fn test_priority_default() {
        let priority = Priority::default();
        assert_eq!(priority, Priority::LOW);
    }

    #[test]
    fn test_priority_variants() {
        assert_eq!(Priority::LOW, Priority::LOW);
        assert_eq!(Priority::MEDIUM, Priority::MEDIUM);
        assert_eq!(Priority::HIGH, Priority::HIGH);
        assert_eq!(Priority::CRITICAL, Priority::CRITICAL);
    }

    #[test]
    fn test_priority_clone_copy() {
        let priority = Priority::CRITICAL;
        let cloned = priority.clone();
        let copied = priority;

        assert_eq!(priority, cloned);
        assert_eq!(priority, copied);
    }

    #[test]
    fn test_priority_debug() {
        let priority = Priority::MEDIUM;
        let debug_str = format!("{:?}", priority);
        assert_eq!(debug_str, "MEDIUM");
    }

    // ========== Source Tests ==========
    #[test]
    fn test_source_default() {
        let source = Source::default();
        assert_eq!(source.system, "");
        assert_eq!(source.source_id, None);
    }

    #[test]
    fn test_source_creation() {
        let source = Source {
            system: "test_system".to_string(),
            source_id: Some("source_123".to_string()),
        };
        assert_eq!(source.system, "test_system");
        assert_eq!(source.source_id, Some("source_123".to_string()));
    }

    #[test]
    fn test_source_clone() {
        let source = Source {
            system: "test_system".to_string(),
            source_id: Some("source_123".to_string()),
        };
        let cloned = source.clone();
        assert_eq!(source.system, cloned.system);
        assert_eq!(source.source_id, cloned.source_id);
    }

    // ========== Value Tests ==========
    #[test]
    fn test_value_default() {
        let value = Value::default();
        matches!(value, Value::None);
    }

    #[test]
    fn test_value_variants() {
        let string_val = Value::String("test".to_string());
        let int_val = Value::Int(42);
        let float_val = Value::Float(3.14);
        let bool_val = Value::Bool(true);
        let list_val = Value::List(vec![Value::Int(1), Value::Int(2)]);
        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        let map_val = Value::Map(map);
        let none_val = Value::None;

        // Test that each variant can be created and matches correctly
        matches!(string_val, Value::String(_));
        matches!(int_val, Value::Int(42));
        matches!(float_val, Value::Float(_));
        matches!(bool_val, Value::Bool(true));
        matches!(list_val, Value::List(_));
        matches!(map_val, Value::Map(_));
        matches!(none_val, Value::None);
    }

    #[test]
    fn test_value_clone() {
        let original = Value::String("test".to_string());
        let cloned = original.clone();
        matches!(cloned, Value::String(s) if s == "test");
    }

    #[test]
    fn test_value_from_string() {
        let value: Value = "test".into();
        matches!(value, Value::String(s) if s == "test");

        let value: Value = String::from("test").into();
        matches!(value, Value::String(s) if s == "test");
    }

    #[test]
    fn test_value_from_i64() {
        let value: Value = 42i64.into();
        matches!(value, Value::Int(42));
    }

    #[test]
    fn test_value_from_f64() {
        let value: Value = 3.14f64.into();
        matches!(value, Value::Float(f) if (f - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn test_value_from_bool() {
        let value_true: Value = true.into();
        let value_false: Value = false.into();
        matches!(value_true, Value::Bool(true));
        matches!(value_false, Value::Bool(false));
    }

    #[test]
    fn test_value_from_str() {
        let value = Value::from_str("test").unwrap();
        matches!(value, Value::String(s) if s == "test");
    }

    // ========== EventBuilder Tests ==========
    #[test]
    fn test_event_builder_new() {
        let source = Source {
            system: "test_system".to_string(),
            source_id: None,
        };
        let builder = EventBuilder::new(source.clone());

        // Build event to verify the source was set correctly
        let event = builder.build();
        assert_eq!(event.source.system, "test_system");
        assert_eq!(event.source.source_id, None);

        // Test default values
        assert_eq!(event.impact, Impact::NEGLIGIBLE);
        assert_eq!(event.urgency, Urgency::LOW);
        assert_eq!(event.priority, Priority::LOW);
    }

    #[test]
    fn test_event_builder_from_source() {
        let source = Source {
            system: "test_system".to_string(),
            source_id: Some("123".to_string()),
        };
        let builder: EventBuilder = source.clone().into();
        let event = builder.build();
        assert_eq!(event.source.system, "test_system");
        assert_eq!(event.source.source_id, Some("123".to_string()));
    }

    #[test]
    fn test_event_builder_with_correlation_id() {
        let source = Source::default();
        let correlation_id = Uuid::new_v4();

        let event = EventBuilder::new(source)
            .with_correlation_id(correlation_id)
            .build();

        assert_eq!(event.correlation_id, Some(correlation_id));
    }

    #[test]
    fn test_event_builder_with_source() {
        let initial_source = Source::default();
        let new_source = Source {
            system: "new_system".to_string(),
            source_id: Some("new_id".to_string()),
        };

        let event = EventBuilder::new(initial_source)
            .with_source(new_source.clone())
            .build();

        assert_eq!(event.source.system, "new_system");
        assert_eq!(event.source.source_id, Some("new_id".to_string()));
    }

    #[test]
    fn test_event_builder_with_priority() {
        let source = Source::default();

        let event = EventBuilder::new(source)
            .with_priority(Priority::CRITICAL)
            .build();

        assert_eq!(event.priority, Priority::CRITICAL);
    }

    #[test]
    fn test_event_builder_with_impact() {
        let source = Source::default();

        let event = EventBuilder::new(source)
            .with_impact(Impact::SEVERE)
            .build();

        assert_eq!(event.impact, Impact::SEVERE);
    }

    #[test]
    fn test_event_builder_with_urgency() {
        let source = Source::default();

        let event = EventBuilder::new(source)
            .with_urgency(Urgency::CRITICAL)
            .build();

        assert_eq!(event.urgency, Urgency::CRITICAL);
    }

    #[test]
    fn test_event_builder_with_all_priority_fields() {
        let source = Source::default();

        let event = EventBuilder::new(source)
            .with_impact(Impact::SIGNIFICANT)
            .with_urgency(Urgency::HIGH)
            .with_priority(Priority::MEDIUM)
            .build();

        assert_eq!(event.impact, Impact::SIGNIFICANT);
        assert_eq!(event.urgency, Urgency::HIGH);
        assert_eq!(event.priority, Priority::MEDIUM);
    }

    #[test]
    fn test_event_builder_with_field() {
        let source = Source::default();
        let value = Value::String("test_value".to_string());

        let event = EventBuilder::new(source)
            .with_field("test_key", value)
            .build();

        assert!(event.fields.contains_key("test_key"));
        matches!(event.fields.get("test_key"), Some(Value::String(s)) if s == "test_value");
    }

    #[test]
    fn test_event_builder_with_text_field() {
        let source = Source::default();

        let event = EventBuilder::new(source)
            .with_text_field("name", "John Doe")
            .build();

        matches!(event.fields.get("name"), Some(Value::String(s)) if s == "John Doe");
    }

    #[test]
    fn test_event_builder_with_int_field() {
        let source = Source::default();

        let event = EventBuilder::new(source).with_int_field("age", 30).build();

        matches!(event.fields.get("age"), Some(Value::Int(30)));
    }

    #[test]
    fn test_event_builder_with_float_field() {
        let source = Source::default();

        let event = EventBuilder::new(source)
            .with_float_field("score", 95.5)
            .build();

        matches!(event.fields.get("score"), Some(Value::Float(95.5)));
    }

    #[test]
    fn test_event_builder_with_bool_field() {
        let source = Source::default();

        let event = EventBuilder::new(source)
            .with_bool_field("active", true)
            .build();

        matches!(event.fields.get("active"), Some(Value::Bool(true)));
    }

    #[test]
    fn test_event_builder_with_list_field() {
        let source = Source::default();
        let list = vec![Value::Int(1), Value::Int(2), Value::Int(3)];

        let event = EventBuilder::new(source)
            .with_list_field("numbers", list)
            .build();

        matches!(event.fields.get("numbers"), Some(Value::List(v)) if v.len() == 3);
    }

    #[test]
    fn test_event_builder_with_map_field() {
        let source = Source::default();
        let mut map = HashMap::new();
        map.insert(
            "nested_key".to_string(),
            Value::String("nested_value".to_string()),
        );

        let event = EventBuilder::new(source)
            .with_map_field("metadata", map)
            .build();

        matches!(event.fields.get("metadata"), Some(Value::Map(m)) if m.len() == 1);
    }

    #[test]
    fn test_event_builder_chaining() {
        let source = Source {
            system: "test_system".to_string(),
            source_id: Some("123".to_string()),
        };
        let correlation_id = Uuid::new_v4();

        let event = EventBuilder::new(source)
            .with_correlation_id(correlation_id)
            .with_impact(Impact::MODERATE)
            .with_urgency(Urgency::HIGH)
            .with_priority(Priority::CRITICAL)
            .with_text_field("user", "alice")
            .with_int_field("count", 42)
            .with_bool_field("success", true)
            .build();

        assert_eq!(event.correlation_id, Some(correlation_id));
        assert_eq!(event.source.system, "test_system");
        assert_eq!(event.impact, Impact::MODERATE);
        assert_eq!(event.urgency, Urgency::HIGH);
        assert_eq!(event.priority, Priority::CRITICAL);
        assert_eq!(event.fields.len(), 3);

        matches!(event.fields.get("user"), Some(Value::String(s)) if s == "alice");
        matches!(event.fields.get("count"), Some(Value::Int(42)));
        matches!(event.fields.get("success"), Some(Value::Bool(true)));
    }

    // ========== Event Tests ==========
    #[test]
    fn test_event_build_generates_unique_ids() {
        let source = Source::default();

        let event1 = EventBuilder::new(source.clone()).build();
        let event2 = EventBuilder::new(source).build();

        assert_ne!(event1.id, event2.id);
    }

    #[test]
    fn test_event_build_sets_timestamps() {
        let source = Source::default();
        let before = Utc::now();

        let event = EventBuilder::new(source).build();

        let after = Utc::now();

        // Both created_at and received_at should be set
        assert!(event.created_at >= before);
        assert!(event.created_at <= after);
        assert!(event.received_at >= before);
        assert!(event.received_at <= after);

        // By default, received_at should equal created_at
        assert_eq!(event.created_at, event.received_at);

        // resolved_at should be None initially
        assert!(event.resolved_at.is_none());
    }

    #[test]
    fn test_event_id_is_uuid_v7() {
        let source = Source::default();
        let event = EventBuilder::new(source).build();

        // UUID v7 has version bits set to 0111 (7) in the version field
        assert_eq!(event.id.get_version_num(), 7);
    }

    #[test]
    fn test_event_clone() {
        let source = Source {
            system: "test".to_string(),
            source_id: Some("123".to_string()),
        };

        let original = EventBuilder::new(source)
            .with_impact(Impact::SEVERE)
            .with_urgency(Urgency::CRITICAL)
            .with_priority(Priority::HIGH)
            .with_text_field("key", "value")
            .build();

        let cloned = original.clone();

        assert_eq!(original.id, cloned.id);
        assert_eq!(original.correlation_id, cloned.correlation_id);
        assert_eq!(original.source.system, cloned.source.system);
        assert_eq!(original.source.source_id, cloned.source.source_id);
        assert_eq!(original.impact, cloned.impact);
        assert_eq!(original.urgency, cloned.urgency);
        assert_eq!(original.priority, cloned.priority);
        assert_eq!(original.created_at, cloned.created_at);
        assert_eq!(original.received_at, cloned.received_at);
        assert_eq!(original.resolved_at, cloned.resolved_at);
        assert_eq!(original.fields.len(), cloned.fields.len());
    }

    #[test]
    fn test_event_default_priority_values() {
        let source = Source::default();
        let event = EventBuilder::new(source).build();

        // Test that default values are set correctly
        assert_eq!(event.impact, Impact::NEGLIGIBLE);
        assert_eq!(event.urgency, Urgency::LOW);
        assert_eq!(event.priority, Priority::LOW);
    }

    #[test]
    fn test_complex_nested_values() {
        let source = Source::default();

        // Create nested map
        let mut inner_map = HashMap::new();
        inner_map.insert(
            "inner_key".to_string(),
            Value::String("inner_value".to_string()),
        );

        // Create list with mixed types
        let mixed_list = vec![
            Value::Int(1),
            Value::String("text".to_string()),
            Value::Bool(true),
            Value::Map(inner_map),
        ];

        let event = EventBuilder::new(source)
            .with_list_field("complex_list", mixed_list)
            .build();

        if let Some(Value::List(list)) = event.fields.get("complex_list") {
            assert_eq!(list.len(), 4);
            matches!(&list[0], Value::Int(1));
            matches!(&list[1], Value::String(s) if s == "text");
            matches!(&list[2], Value::Bool(true));
            matches!(&list[3], Value::Map(m) if m.len() == 1);
        } else {
            panic!("Expected list field not found or wrong type");
        }
    }

    #[test]
    fn test_empty_fields() {
        let source = Source::default();
        let event = EventBuilder::new(source).build();

        assert!(event.fields.is_empty());
        assert!(event.correlation_id.is_none());
        assert!(event.resolved_at.is_none());
    }

    #[test]
    fn test_overwrite_field() {
        let source = Source::default();

        let event = EventBuilder::new(source)
            .with_text_field("key", "first_value")
            .with_text_field("key", "second_value") // Should overwrite
            .build();

        matches!(event.fields.get("key"), Some(Value::String(s)) if s == "second_value");
        assert_eq!(event.fields.len(), 1);
    }

    #[test]
    fn test_priority_enum_ordering() {
        // Test that priorities can be compared
        assert!(Priority::LOW < Priority::MEDIUM);
        assert!(Priority::MEDIUM < Priority::HIGH);
        assert!(Priority::HIGH < Priority::CRITICAL);

        assert!(Impact::NEGLIGIBLE < Impact::MINOR);
        assert!(Impact::MINOR < Impact::MODERATE);
        assert!(Impact::MODERATE < Impact::SIGNIFICANT);
        assert!(Impact::SIGNIFICANT < Impact::SEVERE);

        assert!(Urgency::LOW < Urgency::MEDIUM);
        assert!(Urgency::MEDIUM < Urgency::HIGH);
        assert!(Urgency::HIGH < Urgency::CRITICAL);
    }

    #[test]
    fn test_event_with_extreme_priority_combinations() {
        let source = Source::default();

        // Test maximum severity event
        let critical_event = EventBuilder::new(source.clone())
            .with_impact(Impact::SEVERE)
            .with_urgency(Urgency::CRITICAL)
            .with_priority(Priority::CRITICAL)
            .with_text_field("severity", "maximum")
            .build();

        assert_eq!(critical_event.impact, Impact::SEVERE);
        assert_eq!(critical_event.urgency, Urgency::CRITICAL);
        assert_eq!(critical_event.priority, Priority::CRITICAL);

        // Test minimum severity event
        let minimal_event = EventBuilder::new(source)
            .with_impact(Impact::NEGLIGIBLE)
            .with_urgency(Urgency::LOW)
            .with_priority(Priority::LOW)
            .with_text_field("severity", "minimal")
            .build();

        assert_eq!(minimal_event.impact, Impact::NEGLIGIBLE);
        assert_eq!(minimal_event.urgency, Urgency::LOW);
        assert_eq!(minimal_event.priority, Priority::LOW);
    }

    #[test]
    fn test_timestamp_relationships() {
        let source = Source::default();
        let event = EventBuilder::new(source).build();

        // created_at and received_at should be very close or equal when built
        let time_diff =
            (event.created_at.timestamp_millis() - event.received_at.timestamp_millis()).abs();
        assert!(time_diff <= 1); // Should be within 1ms

        // resolved_at should be None for new events
        assert!(event.resolved_at.is_none());
    }
}
