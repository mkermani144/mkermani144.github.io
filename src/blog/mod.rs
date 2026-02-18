use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
    pub date: String,
    pub summary: String,
    pub slug: String,
    pub html: String,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String,
    date: String,
    summary: String,
    slug: String,
}

static POSTS: OnceLock<Vec<Post>> = OnceLock::new();

pub fn all_posts() -> &'static [Post] {
    posts()
}

pub fn by_slug(slug: &str) -> Option<&'static Post> {
    posts().iter().find(|post| post.slug == slug)
}

pub fn all_slugs() -> Vec<String> {
    posts().iter().map(|post| post.slug.clone()).collect()
}

fn posts() -> &'static Vec<Post> {
    POSTS.get_or_init(|| {
        load_posts().unwrap_or_else(|err| panic!("failed to load blog posts: {err}"))
    })
}

fn load_posts() -> Result<Vec<Post>, String> {
    let blog_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("blog");
    if !blog_dir.exists() {
        return Ok(Vec::new());
    }

    let mut posts = Vec::new();
    for entry in fs::read_dir(&blog_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("md")) {
            continue;
        }
        if let Some(post) = parse_post_file(&path)? {
            posts.push(post);
        }
    }

    posts.sort_by(|a, b| {
        b.timestamp
            .cmp(&a.timestamp)
            .then_with(|| a.slug.cmp(&b.slug))
    });
    validate_unique_slugs(&posts)?;

    Ok(posts)
}

fn parse_post_file(path: &Path) -> Result<Option<Post>, String> {
    let file_name = path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| format!("invalid post filename: {}", path.display()))?;
    let Some(timestamp) = parse_timestamp(file_name)? else {
        return Ok(None);
    };
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let (front_matter_raw, markdown_body) = split_front_matter(&content, path)?;
    let front_matter: FrontMatter =
        serde_yaml::from_str(front_matter_raw).map_err(|e| e.to_string())?;
    validate_required_fields(path, &front_matter)?;
    let markdown_body = strip_leading_title_heading(markdown_body, &front_matter.title);

    Ok(Some(Post {
        title: front_matter.title,
        date: front_matter.date,
        summary: front_matter.summary,
        slug: front_matter.slug,
        html: markdown_to_html(markdown_body),
        timestamp,
    }))
}

fn parse_timestamp(file_name: &str) -> Result<Option<u64>, String> {
    let stem = file_name
        .strip_suffix(".md")
        .ok_or_else(|| format!("post filename must end with .md: {file_name}"))?;
    let Some((prefix, _)) = stem.split_once('-') else {
        return Ok(None);
    };
    match prefix.parse::<u64>() {
        Ok(timestamp) => Ok(Some(timestamp)),
        Err(_) => Ok(None),
    }
}

fn split_front_matter<'a>(content: &'a str, path: &Path) -> Result<(&'a str, &'a str), String> {
    let start = if content.starts_with("---\n") {
        4
    } else if content.starts_with("---\r\n") {
        5
    } else {
        return Err(format!(
            "missing front matter start delimiter in {}",
            path.display()
        ));
    };

    let remaining = &content[start..];
    let mut scanned = 0;
    let mut end = None;
    let mut body_start = None;

    for line in remaining.split_inclusive('\n') {
        let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
        if trimmed == "---" {
            end = Some(start + scanned);
            body_start = Some(start + scanned + line.len());
            break;
        }
        scanned += line.len();
    }

    let end =
        end.ok_or_else(|| format!("missing front matter end delimiter in {}", path.display()))?;
    let front_matter_raw = &content[start..end];
    let body_start = body_start.unwrap_or(content.len());
    let markdown_body = if body_start < content.len() {
        &content[body_start..]
    } else {
        ""
    };

    Ok((front_matter_raw, markdown_body))
}

fn validate_required_fields(path: &Path, fm: &FrontMatter) -> Result<(), String> {
    if fm.title.trim().is_empty() {
        return Err(format!(
            "missing required field `title` in {}",
            path.display()
        ));
    }
    if fm.date.trim().is_empty() {
        return Err(format!(
            "missing required field `date` in {}",
            path.display()
        ));
    }
    if fm.summary.trim().is_empty() {
        return Err(format!(
            "missing required field `summary` in {}",
            path.display()
        ));
    }
    if fm.slug.trim().is_empty() {
        return Err(format!(
            "missing required field `slug` in {}",
            path.display()
        ));
    }
    Ok(())
}

fn validate_unique_slugs(posts: &[Post]) -> Result<(), String> {
    let mut seen = std::collections::HashSet::new();
    for post in posts {
        if !seen.insert(post.slug.as_str()) {
            return Err(format!("duplicate slug detected: {}", post.slug));
        }
    }
    Ok(())
}

fn markdown_to_html(markdown: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn strip_leading_title_heading<'a>(markdown: &'a str, title: &str) -> &'a str {
    let mut offset = 0;
    for line in markdown.split_inclusive('\n') {
        let trimmed_line = line.trim_end_matches('\n').trim_end_matches('\r');
        if trimmed_line.trim().is_empty() {
            offset += line.len();
            continue;
        }

        let Some(heading_text) = parse_h1_text(trimmed_line) else {
            return markdown;
        };
        if heading_text != title.trim() {
            return markdown;
        }

        offset += line.len();
        let rest = &markdown[offset..];
        let mut extra = 0;
        for candidate in rest.split_inclusive('\n') {
            let trimmed = candidate.trim_end_matches('\n').trim_end_matches('\r');
            if trimmed.trim().is_empty() {
                extra += candidate.len();
                continue;
            }
            break;
        }
        return &markdown[offset + extra..];
    }
    markdown
}

fn parse_h1_text(line: &str) -> Option<&str> {
    let after_hash = line.strip_prefix('#')?;
    if after_hash.starts_with('#') {
        return None;
    }
    let heading_text = after_hash.trim_start().trim_end_matches('#').trim_end();
    if heading_text.is_empty() {
        return None;
    }
    Some(heading_text)
}

#[cfg(test)]
mod tests {
    use super::{markdown_to_html, parse_timestamp, strip_leading_title_heading};

    #[test]
    fn parses_published_filename_timestamp() {
        assert_eq!(
            parse_timestamp("1735689600-my-post.md"),
            Ok(Some(1_735_689_600))
        );
    }

    #[test]
    fn treats_non_numeric_prefix_as_draft() {
        assert_eq!(parse_timestamp("draft-my-post.md"), Ok(None));
    }

    #[test]
    fn treats_missing_prefix_separator_as_draft() {
        assert_eq!(parse_timestamp("my-post.md"), Ok(None));
    }

    #[test]
    fn errors_on_non_markdown_filename() {
        assert!(parse_timestamp("1735689600-my-post.txt").is_err());
    }

    #[test]
    fn renders_markdown_headings_and_lists() {
        let html = markdown_to_html("# Title\n\n- One\n- Two\n");
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>One</li>"));
        assert!(html.contains("<li>Two</li>"));
    }

    #[test]
    fn strips_leading_h1_when_it_matches_title() {
        let markdown = "\n# Hello World\n\nSome content.";
        let stripped = strip_leading_title_heading(markdown, "Hello World");
        assert_eq!(stripped, "Some content.");
    }

    #[test]
    fn keeps_leading_h1_when_it_does_not_match_title() {
        let markdown = "# Different Title\n\nSome content.";
        let stripped = strip_leading_title_heading(markdown, "Hello World");
        assert_eq!(stripped, markdown);
    }
}
