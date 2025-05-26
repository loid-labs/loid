# Neurons

## Server Status Check

```yaml
version: 1
name: "Server Status Check"
description: "Determines if a server is reachable on the network"

activation:
    goals:
        - verify_server_status
    conditions:
        server_ip:  # when nothing is defined, this means the field must exist and have a value

execution:
    uses: bash
    timeout: 10
    cmd: [ "ping", "-c", "4", ${ server_ip } ]
    stdout:
        -   $regex: '(?s).*?(?<server_status>(\d+)%)\spacket\sloss'
            transform:
                $map:
                    server_status:
                        "0%": "up"
                        ".*": "down"

    goals:
        # Always fulfill the verification goal
        -   action: fulfill
            goal: verify_server_status
            condition: true
        
        # Set appropriate next goal based on result
        -   action: set
            goal: restart_server
            condition: "${server_status == "
            priority: 20
            metadata:
                reason: "Server ${server_ip} is unreachable, needs to be restored"

        -   action: set
            goal: resolve_alert
            condition: "${server_status == "
            priority: 30
            context:
                $set:
                    new_alert_status: "resolved"
            metadata:
                reason: "Server ${server_ip} is reachable, alert can be resolved"
```