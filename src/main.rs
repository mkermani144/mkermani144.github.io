use leptos_axum::generate_route_list_with_ssg;
use leptos_config::get_configuration;
use std::process::Command;

use personal_website::app::shell;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = get_configuration(Some("Cargo.toml"))?;
    let leptos_options = conf.leptos_options;
    let shell_options = leptos_options.clone();
    let styles_out = format!("{}/styles.css", leptos_options.site_root);

    // Build Tailwind CSS as part of SSG so `cargo run` is the only build command.
    let status = Command::new("npx")
        .args([
            "@tailwindcss/cli",
            "-i",
            "style/tailwind.css",
            "-o",
            &styles_out,
            "--minify",
        ])
        .status()?;
    if !status.success() {
        return Err("tailwind build failed".into());
    }

    let (_routes, static_generator) =
        generate_route_list_with_ssg(move || shell(shell_options.clone()));
    static_generator.generate(&leptos_options).await;

    Ok(())
}
