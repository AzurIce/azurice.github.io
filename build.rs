use std::path::Path;

use aoike::build::{Entity, post::Post, utils::inject_str};

fn main() {
    println!("cargo:rerun-if-changed=docs");

    if !Path::new("static/css").exists() {
        aoike_sycamore::build::init_aoike_sycamore();
    }

    // Parse markdown files to HTML using aoike-build
    let posts = aoike::build::parse_posts("docs/posts");
    let index = Entity::new("docs/index.md");
    let index = Post::try_from(index).unwrap();

    let assets = aoike::build::get_assets_trunk_data(&posts, &index, "docs");
    let index_html = std::fs::read_to_string("index.html").unwrap();
    let injected_index_html = inject_str(
        &index_html,
        &assets,
        "AOIKE_SYCAMORE_SITE_ASSETS",
        Some("</head>"),
    );
    if index_html != injected_index_html {
        std::fs::write("index.html", injected_index_html).unwrap();
    }
    let out_dir = std::env::current_dir().unwrap().join("src");
    let code = std::fs::read_to_string(out_dir.join("docsgen.rs")).unwrap_or(String::new());
    let gen_code = aoike::build::generate_code(posts, index);
    if code != gen_code {
        std::fs::write(out_dir.join("docsgen.rs"), gen_code).unwrap();
    }
}
