#![deny(clippy::all)]

use std::{
  collections::HashMap,
  fmt::{Display, Formatter},
  fs,
  path::{self, PathBuf},
};

use farmfe_core::{
  config::Config,
  plugin::Plugin,
  relative_path::{PathExt, RelativePathBuf},
};

use farmfe_macro_plugin::farm_plugin;

struct Routes {}

#[derive(Debug)]
struct Route {
  root_path: PathBuf,
  entry: RouteEntry,
}

impl Route {
  fn new(root_path: PathBuf, entry: RouteEntry) -> Self {
    Self { root_path, entry }
  }
}

#[derive(Debug)]
enum RouteEntry {
  File(PathBuf),
  Nested(PathBuf, Vec<Route>),
}

impl Route {
  fn get_id(&self) -> String {
    let path = match &self.entry {
      RouteEntry::File(path) => path,
      RouteEntry::Nested(path, _entries) => path,
    };
    path
      .canonicalize()
      .unwrap()
      .relative_to(self.root_path.canonicalize().unwrap())
      .unwrap()
      .with_extension("")
      .to_string()
      .replace("\\", "/")
  }

  fn get_import_path(&self) -> String {
    let path = match &self.entry {
      RouteEntry::File(path) => path.clone(),
      RouteEntry::Nested(path, _entries) => path.join("index.md"),
    };
    let docs_index = path
      .components()
      .position(|c| c.as_os_str() == "docs")
      .unwrap();
    let path = path.components().skip(docs_index + 1).collect::<PathBuf>();
    let path = PathBuf::from("./docs").join(path);
    path.to_str().unwrap().to_string().replace("\\", "/")
  }
}

impl Route {
  fn parse(&self) -> String {
    match &self.entry {
      RouteEntry::File(page) => format!(
        r#"<Route path="/{}" component={{lazy(() => import("{}"))}} />"#,
        self.get_id(),
        self.get_import_path()
      ),
      RouteEntry::Nested(path, routes) => {
        let mut routes = routes.iter().map(|r| r.parse()).collect::<Vec<String>>();
        format!(
          r#"<Route path="/{}">
  <Route path="/" component={{lazy(() => import("{}"))}} />
  {}
</Route>"#,
          self.get_id(),
          self.get_import_path(),
          routes.join("\n")
        )
      }
    }
  }
}

#[cfg(test)]
mod test {
  use std::path::PathBuf;

  use crate::parse_route;

  #[test]
  fn test_get_route() {
    // let root_dir = PathBuf::from("F:\\azurice.github.io\\aoike\\src\\docs");
    let dir = PathBuf::from("F:\\azurice.github.io\\aoike\\src\\docs");
    let content = parse_route(dir);
    println!("{}", content);
    // let route = super::get_route(root_dir, dir);
    // println!("{:?}", route);
    // let route = route.iter().map(|r| r.parse()).collect::<Vec<String>>();
    // for r in route {
    //   println!("{}", r);
    // }
  }
}

fn get_route(root_dir: PathBuf, dir: PathBuf) -> Vec<Route> {
  println!("reading dir: {:?}...", dir);
  // index of this dir
  let mut routes = vec![];

  // Add pages in this dir and sub dirs
  for entry in fs::read_dir(&dir).unwrap() {
    let entry = entry.unwrap();

    let path = entry.path();
    if path.is_file() {
      if path.file_name() == Some("index.md".as_ref()) {
        continue;
      }
      if path.extension().map(|s| s.to_str().unwrap()) != Some("md") {
        continue;
      }

      routes.push(Route::new(dir.clone(), RouteEntry::File(path.clone())));
    } else {
      routes.push(Route::new(
        dir.clone(),
        RouteEntry::Nested(path.clone(), get_route(root_dir.clone(), path)),
      ))
    }
  }
  routes
}

fn parse_route(dir: PathBuf) -> String {
  let routes = get_route(dir.clone(), dir);
  format!(
    r#"import {{ lazy }} from "solid-js";
import {{ Router, Route }} from "@solidjs/router";

export default function AoikeRouter() {{
  return <Router>
    <Route path="/" component={{lazy(() => import("./docs/index.md"))}} />
    {}
  </Router>;
}}
"#,
    routes
      .into_iter()
      .map(|r| r.parse())
      .collect::<Vec<String>>()
      .join("\n")
  )
}

#[farm_plugin]
pub struct FarmPluginAoike {}

impl FarmPluginAoike {
  fn new(config: &Config, options: String) -> Self {
    println!("creating farm-plugin-aoike...");
    Self {}
  }
}

impl Plugin for FarmPluginAoike {
  fn name(&self) -> &str {
    "FarmPluginAoike"
  }

  fn priority(&self) -> i32 {
    200
  }

  fn resolve(
    &self,
    param: &farmfe_core::plugin::PluginResolveHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginResolveHookResult>> {
    // println!("resolve: {:?}", param.source);
    if param.source == "AoikeRouter.tsx" {
      println!("load AoikeRouter.tsx");
      return Ok(Some(farmfe_core::plugin::PluginResolveHookResult {
        resolved_path: "./src/AoikeRouter.tsx".to_string(),
        ..Default::default()
      }));
    }
    Ok(None)
  }

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    println!("load: {:?}, {:?}", param.resolved_path, param.module_id);
    if param.module_id.ends_with("AoikeRouter.tsx") {
      let dir = PathBuf::from("./src/docs");
      let content = parse_route(dir);
      println!("{:?}", content);

      return Ok(Some(farmfe_core::plugin::PluginLoadHookResult {
        module_type: farmfe_core::module::ModuleType::Tsx,
        content,
        source_map: None,
      }));
    }
    let page_id = match param.module_id.strip_prefix("src/docs/") {
      Some(s) => s,
      None => {
        return Ok(None);
      }
    };
    let file_path = PathBuf::from(param.resolved_path);

    if page_id.ends_with(".md") {
      let page_id = page_id.strip_suffix(".md").unwrap();

      println!("markdown file: {:?}, id: {:?}", file_path, page_id);

      let content = fs::read_to_string(&file_path).unwrap();

      Ok(Some(farmfe_core::plugin::PluginLoadHookResult {
        module_type: farmfe_core::module::ModuleType::Custom("md".to_string()),
        content,
        source_map: None,
      }))
    } else {
      println!(
        "unsupported file: {:?}, id: {:?}",
        param.resolved_path, param.module_id
      );
      Ok(None)
    }
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    println!(
      "transform: {:?}, {:?}",
      param.resolved_path, param.module_type
    );

    let parser = pulldown_cmark::Parser::new(&param.content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    if matches!(param.module_type, farmfe_core::module::ModuleType::Custom(ref s) if s == "md") {
      Ok(Some(farmfe_core::plugin::PluginTransformHookResult {
        module_type: Some(farmfe_core::module::ModuleType::Tsx),
        content: format!(
          r#"export default function Content() {{
            return (<>
              {}
            </>)
          }}"#,
          html_output
        ),
        ..Default::default()
      }))
    } else {
      Ok(None)
    }
  }
}
