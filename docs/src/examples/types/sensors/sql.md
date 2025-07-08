# SQL

```yaml
version: 1
title: "My SQL Sensor Example"
key: "my_sql_sensor_example"
description: ""

sensor:
    type: sql
    statement: "SELECT * FROM table"
    resource: "my-mariadb-server"
    interval: 15m
```