use aoike_sycamore::{
    components::giscus::{GiscusOptions, InputPosition},
    AoikeApp, ConfigContext,
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
                    giscus_options: Some(
                        GiscusOptions::new(
                            "AzurIce/azurice.github.io".to_string(),
                            "R_kgDOI7WMeQ".to_string(),
                            "DIC_kwDOI7WMec4CUE3s".to_string(),
                        )
                        .with_category("Giscus".to_string())
                        .with_reactions_enabled(true)
                        .with_lazy(true)
                        .with_input_position(InputPosition::Top),
                    ),
                    ..Default::default()
                },
                index=docsgen::index(),
                posts=docsgen::posts(),
            )
        }
    });
}
