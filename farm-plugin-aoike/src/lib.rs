#![deny(clippy::all)]

use std::{collections::HashMap, fs, path::PathBuf};

use farmfe_core::{config::Config, plugin::Plugin, relative_path::PathExt};

use farmfe_macro_plugin::farm_plugin;

#[cfg(test)]
mod test {
  use std::path::PathBuf;

  #[test]
  fn test_path() {
    let path = PathBuf::from("./");
    println!("{:?}", path);
    let path = path.join("docs");
    println!("{:?}", path);
    // join "index.html" to path using unix style
  }

  #[test]
  fn test_get_route() {
    let root_dir = PathBuf::from("F:\\azurice.github.io\\aoike\\src\\docs");
    let dir = PathBuf::from("F:\\azurice.github.io\\aoike\\src\\docs");
    let route = super::get_route(root_dir, dir);
    println!("{}", route);
  }
}

fn get_route(root_dir: PathBuf, dir: PathBuf) -> String {
  println!("reading dir: {:?}...", dir);
  // index of this dir
  let mut routes = vec![];

  let get_path = |root_dir: &PathBuf, filepath: &PathBuf| {
    let rel_path = filepath
      .canonicalize()
      .unwrap()
      .relative_to(&root_dir.canonicalize().unwrap())
      .unwrap();

    let rel_path_without_ext = rel_path.with_extension("").to_string().replace("\\", "/");
    let import_path = PathBuf::from("./docs/").join(rel_path.to_string());
    let import_path = import_path.to_str().unwrap().replace("\\", "/");
    (rel_path_without_ext, import_path)
  };

  // Add pages in this dir and sub dirs
  for entry in fs::read_dir(&dir).unwrap() {
    let entry = entry.unwrap();

    let path = entry.path();
    if path.is_file() {
      if path.file_name() == Some("index.md".as_ref()) {
        continue;
      }

      if let Some("md") = path.extension().map(|s| s.to_str().unwrap()) {
        let (path, import_path) = get_path(&root_dir, &path);
        routes.push(format!(
          r#"{{
            path: "/{path}",
            component: lazy(() => import("{import_path}")),
          }}"#
        ));
      }
    } else {
      routes.push(get_route(root_dir.clone(), path));
    }
  }

  let rel_dir = dir
    .canonicalize()
    .unwrap()
    .relative_to(root_dir.canonicalize().unwrap())
    .unwrap()
    .to_string()
    .replace("\\", "/");
  let rel_import_dir = PathBuf::from("./docs/").join(&rel_dir).join("index.md");
  let rel_import_dir = rel_import_dir.to_str().unwrap().replace("\\", "/");

  if root_dir == dir {
    format!(
      r#"import {{ lazy }} from "solid-js";
import {{ Router }} from "@solidjs/router";
const routes = [{{
  path: "/{rel_dir}",
  component: lazy(() => import("{rel_import_dir}")),
  children: [{}]
}}]
export default function AoikeRouter() {{
  return <Router>{{routes}}</Router>
}};"#,
      routes.join(",\n")
    )
  } else {
    format!(
      r#"{{
      path: "/{rel_dir}",
      component: lazy(() => import("{rel_import_dir}")),
      children: [{}]
    }}"#,
      routes.join(",\n")
    )
  }
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
      let content = get_route(dir.clone(), dir);
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
