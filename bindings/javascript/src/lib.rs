#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use chrono::Utc;
use loid_events::prelude::{Event, EventBuilder, Impact, Priority, Source, Urgency};

#[napi(string_enum, js_name = "Impact")]
pub enum JsImpact {
    NEGLIGIBLE,
    MINOR,
    MODERATE,
    SIGNIFICANT,
    SEVERE,
}

impl From<Impact> for JsImpact {
    fn from(impact: Impact) -> Self {
        match impact {
            Impact::NEGLIGIBLE => JsImpact::NEGLIGIBLE,
            Impact::MINOR => JsImpact::MINOR,
            Impact::MODERATE => JsImpact::MODERATE,
            Impact::SIGNIFICANT => JsImpact::SIGNIFICANT,
            Impact::SEVERE => JsImpact::SEVERE,
        }
    }
}

impl From<JsImpact> for Impact {
    fn from(js_impact: JsImpact) -> Self {
        match js_impact {
            JsImpact::NEGLIGIBLE => Impact::NEGLIGIBLE,
            JsImpact::MINOR => Impact::MINOR,
            JsImpact::MODERATE => Impact::MODERATE,
            JsImpact::SIGNIFICANT => Impact::SIGNIFICANT,
            JsImpact::SEVERE => Impact::SEVERE,
        }
    }
}

#[napi(string_enum, js_name = "Urgency")]
pub enum JsUrgency {
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

impl From<Urgency> for JsUrgency {
    fn from(urgency: Urgency) -> Self {
        match urgency {
            Urgency::LOW => JsUrgency::LOW,
            Urgency::MEDIUM => JsUrgency::MEDIUM,
            Urgency::HIGH => JsUrgency::HIGH,
            Urgency::CRITICAL => JsUrgency::CRITICAL,
        }
    }
}

impl From<JsUrgency> for Urgency {
    fn from(js_urgency: JsUrgency) -> Self {
        match js_urgency {
            JsUrgency::LOW => Urgency::LOW,
            JsUrgency::MEDIUM => Urgency::MEDIUM,
            JsUrgency::HIGH => Urgency::HIGH,
            JsUrgency::CRITICAL => Urgency::CRITICAL,
        }
    }
}

#[napi(string_enum, js_name = "Priority")]
pub enum JsPriority {
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

impl From<Priority> for JsPriority {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::LOW => JsPriority::LOW,
            Priority::MEDIUM => JsPriority::MEDIUM,
            Priority::HIGH => JsPriority::HIGH,
            Priority::CRITICAL => JsPriority::CRITICAL,
        }
    }
}

impl From<JsPriority> for Priority {
    fn from(js_priority: JsPriority) -> Self {
        match js_priority {
            JsPriority::LOW => Priority::LOW,
            JsPriority::MEDIUM => Priority::MEDIUM,
            JsPriority::HIGH => Priority::HIGH,
            JsPriority::CRITICAL => Priority::CRITICAL,
        }
    }
}

#[napi(object, js_name = "Source")]
pub struct JsSource {
    pub system: String,
    pub source_id: Option<String>,
}

impl From<Source> for JsSource {
    fn from(source: Source) -> Self {
        JsSource {
            system: source.system,
            source_id: source.source_id,
        }
    }
}

impl From<JsSource> for Source {
    fn from(js_source: JsSource) -> Self {
        Source {
            system: js_source.system,
            source_id: js_source.source_id,
        }
    }
}

#[napi(js_name = "Event")]
pub struct JsEvent {
    inner: Event,
}

#[napi]
impl JsEvent {
    #[napi(getter)]
    pub fn id(&self) -> String {
        self.inner.id.to_string()
    }

    #[napi(getter)]
    pub fn correlation_id(&self) -> Option<String> {
        self.inner.correlation_id.map(|id| id.to_string())
    }

    #[napi(getter)]
    pub fn source(&self) -> JsSource {
        self.inner.source.clone().into()
    }

    #[napi(getter)]
    pub fn impact(&self) -> JsImpact {
        self.inner.impact.into()
    }

    #[napi(getter)]
    pub fn priority(&self) -> JsPriority {
        self.inner.priority.into()
    }

    #[napi(getter)]
    pub fn urgency(&self) -> JsUrgency {
        self.inner.urgency.into()
    }

    #[napi(getter)]
    pub fn received_at(&self) -> chrono::DateTime<Utc> {
        self.inner.received_at
    }

    #[napi(getter)]
    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.inner.created_at
    }

    #[napi(getter)]
    pub fn resolved_at(&self) -> Option<chrono::DateTime<Utc>> {
        self.inner.resolved_at
    }

    #[napi(js_name = "toString")]
    pub fn to_string(&self) -> String {
        format!(
            "Event({}, corr={}, src={}:{}, impact={:?}, priority={:?}, urgency={:?}, rcv={}, crt={}, res={}, fields={})",
            self.inner.id,
            self.inner
                .correlation_id
                .map_or("None".to_string(), |id| id.to_string()),
            self.inner.source.system,
            self.inner
                .source
                .source_id
                .as_ref()
                .map_or("None", |s| s.as_str()),
            self.inner.impact,
            self.inner.priority,
            self.inner.urgency,
            self.inner.received_at.to_rfc3339(),
            self.inner.created_at.to_rfc3339(),
            self.inner
                .resolved_at
                .map_or("None".to_string(), |dt| dt.to_rfc3339()),
            self.inner.fields.len()
        )
    }
}

impl From<Event> for JsEvent {
    fn from(event: Event) -> Self {
        JsEvent { inner: event }
    }
}

#[napi(js_name = "EventBuilder")]
pub struct JsEventBuilder {
    inner: EventBuilder,
}

#[napi]
impl JsEventBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        JsEventBuilder {
            inner: EventBuilder::new(),
        }
    }

    #[napi]
    pub fn with_correlation_id(&mut self, correlation_id: String) -> napi::Result<&Self> {
        let uuid = correlation_id.parse().map_err(|_| {
            napi::Error::new(napi::Status::InvalidArg, "Invalid UUID format".to_string())
        })?;

        self.inner.with_correlation_id(uuid);
        Ok(self)
    }

    #[napi]
    pub fn with_priority(&mut self, priority: JsPriority) -> &Self {
        self.inner.with_priority(priority.into());
        self
    }

    #[napi]
    pub fn with_impact(&mut self, impact: JsImpact) -> &Self {
        self.inner.with_impact(impact.into());
        self
    }

    #[napi]
    pub fn with_urgency(&mut self, urgency: JsUrgency) -> &Self {
        self.inner.with_urgency(urgency.into());
        self
    }

    #[napi]
    pub fn build(&self) -> JsEvent {
        JsEvent {
            inner: self.inner.build(),
        }
    }
}
