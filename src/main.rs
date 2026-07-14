use aoike_sycamore::{
    AoikeApp, ConfigContext,
    components::{CommentSystem, waline::WalineOptions},
};

mod docsgen;

use sycamore::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            AoikeApp(
                config=ConfigContext {
                    title: Some("冰弦のBlog".to_string()),
                    desc: Some("『看清世界的真相后仍热爱生活』".to_string()),
                    // author: Some("Azur冰弦".to_string()),
                    email: Some("973562770@qq.com".to_string()),
                    // favicon: Some(FAVICON),
                    avatar: Some("/static/avatar.jpg".to_string()),
                    github_owner: Some("AzurIce".to_string()),
                    github_repo: Some("azurice.github.io".to_string()),
                    bilibili_url: Some("https://space.bilibili.com/46452693".to_string()),
                    steam_url: Some("https://steamcommunity.com/id/AzurIce".to_string()),
                    // extra_head: Some(RsxFn::new(|| {
                    //     rsx! {
                    //         document::Link { rel: "stylesheet", href: MAIN_CSS }
                    //     }
                    // })),
                    comment_system: Some(CommentSystem::Waline(
                        WalineOptions::new(
                            "https://waline.azurice.top".to_string(),
                            "".to_string(),
                        )
                        .with_login(true)
                        .with_page_size(10),
                    )),
                    ..Default::default()
                },
                index=docsgen::index(),
                posts=docsgen::posts(),
                gallery=docsgen::gallery(),
            )
        }
    });
}
