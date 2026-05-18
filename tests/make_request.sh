#!/bin/sh
curl -X POST http:/0.0.0.0:3000/check_conflicts \
        2 -H 'content-type: application/json' -d \
        '{
"msg": "hello"
}'
