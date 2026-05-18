#!/bin/sh
curl -X POST http:/0.0.0.0:3000/check_conflicts \
        2 -H 'content-type: application/json' -d \
        '{
"station_graph": [
 {"start": "Station West", "end": "Entry Signal West" },
 {"start": "Entry Signal West", "end": "Point 1" },
 {"start": "Point 1", "end": "Exit Signal West 1" },
 {"start": "Point 1", "end": "Exit Signal West 2" },
 {"start": "Exit Signal West 1", "end": "Exit Signal East 1" },
 {"start": "Exit Signal West 2", "end": "Exit Signal East 2" },
 {"start": "Exit Signal East 1", "end": "Point 2" },
 {"start": "Exit Signal East 2", "end": "Point 2" },
 {"start": "Point 2", "end": "Entry Signal East" },
 {"start": "Entry Signal East", "end": "Station East" }
],
 "routes": [
 {"start": "Entry Signal West", "end": "Exit Signal East 1",
"occupied": false },
 {"start": "Entry Signal West", "end": "Exit Signal East 2",
"occupied": false },
 {"start": "Exit Signal East 1", "end": "Station East", "occupied":
false },
 {"start": "Exit Signal East 2", "end": "Station East", "occupied":
false },
 {"start": "Entry Signal East", "end": "Exit Signal West 1",
"occupied": false },
 {"start": "Entry Signal East", "end": "Exit Signal West 2",
"occupied": false },
 {"start": "Exit Signal West 1", "end": "Station West", "occupied":
true },
 {"start": "Exit Signal West 2", "end": "Station West", "occupied":
false }
 ],
 "check_route": {"start": "Entry Signal West", "end": "Exit Signal East 2"}
}'
