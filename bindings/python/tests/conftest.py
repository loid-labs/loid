import pytest

from loid import EventBuilder


@pytest.fixture()
def event_builder() -> EventBuilder:
    return EventBuilder()
