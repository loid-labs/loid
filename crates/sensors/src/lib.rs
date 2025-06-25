use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
pub enum LifecycleState {
    /// Sensor is in the process of starting up
    Starting,
    /// Sensor is actively running and scheduled for collection
    Running,
    /// Sensor is in the process of shutting down
    Stopping,
    /// Sensor is not running and not scheduled to run
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
pub enum HealthState {
    /// Operating normally, collecting data successfully
    Healthy,
    /// Operating but with reduced performance or intermittent issues
    Degraded,
    /// Not collecting data, experiencing failures
    Failed,
    /// Cannot determine health status
    Unknown,
    /// External resource is unavailable
    ResourceDown,
    /// Configuration is invalid or incomplete
    Misconfigured,
    /// Temporarily throttled to protect resources
    Throttled,
    /// Waiting for external conditions (e.g., scheduled time)
    Waiting,
}

// Better approach: Separate concerns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorStatus {
    pub lifecycle: LifecycleState,
    pub health: HealthState,
    pub last_update: DateTime<Utc>,
}

trait Sensor {}

#[cfg(test)]
mod tests {}
