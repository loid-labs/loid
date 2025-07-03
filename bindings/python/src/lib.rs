use loid_events::prelude::{Event, Impact, Priority, Source, Urgency, EventBuilder};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyDateTime;

#[pyclass(name = "Impact", eq, ord)]
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct PyImpact(Impact);

#[pymethods]
impl PyImpact {
    #[new]
    fn new(value: &str) -> PyResult<Self> {
        let impact = match value.to_uppercase().as_str() {
            "NEGLIGIBLE" => Impact::NEGLIGIBLE,
            "MINOR" => Impact::MINOR,
            "MODERATE" => Impact::MODERATE,
            "SIGNIFICANT" => Impact::SIGNIFICANT,
            "SEVERE" => Impact::SEVERE,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Invalid impact value",
                ));
            }
        };
        Ok(PyImpact(impact))
    }

    #[classattr]
    const NEGLIGIBLE: PyImpact = PyImpact(Impact::NEGLIGIBLE);

    #[classattr]
    const MINOR: PyImpact = PyImpact(Impact::MINOR);

    #[classattr]
    const MODERATE: PyImpact = PyImpact(Impact::MODERATE);

    #[classattr]
    const SIGNIFICANT: PyImpact = PyImpact(Impact::SIGNIFICANT);

    #[classattr]
    const SEVERE: PyImpact = PyImpact(Impact::SEVERE);

    fn __str__(&self) -> String {
        format!("Impact.{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Impact('{:?}')", self.0)
    }

    fn __int__(&self) -> i32 {
        self.0 as i32
    }
}

#[pyclass(name = "Urgency", eq, ord)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PyUrgency(Urgency);

#[pymethods]
impl PyUrgency {
    #[new]
    fn new(value: &str) -> PyResult<Self> {
        let urgency = match value.to_uppercase().as_str() {
            "LOW" => Urgency::LOW,
            "MEDIUM" => Urgency::MEDIUM,
            "HIGH" => Urgency::HIGH,
            "CRITICAL" => Urgency::CRITICAL,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Invalid urgency value",
                ));
            }
        };
        Ok(PyUrgency(urgency))
    }

    #[classattr]
    const LOW: PyUrgency = PyUrgency(Urgency::LOW);

    #[classattr]
    const MEDIUM: PyUrgency = PyUrgency(Urgency::MEDIUM);

    #[classattr]
    const HIGH: PyUrgency = PyUrgency(Urgency::HIGH);

    #[classattr]
    const CRITICAL: PyUrgency = PyUrgency(Urgency::CRITICAL);

    fn __str__(&self) -> String {
        format!("Urgency.{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Urgency('{:?}')", self.0)
    }

    fn __int__(&self) -> i32 {
        self.0 as i32
    }
}

#[pyclass(name = "Priority", eq, ord)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PyPriority(Priority);

#[pymethods]
impl PyPriority {
    #[new]
    fn new(value: &str) -> PyResult<Self> {
        let priority = match value.to_uppercase().as_str() {
            "LOW" => Priority::LOW,
            "MEDIUM" => Priority::MEDIUM,
            "HIGH" => Priority::HIGH,
            "CRITICAL" => Priority::CRITICAL,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Invalid priority value",
                ));
            }
        };
        Ok(PyPriority(priority))
    }

    #[classattr]
    const LOW: PyPriority = PyPriority(Priority::LOW);

    #[classattr]
    const MEDIUM: PyPriority = PyPriority(Priority::MEDIUM);

    #[classattr]
    const HIGH: PyPriority = PyPriority(Priority::HIGH);

    #[classattr]
    const CRITICAL: PyPriority = PyPriority(Priority::CRITICAL);

    fn __str__(&self) -> String {
        format!("Priority.{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("PyPriority('{:?}')", self.0)
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
        PyImpact(self.0.impact)
    }

    #[getter]
    fn priority(&self) -> PyPriority {
        PyPriority(self.0.priority)
    }

    #[getter]
    fn urgency(&self) -> PyUrgency {
        PyUrgency(self.0.urgency)
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
            self.0.correlation_id.map_or("None".to_string(), |id| id.to_string()),
            self.0.source.system,
            self.0.source.source_id.as_ref().map_or("None", |s| s.as_str()),
            self.0.impact,
            self.0.priority,
            self.0.urgency,
            self.0.received_at.to_rfc3339(),
            self.0.created_at.to_rfc3339(),
            self.0.resolved_at.map_or("None".to_string(), |dt| dt.to_rfc3339()),
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
    fn new(system: String, source_id: Option<String>) -> Self {
        PyEventBuilder(EventBuilder::new(Source { system, source_id }))
    }

    fn with_correlation_id(mut self_: PyRefMut<'_, Self>, correlation_id: String) -> PyRefMut<'_, Self> {
        self_.0.with_correlation_id(correlation_id.parse().unwrap());
        self_
    }

    fn with_priority(mut self_: PyRefMut<'_, Self>, priority: PyPriority) -> PyRefMut<'_, Self> {
        self_.0.with_priority(priority.0);
        self_
    }

    fn with_impact(mut self_: PyRefMut<'_, Self>, impact: PyImpact) -> PyRefMut<'_, Self> {
        self_.0.with_impact(impact.0);
        self_
    }

    fn with_urgency(mut self_: PyRefMut<'_, Self>, urgency: PyUrgency) -> PyRefMut<'_, Self> {
        self_.0.with_urgency(urgency.0);
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