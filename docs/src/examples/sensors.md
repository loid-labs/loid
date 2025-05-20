# Sensors

## Generate

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

## File

```yaml
version: 1
title: "My File Sensor Example"
key: "my_file_sensor_example"
description: ""

sensor:
  type: file
  path: /path/to/file
  tail: true
```

## Kafka

```yaml
version: 1
title: "My Kafka Sensor Example"
key: "my_kafka_sensor_example"
description: ""

sensor:
  type: kafka
  brokers:
    - localhost:9092
  topics:
    - test-topic
  consumer_group: test-group
  client_id: test_client
  start_from_latest: true
```

## MQTT

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

## SQL

```yaml
version: 1
title: "My SQL Sensor Example"
key: "my_sql_sensor_example"
description: ""

sensor:
  type: sql
  statement: "SELECT * FROM table"
  connection:
    mariadb:
      uri: "mariadb://user:password@localhost:3306/db"

  interval: 15m
```

## HTTP Client

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

## HTTP Server

```yaml
version: 1
title: "My HTTP Server Sensor Example"
key: "my_http_server_sensor_example"
description: "I am an http bin you can send json data to and I put them on the queue"

sensor:
  type: http
  path: "/my/prometheus/alerts"
```
