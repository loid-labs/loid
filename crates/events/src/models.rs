use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::{NoContext, Timestamp, Uuid};

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Default, Hash,
)]
pub enum Impact {
    #[default]
    NEGLIGIBLE,
    MINOR,
    MODERATE,
    SIGNIFICANT,
    SEVERE,
}

impl Display for Impact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Impact::NEGLIGIBLE => write!(f, "NEGLIGIBLE"),
            Impact::MINOR => write!(f, "MINOR"),
            Impact::MODERATE => write!(f, "MODERATE"),
            Impact::SIGNIFICANT => write!(f, "SIGNIFICANT"),
            Impact::SEVERE => write!(f, "SEVERE"),
        }
    }
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Default, Hash,
)]
pub enum Urgency {
    #[default]
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

impl Display for Urgency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Urgency::LOW => write!(f, "LOW"),
            Urgency::MEDIUM => write!(f, "MEDIUM"),
            Urgency::HIGH => write!(f, "HIGH"),
            Urgency::CRITICAL => write!(f, "CRITICAL"),
        }
    }
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Default, Hash,
)]
pub enum Priority {
    #[default]
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::LOW => write!(f, "LOW"),
            Priority::MEDIUM => write!(f, "MEDIUM"),
            Priority::HIGH => write!(f, "HIGH"),
            Priority::CRITICAL => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub system: String,
    pub source_id: Option<String>,
}

impl Default for Source {
    fn default() -> Self {
        Self {
            system: "manual".to_string(),
            source_id: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
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

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::None => serializer.serialize_none(),
            Value::String(s) => serializer.serialize_str(s),
            Value::Int(i) => serializer.serialize_i64(*i),
            Value::Float(f) => serializer.serialize_f64(*f),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::List(list) => list.serialize(serializer),
            Value::Map(map) => map.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid value: string, int, float, bool, list, map, or null")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Bool(value))
            }

            // Signed integer types
            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Int(value as i64))
            }

            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Int(value as i64))
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Int(value as i64))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Int(value))
            }

            // Unsigned integer types
            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Int(value as i64))
            }

            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Int(value as i64))
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Int(value as i64))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Handle potential overflow when converting u64 to i64
                if value <= i64::MAX as u64 {
                    Ok(Value::Int(value as i64))
                } else {
                    // If the u64 value is too large for i64, store as float
                    // This preserves the value but may lose precision for very large numbers
                    Ok(Value::Float(value as f64))
                }
            }

            // Float types
            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Float(value as f64))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Float(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::String(value.to_string()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::String(value))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::None)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::None)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }
                Ok(Value::List(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut result = HashMap::new();
                while let Some((key, value)) = map.next_entry()? {
                    result.insert(key, value);
                }
                Ok(Value::Map(result))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn new() -> Self {
        Self {
            correlation_id: None,
            source: Source::default(),
            fields: HashMap::new(),
            priority: Priority::default(),
            impact: Impact::default(),
            urgency: Urgency::default(),
            received_at: None,
        }
    }

    pub fn with_correlation_id(&mut self, correlation_id: Uuid) -> &mut Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_source(&mut self, source: Source) -> &mut Self {
        self.source = source;
        self
    }

    pub fn with_priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = priority;
        self
    }

    pub fn with_impact(&mut self, impact: Impact) -> &mut Self {
        self.impact = impact;
        self
    }

    pub fn with_urgency(&mut self, urgency: Urgency) -> &mut Self {
        self.urgency = urgency;
        self
    }

    pub fn with_field(&mut self, key: &str, value: Value) -> &mut Self {
        self.fields.insert(key.to_string(), value);
        self
    }

    pub fn with_text_field(&mut self, key: &str, value: &str) -> &mut Self {
        self.with_field(key, Value::String(value.to_string()))
    }

    pub fn with_int_field(&mut self, key: &str, value: i64) -> &mut Self {
        self.with_field(key, Value::Int(value))
    }

    pub fn with_float_field(&mut self, key: &str, value: f64) -> &mut Self {
        self.with_field(key, Value::Float(value))
    }

    pub fn with_bool_field(&mut self, key: &str, value: bool) -> &mut Self {
        self.with_field(key, Value::Bool(value))
    }

    pub fn with_list_field(&mut self, key: &str, value: Vec<Value>) -> &mut Self {
        self.with_field(key, Value::List(value))
    }

    pub fn with_map_field(&mut self, key: &str, value: HashMap<String, Value>) -> &mut Self {
        self.with_field(key, Value::Map(value))
    }

    pub fn build(&self) -> Event {
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
            source: self.source.clone(),
            created_at: now,
            received_at,
            fields: self.fields.clone(),
            priority: self.priority,
            impact: self.impact,
            urgency: self.urgency,
            resolved_at: None,
        }
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

// Add these From implementations after your existing ones
impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::Int(value as i64)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::Int(value as i64)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Int(value as i64)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        if value <= i64::MAX as u64 {
            Self::Int(value as i64)
        } else {
            Self::Float(value as f64)
        }
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::Int(value as i64)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Int(value as i64)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(value as i64)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
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
        assert_eq!(source.system, "manual");
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
        let float_val = Value::Float(6.852);
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
        let value: Value = 6.8646.into();
        matches!(value, Value::Float(f) if (f - 6.8646).abs() < f64::EPSILON);
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
        let mut builder = EventBuilder::new();

        // Build event to verify the source was set correctly
        let event = builder.with_source(source).build();
        assert_eq!(event.source.system, "test_system");
        assert_eq!(event.source.source_id, None);

        // Test default values
        assert_eq!(event.impact, Impact::NEGLIGIBLE);
        assert_eq!(event.urgency, Urgency::LOW);
        assert_eq!(event.priority, Priority::LOW);
    }

    #[test]
    fn test_event_builder_with_correlation_id() {
        let source = Source::default();
        let correlation_id = Uuid::new_v4();

        let event = EventBuilder::new()
            .with_source(source)
            .with_correlation_id(correlation_id)
            .build();

        assert_eq!(event.correlation_id, Some(correlation_id));
    }

    #[test]
    fn test_event_builder_with_priority() {
        let source = Source::default();

        let event = EventBuilder::new()
            .with_priority(Priority::CRITICAL)
            .build();

        assert_eq!(event.priority, Priority::CRITICAL);
    }

    #[test]
    fn test_event_builder_with_impact() {
        let event = EventBuilder::new().with_impact(Impact::SEVERE).build();

        assert_eq!(event.impact, Impact::SEVERE);
    }

    #[test]
    fn test_event_builder_with_urgency() {
        let event = EventBuilder::new().with_urgency(Urgency::CRITICAL).build();

        assert_eq!(event.urgency, Urgency::CRITICAL);
    }

    #[test]
    fn test_event_builder_with_all_priority_fields() {
        let event = EventBuilder::new()
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
        let value = Value::String("test_value".to_string());

        let event = EventBuilder::new().with_field("test_key", value).build();

        assert!(event.fields.contains_key("test_key"));
        matches!(event.fields.get("test_key"), Some(Value::String(s)) if s == "test_value");
    }

    #[test]
    fn test_event_builder_with_text_field() {
        let event = EventBuilder::new()
            .with_text_field("name", "John Doe")
            .build();

        matches!(event.fields.get("name"), Some(Value::String(s)) if s == "John Doe");
    }

    #[test]
    fn test_event_builder_with_int_field() {
        let event = EventBuilder::new().with_int_field("age", 30).build();

        matches!(event.fields.get("age"), Some(Value::Int(30)));
    }

    #[test]
    fn test_event_builder_with_float_field() {
        let event = EventBuilder::new().with_float_field("score", 95.5).build();

        matches!(event.fields.get("score"), Some(Value::Float(95.5)));
    }

    #[test]
    fn test_event_builder_with_bool_field() {
        let event = EventBuilder::new().with_bool_field("active", true).build();

        matches!(event.fields.get("active"), Some(Value::Bool(true)));
    }

    #[test]
    fn test_event_builder_with_list_field() {
        let list = vec![Value::Int(1), Value::Int(2), Value::Int(3)];

        let event = EventBuilder::new().with_list_field("numbers", list).build();

        matches!(event.fields.get("numbers"), Some(Value::List(v)) if v.len() == 3);
    }

    #[test]
    fn test_event_builder_with_map_field() {
        let mut map = HashMap::new();
        map.insert(
            "nested_key".to_string(),
            Value::String("nested_value".to_string()),
        );

        let event = EventBuilder::new().with_map_field("metadata", map).build();

        matches!(event.fields.get("metadata"), Some(Value::Map(m)) if m.len() == 1);
    }

    #[test]
    fn test_event_builder_chaining() {
        let source = Source {
            system: "test_system".to_string(),
            source_id: Some("123".to_string()),
        };
        let correlation_id = Uuid::new_v4();

        let event = EventBuilder::new()
            .with_source(source)
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
        let event1 = EventBuilder::new().build();
        let event2 = EventBuilder::new().build();

        assert_ne!(event1.id, event2.id);
    }

    #[test]
    fn test_event_build_sets_timestamps() {
        let before = Utc::now();

        let event = EventBuilder::new().build();

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
        let event = EventBuilder::new().build();

        // UUID v7 has version bits set to 0111 (7) in the version field
        assert_eq!(event.id.get_version_num(), 7);
    }

    #[test]
    fn test_event_clone() {
        let source = Source {
            system: "test".to_string(),
            source_id: Some("123".to_string()),
        };

        let original = EventBuilder::new()
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
        let event = EventBuilder::new().build();

        // Test that default values are set correctly
        assert_eq!(event.impact, Impact::NEGLIGIBLE);
        assert_eq!(event.urgency, Urgency::LOW);
        assert_eq!(event.priority, Priority::LOW);
    }

    #[test]
    fn test_complex_nested_values() {
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

        let event = EventBuilder::new()
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
        let event = EventBuilder::new().build();

        assert!(event.fields.is_empty());
        assert!(event.correlation_id.is_none());
        assert!(event.resolved_at.is_none());
    }

    #[test]
    fn test_overwrite_field() {
        let event = EventBuilder::new()
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
        // Test maximum severity event
        let critical_event = EventBuilder::new()
            .with_impact(Impact::SEVERE)
            .with_urgency(Urgency::CRITICAL)
            .with_priority(Priority::CRITICAL)
            .with_text_field("severity", "maximum")
            .build();

        assert_eq!(critical_event.impact, Impact::SEVERE);
        assert_eq!(critical_event.urgency, Urgency::CRITICAL);
        assert_eq!(critical_event.priority, Priority::CRITICAL);

        // Test minimum severity event
        let minimal_event = EventBuilder::new()
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
        let event = EventBuilder::new().build();

        // created_at and received_at should be very close or equal when built
        let time_diff =
            (event.created_at.timestamp_millis() - event.received_at.timestamp_millis()).abs();
        assert!(time_diff <= 1); // Should be within 1ms

        // resolved_at should be None for new events
        assert!(event.resolved_at.is_none());
    }
}

#[cfg(test)]
mod json_tests {
    use super::*;
    use serde_json;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_event_to_json_and_back() {
        // Create a test event with all fields populated
        let source = Source {
            system: "test_system".to_string(),
            source_id: Some("test_id_123".to_string()),
        };

        let correlation_id = Uuid::new_v4();

        let mut fields = HashMap::new();
        fields.insert(
            "message".to_string(),
            Value::String("Test event message".to_string()),
        );
        fields.insert("count".to_string(), Value::Int(42));
        fields.insert("enabled".to_string(), Value::Bool(true));
        fields.insert("score".to_string(), Value::Float(95.5));

        let original_event = EventBuilder::new()
            .with_correlation_id(correlation_id)
            .with_impact(Impact::MODERATE)
            .with_urgency(Urgency::HIGH)
            .with_priority(Priority::CRITICAL)
            .with_text_field("message", "Test event message")
            .with_int_field("count", 42)
            .with_bool_field("enabled", true)
            .with_float_field("score", 95.5)
            .build();

        // Serialize to JSON
        let json_string =
            serde_json::to_string(&original_event).expect("Failed to serialize event to JSON");

        // Deserialize back from JSON
        let deserialized_event: Event =
            serde_json::from_str(&json_string).expect("Failed to deserialize event from JSON");

        // Verify the roundtrip worked correctly
        assert_eq!(original_event.id, deserialized_event.id);
        assert_eq!(
            original_event.correlation_id,
            deserialized_event.correlation_id
        );
        assert_eq!(
            original_event.source.system,
            deserialized_event.source.system
        );
        assert_eq!(
            original_event.source.source_id,
            deserialized_event.source.source_id
        );
        assert_eq!(original_event.impact, deserialized_event.impact);
        assert_eq!(original_event.urgency, deserialized_event.urgency);
        assert_eq!(original_event.priority, deserialized_event.priority);
        assert_eq!(original_event.created_at, deserialized_event.created_at);
        assert_eq!(original_event.received_at, deserialized_event.received_at);
        assert_eq!(original_event.resolved_at, deserialized_event.resolved_at);

        // Verify fields
        assert_eq!(original_event.fields.len(), deserialized_event.fields.len());
        for (key, value) in &original_event.fields {
            assert_eq!(deserialized_event.fields.get(key), Some(value));
        }
    }
}
