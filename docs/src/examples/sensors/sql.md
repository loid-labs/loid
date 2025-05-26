# SQL

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