# HTTP Server

```yaml
version: 1
title: "My HTTP Server Sensor Example"
key: "my_http_server_sensor_example"
description: "I am an http bin you can send json data to and I put them on the queue"

sensor:
    type: http
    path: "/my/prometheus/alerts"
```
