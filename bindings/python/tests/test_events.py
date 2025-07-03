from loid import EventBuilder, Impact, Urgency, Priority


def test_default_event(event_builder: EventBuilder):
    event = event_builder.build()
    assert event.impact == Impact.NEGLIGIBLE


def test_custom_event(event_builder: EventBuilder):
    event = (event_builder
             .with_impact(Impact.SEVERE)
             .with_urgency(Urgency.MEDIUM)
             .with_priority(Priority.HIGH)
             .with_correlation_id("812aa279-4b83-4e40-9192-168c27cc4422")
             .build()
             )

    assert event.impact == Impact.SEVERE
    assert event.urgency == Urgency.MEDIUM
    assert event.priority == Priority.HIGH
    assert event.correlation_id == "812aa279-4b83-4e40-9192-168c27cc4422"
