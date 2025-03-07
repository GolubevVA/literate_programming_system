#![forbid(unsafe_code)]

use crate::error::LPError;
use mlua::{Function, Lua};
use std::{path::PathBuf, sync::Arc};

pub fn get_plugin_func(
    lua: &Arc<Lua>,
    plugin_path: &PathBuf,
    func_name: &str,
) -> Result<Function, LPError> {
    let code = std::fs::read_to_string(&plugin_path)
        .map_err(|_| LPError::CannotReadFile(plugin_path.display().to_string()))?;

    lua.load(&code)
        .set_name(plugin_path.to_string_lossy().as_ref())
        .exec()
        .map_err(|e| LPError::LuaRuntime(e.to_string()))?;

    let func: Function = lua
        .globals()
        .get(func_name)
        .map_err(|e| LPError::LuaRuntime(format!("No {} function: {}", func_name, e)))?;

    Ok(func)
}
