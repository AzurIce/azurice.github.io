#![deny(clippy::all)]

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

  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    println!(
      "load path: {:?}, id: {:?}",
      param.resolved_path, param.module_id
    );
    Ok(None)
  }
}
