help:
  @just --list

dev:
  #!/usr/bin/env bash
  command -v cargo-watch >/dev/null 2>&1 || cargo install cargo-watch
  mkdir -p target/site
  (cd target/site && python3 -m http.server 8000) &
  server_pid=$!
  trap 'kill "$server_pid" 2>/dev/null || true' EXIT INT TERM
  cargo watch -x run || [ "$?" -eq 130 ]

newblog +blog_title:
  #!/usr/bin/env bash
  set -euo pipefail

  title="{{blog_title}}"
  if [ -z "${title// }" ]; then
    echo "Usage: just newblog blog_title"
    exit 1
  fi

  slug="$(printf '%s' "$title" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//; s/-+/-/g')"

  if [ -z "$slug" ]; then
    echo "Title must contain at least one letter or number."
    exit 1
  fi

  timestamp="$(date +%s)"
  mkdir -p blog
  post_path="blog/${timestamp}-${slug}.md"
  post_date="$(date -u +%Y-%m-%d)"

  printf '%s\n' \
    '---' \
    "title: \"$title\"" \
    "date: \"$post_date\"" \
    'summary: "TODO: add summary."' \
    "slug: \"$slug\"" \
    '---' \
    '' \
    "# $title" \
    '' \
    'Start writing here.' \
    > "$post_path"

  echo "Created $post_path"
