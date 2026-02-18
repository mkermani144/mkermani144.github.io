#!/usr/bin/env bash
set -euo pipefail
shopt -s nullglob

mkdir -p blog
drafts=()
seen_slugs_file="$(mktemp)"
trap 'rm -f "$seen_slugs_file"' EXIT

for path in blog/*.md; do
  file="$(basename "$path")"
  stem="${file%.md}"
  if [[ "$stem" != *-* ]]; then
    drafts+=("$path")
    continue
  fi
  prefix="${stem%%-*}"
  if [[ ! "$prefix" =~ ^[0-9]+$ ]]; then
    drafts+=("$path")
    continue
  fi
  slug="${stem#*-}"
  if [ -n "$slug" ]; then
    printf '%s\n' "$slug" >> "$seen_slugs_file"
  fi
done

if [ "${#drafts[@]}" -eq 0 ]; then
  echo "No draft posts found."
  exit 0
fi

publish_date="$(date +%Y-%m-%d)"
next_timestamp="$(date +%s)"
published_count=0

for path in "${drafts[@]}"; do
  file="$(basename "$path")"
  stem="${file%.md}"
  slug="$(awk '
    BEGIN { in_front_matter = 0; delimiters = 0; found = 0 }
    NR == 1 && $0 == "---" {
      in_front_matter = 1
      delimiters = 1
      next
    }
    {
      if (in_front_matter && $0 == "---") {
        in_front_matter = 0
        delimiters = 2
        next
      }
      if (in_front_matter && !found && $0 ~ /^slug:[[:space:]]*/) {
        value = $0
        sub(/^slug:[[:space:]]*/, "", value)
        sub(/^[[:space:]]+/, "", value)
        sub(/[[:space:]]+$/, "", value)
        if (value ~ /^".*"$/) {
          sub(/^"/, "", value)
          sub(/"$/, "", value)
        }
        print value
        found = 1
      }
    }
    END {
      if (delimiters < 2 || !found) {
        exit 1
      }
    }
  ' "$path")" || {
    echo "Skipping $path (front matter missing or \`slug\` field missing)."
    continue
  }

  if [ -z "$slug" ]; then
    echo "Skipping $path (empty \`slug\` value)."
    continue
  fi

  if grep -Fxq "$slug" "$seen_slugs_file"; then
    echo "Skipping $path (slug '$slug' already published or duplicated)."
    continue
  fi

  tmp="$(mktemp)"
  if ! awk -v publish_date="$publish_date" '
    BEGIN { in_front_matter = 0; delimiters = 0; replaced = 0 }
    NR == 1 && $0 == "---" {
      in_front_matter = 1
      delimiters = 1
      print
      next
    }
    {
      if (in_front_matter && $0 == "---") {
        in_front_matter = 0
        delimiters = 2
        print
        next
      }
      if (delimiters == 1 && !replaced && $0 ~ /^date:[[:space:]]*/) {
        print "date: \"" publish_date "\""
        replaced = 1
        next
      }
      print
    }
    END {
      if (delimiters < 2 || !replaced) {
        exit 1
      }
    }
  ' "$path" > "$tmp"; then
    rm -f "$tmp"
    echo "Skipping $path (front matter missing or \`date\` field missing)."
    continue
  fi
  mv "$tmp" "$path"

  new_path="blog/${next_timestamp}-${slug}.md"
  while [ -e "$new_path" ]; do
    next_timestamp=$((next_timestamp + 1))
    new_path="blog/${next_timestamp}-${slug}.md"
  done

  mv "$path" "$new_path"
  printf '%s\n' "$slug" >> "$seen_slugs_file"
  echo "Published $path -> $new_path"
  published_count=$((published_count + 1))
  next_timestamp=$((next_timestamp + 1))
done

echo "Published $published_count post(s) with date $publish_date."
