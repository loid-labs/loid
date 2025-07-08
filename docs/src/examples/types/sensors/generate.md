# Generate

```yaml
version: 1
title: "My Generate Sensor Example"
key: "generate_coffee_event"
description: ""

sensor:
    type: generate
    interval: 9 * * * *
    event: '{"make": "coffee"}'
```