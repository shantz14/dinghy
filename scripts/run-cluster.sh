#!/usr/bin/bash
mkdir -p ~/.dinghy
cat << 'EOF' > ~/.dinghy/config
API_ADDRESS: http://localhost:8080
EOF

./bin/apiserver 8080

