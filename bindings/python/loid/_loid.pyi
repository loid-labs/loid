from typing import Self

__version__: str


class Impact:
    NEGLIGIBLE: Impact
    MINOR: Impact
    MODERATE: Impact
    SEVERE: Impact
    SIGNIFICANT: Impact


class Urgency:
    CRITICAL: Urgency
    HIGH: Urgency
    LOW: Urgency
    MEDIUM: Urgency


class Priority:
    CRITICAL: Priority
    HIGH: Priority
    LOW: Priority
    MEDIUM: Priority


class EventBuilder:
    def build(self) -> Event:
        ...

    def with_correlation_id(self, correlation_id: str) -> Self:
        ...

    def with_impact(self, impact: Impact) -> Self:
        ...

    def with_priority(self, priority: Priority) -> Self:
        ...

    def with_urgency(self, urgency: Urgency) -> Self:
        ...


class Event:
    impact: Impact
    urgency: Urgency
    priority: Priority
    correlation_id: str
