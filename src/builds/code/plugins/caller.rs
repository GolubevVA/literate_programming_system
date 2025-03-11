#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::error::LPError;
use mlua::{Function, Lua};

use super::plugin::get_plugin_func;

pub struct PluginsCaller {
    plugin_functions: HashMap<String, Function>,
}

const PLUGIN_EXTENSION: &str = "lua";
const PLUGIN_FUNC_NAME: &str = "get_import_code";

impl PluginsCaller {
    /// Scans the plugins directory (`dir`) for files named `*.lua` (without descending into subdirs).
    /// Each plugin file is loaded into a LuaPlugin. The name is set to the file's name without the extension.
    pub fn new(lua: Arc<Lua>, dir: &Path) -> Result<Self, LPError> {
        let mut plugin_functions = HashMap::new();

        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => {
                println!("No plugins found in {}", dir.display());
                return Ok(PluginsCaller { plugin_functions });
            }
        };

        for entry in entries {
            let entry = entry.map_err(|_| LPError::CannotReadFile(dir.display().to_string()))?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == PLUGIN_EXTENSION {
                        let filename = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        println!("Loading plugin: {}", filename);

                        let plugin_func = get_plugin_func(&lua, &path, PLUGIN_FUNC_NAME)?;

                        plugin_functions.insert(filename, plugin_func);
                    }
                }
            }
        }

        Ok(PluginsCaller { plugin_functions })
    }

    /// each plugin correspons to some files extension
    /// this function calls the plugin that corresponds to the given extension
    /// Other parameters are passed to the plugin's function
    ///
    /// `current_path` - path to the file to which an import statement should be added
    /// `referenced_path` - path to the file, the code from which should be imported
    /// Both of them should have the same extension
    /// `code_block` - the code block that should be imported
    pub fn call_plugin_func(
        &self,
        extension: &str,
        current_path: &PathBuf,
        referenced_path: &PathBuf,
        code_block: &str,
    ) -> Result<String, LPError> {
        if let Some(plugin_func) = self.plugin_functions.get(extension) {
            let current_path_str = current_path.to_string_lossy();
            let referenced_path_str = referenced_path.to_string_lossy();

            let result: String = plugin_func
                .call((
                    current_path_str.as_ref(),
                    referenced_path_str.as_ref(),
                    code_block,
                ))
                .map_err(|e| LPError::LuaRuntime(e.to_string()))?;

            Ok(result)
        } else {
            Err(LPError::PluginNotFound(extension.to_string()))
        }
    }
}
