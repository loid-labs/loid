# Kafka

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