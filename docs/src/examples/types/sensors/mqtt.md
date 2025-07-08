# MQTT

```yaml
version: 1
title: "My MQTT Sensor Example"
key: "my_mqtt_sensor_example"
description: ""

sensor:
    type: mqtt
    host: localhost
    port: 1883
    client_id: "string"
    username: "string"
    password: "string"
    topics:
        - "sensors/temperature"
        - "sensors/humidity"
    qos: 1
    keep_alive: 60
```