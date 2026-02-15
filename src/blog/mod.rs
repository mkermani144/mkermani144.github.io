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
        posts.push(parse_post_file(&path)?);
    }

    posts.sort_by(|a, b| {
        b.timestamp
            .cmp(&a.timestamp)
            .then_with(|| a.slug.cmp(&b.slug))
    });
    validate_unique_slugs(&posts)?;

    Ok(posts)
}

fn parse_post_file(path: &Path) -> Result<Post, String> {
    let file_name = path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| format!("invalid post filename: {}", path.display()))?;
    let timestamp = parse_timestamp(file_name)?;
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let (front_matter_raw, markdown_body) = split_front_matter(&content, path)?;
    let front_matter: FrontMatter =
        serde_yaml::from_str(front_matter_raw).map_err(|e| e.to_string())?;
    validate_required_fields(path, &front_matter)?;

    Ok(Post {
        title: front_matter.title,
        date: front_matter.date,
        summary: front_matter.summary,
        slug: front_matter.slug,
        html: markdown_to_html(markdown_body),
        timestamp,
    })
}

fn parse_timestamp(file_name: &str) -> Result<u64, String> {
    let stem = file_name
        .strip_suffix(".md")
        .ok_or_else(|| format!("post filename must end with .md: {file_name}"))?;
    let (prefix, _) = stem.split_once('-').ok_or_else(|| {
        format!("post filename must start with unix timestamp prefix: {file_name}")
    })?;
    prefix
        .parse::<u64>()
        .map_err(|_| format!("invalid unix timestamp prefix in filename: {file_name}"))
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
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
