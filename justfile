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
  shopt -s nullglob

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

  mkdir -p blog
  base_slug="$slug"
  suffix=2
  while :; do
    matches=(blog/*-"${slug}".md)
    if [ ! -e "blog/draft-${slug}.md" ] && [ "${#matches[@]}" -eq 0 ]; then
      break
    fi
    slug="${base_slug}-${suffix}"
    suffix=$((suffix + 1))
  done
  post_path="blog/draft-${slug}.md"
  post_date="$(date +%Y-%m-%d)"

  printf '%s\n' \
    '---' \
    "title: \"$title\"" \
    "date: \"$post_date\"" \
    'summary: "TODO: add summary."' \
    "slug: \"$slug\"" \
    '---' \
    '' \
    'Start writing here.' \
    > "$post_path"

  if [ "$slug" != "$base_slug" ]; then
    echo "Slug '$base_slug' already existed; using '$slug'."
  fi
  echo "Created $post_path"

publish:
  #!/usr/bin/env bash
  set -euo pipefail
  bash scripts/publish_blog.sh
