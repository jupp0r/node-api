#!/bin/bash

# curl -O "https://raw.githubusercontent.com/nodejs/node/master/src/node_api.h"
# curl -O "https://raw.githubusercontent.com/nodejs/node/master/src/node_api_types.h"

# nodejs 8.7.0
curl -O "https://raw.githubusercontent.com/nodejs/node/dc4f1b981a888a2f39219d384b6d3eb593203383/src/node_api.h"
curl -O "https://raw.githubusercontent.com/nodejs/node/dc4f1b981a888a2f39219d384b6d3eb593203383/src/node_api_types.h"

bindgen node_api.h > src/lib.rs
