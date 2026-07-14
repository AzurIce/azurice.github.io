use aoike::build::{Entity, post::Post, utils::patch_file};

fn main() {
    println!("cargo:rerun-if-changed=docs");
    println!("cargo:rerun-if-changed=static/gallery");

    aoike_sycamore::build::init_aoike_sycamore();

    // Parse markdown files to HTML using aoike-build
    let posts = aoike::build::parse_posts("docs/posts");
    let index = Entity::new("docs/index.md");
    let index = Post::try_from(index).unwrap();

    // Parse gallery directory
    let gallery = aoike::build::gallery::parse_gallery("static/gallery");

    let assets = aoike::build::get_assets_trunk_data(&posts, &index, "docs");
    patch_file(
        "index.html",
        &assets,
        "AOIKE_SYCAMORE_SITE_ASSETS",
        Some("</head>"),
    )
    .unwrap();
    let out_dir = std::env::current_dir().unwrap().join("src");
    let code = std::fs::read_to_string(out_dir.join("docsgen.rs")).unwrap_or(String::new());
    let mut gen_code = aoike::build::generate_code(posts, index);
    gen_code.push_str(&aoike::build::gallery::generate_gallery_code(gallery));
    if code != gen_code {
        std::fs::write(out_dir.join("docsgen.rs"), gen_code).unwrap();
    }
}
