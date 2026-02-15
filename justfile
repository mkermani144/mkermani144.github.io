dev:
  #!/usr/bin/env bash
  command -v cargo-watch >/dev/null 2>&1 || cargo install cargo-watch
  mkdir -p target/site
  (cd target/site && python3 -m http.server 8000) &
  server_pid=$!
  trap 'kill "$server_pid" 2>/dev/null || true' EXIT INT TERM
  cargo watch -x run || [ "$?" -eq 130 ]
