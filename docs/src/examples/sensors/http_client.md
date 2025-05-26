# HTTP Client

```yaml
version: 1
title: "My HTTP Client Sensor Example"
key: "my_http_client_sensor_example"
description: "I make requests to another http server and fetch events"

sensor:
    type: http
    address: 0.0.0.0:8080
    path: "/data"
    cors_enabled: true
    auth:
        type: "basic"
        username: "string"
        password: "string"
```
