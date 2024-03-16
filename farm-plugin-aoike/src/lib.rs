#![deny(clippy::all)]

use std::fs;

use farmfe_core::{config::Config, plugin::Plugin};

use farmfe_macro_plugin::farm_plugin;

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

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.module_id.ends_with(".md") {
      println!(
        "load markdown file: {:?}, id: {:?}",
        param.resolved_path, param.module_id
      );
      let content = fs::read_to_string(&param.resolved_path).unwrap();
      Ok(Some(farmfe_core::plugin::PluginLoadHookResult {
        module_type: farmfe_core::module::ModuleType::Custom("md".to_string()),
        content,
        source_map: None,
      }))
    } else {
      println!(
        "load path: {:?}, id: {:?}",
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
          r#"import React from "react";
          export default function Content() {{
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
