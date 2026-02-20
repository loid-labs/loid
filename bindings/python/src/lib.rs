use loid_events::prelude::{Event, EventBuilder, Impact, Priority, Source, Urgency};
use pyo3::prelude::*;
use pyo3::types::PyDateTime;
use pyo3::IntoPyObjectExt;

#[pyclass(name = "Impact", eq, eq_int, ord, frozen, hash)]
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum PyImpact {
    NEGLIGIBLE,
    MINOR,
    MODERATE,
    SIGNIFICANT,
    SEVERE,
}

impl From<Impact> for PyImpact {
    fn from(impact: Impact) -> Self {
        match impact {
            Impact::NEGLIGIBLE => PyImpact::NEGLIGIBLE,
            Impact::MINOR => PyImpact::MINOR,
            Impact::MODERATE => PyImpact::MODERATE,
            Impact::SIGNIFICANT => PyImpact::SIGNIFICANT,
            Impact::SEVERE => PyImpact::SEVERE,
        }
    }
}

impl From<PyImpact> for Impact {
    fn from(js_impact: PyImpact) -> Self {
        match js_impact {
            PyImpact::NEGLIGIBLE => Impact::NEGLIGIBLE,
            PyImpact::MINOR => Impact::MINOR,
            PyImpact::MODERATE => Impact::MODERATE,
            PyImpact::SIGNIFICANT => Impact::SIGNIFICANT,
            PyImpact::SEVERE => Impact::SEVERE,
        }
    }
}

#[pyclass(name = "Urgency", eq, eq_int, ord, frozen, hash)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PyUrgency {
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

impl From<Urgency> for PyUrgency {
    fn from(urgency: Urgency) -> Self {
        match urgency {
            Urgency::LOW => PyUrgency::LOW,
            Urgency::MEDIUM => PyUrgency::MEDIUM,
            Urgency::HIGH => PyUrgency::HIGH,
            Urgency::CRITICAL => PyUrgency::CRITICAL,
        }
    }
}

impl From<PyUrgency> for Urgency {
    fn from(js_urgency: PyUrgency) -> Self {
        match js_urgency {
            PyUrgency::LOW => Urgency::LOW,
            PyUrgency::MEDIUM => Urgency::MEDIUM,
            PyUrgency::HIGH => Urgency::HIGH,
            PyUrgency::CRITICAL => Urgency::CRITICAL,
        }
    }
}

#[pyclass(name = "Priority", eq, eq_int, ord, frozen, hash)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PyPriority {
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

impl From<Priority> for PyPriority {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::LOW => PyPriority::LOW,
            Priority::MEDIUM => PyPriority::MEDIUM,
            Priority::HIGH => PyPriority::HIGH,
            Priority::CRITICAL => PyPriority::CRITICAL,
        }
    }
}

impl From<PyPriority> for Priority {
    fn from(js_priority: PyPriority) -> Self {
        match js_priority {
            PyPriority::LOW => Priority::LOW,
            PyPriority::MEDIUM => Priority::MEDIUM,
            PyPriority::HIGH => Priority::HIGH,
            PyPriority::CRITICAL => Priority::CRITICAL,
        }
    }
}

#[pyclass(name = "Source")]
#[derive(Clone)]
pub struct PySource(Source);

#[pymethods]
impl PySource {
    #[new]
    fn new(system: String, source_id: Option<String>) -> Self {
        PySource(Source { system, source_id })
    }

    #[getter]
    fn system(&self) -> String {
        self.0.system.clone()
    }

    #[getter]
    fn source_id(&self) -> Option<String> {
        self.0.source_id.clone()
    }

    fn __str__(&self) -> String {
        format!(
            "PySource(system='{}', source_id={:?})",
            self.0.system, self.0.source_id
        )
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass(name = "Event")]
#[derive(Clone)]
pub struct PyEvent(Event);

#[pymethods]
impl PyEvent {
    #[getter]
    fn id(&self) -> String {
        self.0.id.to_string()
    }

    #[getter]
    fn correlation_id(&self) -> Option<String> {
        self.0.correlation_id.map(|id| id.to_string())
    }

    #[getter]
    fn source(&self) -> PySource {
        PySource(self.0.source.clone())
    }

    #[getter]
    fn impact(&self) -> PyImpact {
        self.0.impact.into()
    }

    #[getter]
    fn priority(&self) -> PyPriority {
        self.0.priority.into()
    }

    #[getter]
    fn urgency(&self) -> PyUrgency {
        self.0.urgency.into()
    }

    #[getter]
    fn received_at<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        self.0.received_at.into_pyobject_or_pyerr(py)
    }

    #[getter]
    fn created_at<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        self.0.created_at.into_pyobject_or_pyerr(py)
    }

    #[getter]
    fn resolved_at<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        match self.0.resolved_at {
            Some(dt) => Ok(Some(dt.into_pyobject_or_pyerr(py)?)),
            None => Ok(None),
        }
    }

    fn __str__(&self) -> String {
        format!(
            "Event({}, corr={}, src={}:{}, impact={}, priority={}, urgency={}, rcv={}, crt={}, res={}, fields={})",
            self.0.id,
            self.0
                .correlation_id
                .map_or("None".to_string(), |id| id.to_string()),
            self.0.source.system,
            self.0
                .source
                .source_id
                .as_ref()
                .map_or("None", |s| s.as_str()),
            self.0.impact,
            self.0.priority,
            self.0.urgency,
            self.0.received_at.to_rfc3339(),
            self.0.created_at.to_rfc3339(),
            self.0
                .resolved_at
                .map_or("None".to_string(), |dt| dt.to_rfc3339()),
            self.0.fields.len()
        )
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass(name = "EventBuilder")]
pub struct PyEventBuilder(EventBuilder);

#[pymethods]
impl PyEventBuilder {
    #[new]
    #[pyo3(signature = ())]
    fn new() -> Self {
        PyEventBuilder(EventBuilder::new())
    }

    #[pyo3(signature = (source))]
    fn with_source(mut self_: PyRefMut<'_, Self>, source: PySource) -> PyRefMut<'_, Self> {
        self_.0.with_source(source.0);
        self_
    }

    #[pyo3(signature = (correlation_id))]
    fn with_correlation_id(
        mut self_: PyRefMut<'_, Self>,
        correlation_id: String,
    ) -> PyRefMut<'_, Self> {
        self_.0.with_correlation_id(correlation_id.parse().unwrap());
        self_
    }

    #[pyo3(signature = (priority))]
    fn with_priority(mut self_: PyRefMut<'_, Self>, priority: PyPriority) -> PyRefMut<'_, Self> {
        self_.0.with_priority(priority.into());
        self_
    }

    fn with_impact(mut self_: PyRefMut<'_, Self>, impact: PyImpact) -> PyRefMut<'_, Self> {
        self_.0.with_impact(impact.into());
        self_
    }

    fn with_urgency(mut self_: PyRefMut<'_, Self>, urgency: PyUrgency) -> PyRefMut<'_, Self> {
        self_.0.with_urgency(urgency.into());
        self_
    }

    fn build(&self) -> PyEvent {
        PyEvent(self.0.build())
    }
}

#[pymodule]
fn _loid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<PyImpact>()?;
    m.add_class::<PyUrgency>()?;
    m.add_class::<PyPriority>()?;
    m.add_class::<PySource>()?;
    m.add_class::<PyEvent>()?;
    m.add_class::<PyEventBuilder>()?;
    Ok(())
}
