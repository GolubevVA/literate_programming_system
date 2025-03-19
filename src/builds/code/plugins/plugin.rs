#![forbid(unsafe_code)]

use crate::error::LPError;
use mlua::{Function, Lua};
use std::{path::PathBuf, sync::Arc};

/// Get the plugin functions.
/// 
/// Compiles the plugin code and gets the functions with the given names.
/// 
/// # Arguments
/// * `lua` - the Lua instance.
/// * `plugin_path` - the path to the plugin.
/// * `func_names` - the names of the functions to get.
/// # Returns
/// Returns either a vector of functions in the same order as the given list of their names or an LPError.
pub fn get_plugin_funcs(
    lua: &Arc<Lua>,
    plugin_path: &PathBuf,
    func_names: Vec<&str>,
) -> Result<Vec<Function>, LPError> {
    let code = std::fs::read_to_string(&plugin_path)
        .map_err(|_| LPError::CannotReadFile(plugin_path.display().to_string()))?;

    lua.load(&code)
        .set_name(plugin_path.to_string_lossy().as_ref())
        .exec()
        .map_err(|e| LPError::LuaRuntime(e.to_string()))?;

    let mut funcs = Vec::new();

    for func_name in func_names.iter() {
        let func: Function = lua
            .globals()
            .get(*func_name)
            .map_err(|e| LPError::LuaRuntime(format!("No {} function: {}", func_name, e)))?;

        funcs.push(func);
    }

    Ok(funcs)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file(content: &str) -> PathBuf {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes()).expect("Failed to write to temp file");
        let path = file.path().to_path_buf();

        std::mem::forget(file);
        path
    }

    #[test]
    fn test_get_plugin_funcs_success() {
        let lua = Arc::new(Lua::new());
        
        let lua_code = r#"
        function test_function()
            return "hello world"
        end

        function another_function(arg)
            return arg * 2
        end
        "#;
        
        let plugin_path = create_temp_file(lua_code);
        
        let result = get_plugin_funcs(&lua, &plugin_path, vec!["test_function", "another_function"]);
        assert!(result.is_ok());
        
        let funcs = result.unwrap();
        assert_eq!(funcs.len(), 2);
        
        std::fs::remove_file(plugin_path).expect("Failed to remove temp file");
    }

    #[test]
    fn test_get_plugin_funcs_nonexistent_file() {
        let lua = Arc::new(Lua::new());
        let plugin_path = PathBuf::from("/nonexistent/file.lua");
        
        let result = get_plugin_funcs(&lua, &plugin_path, vec!["test_function"]);
        assert!(result.is_err());
        
        match result {
            Err(LPError::CannotReadFile(_)) => {},
            _ => panic!("Expected CannotReadFile error"),
        }
    }

    #[test]
    fn test_get_plugin_funcs_invalid_lua() {
        let lua = Arc::new(Lua::new());
        
        let lua_code = r#"
        garbage
        "#;
        
        let plugin_path = create_temp_file(lua_code);
        
        let result = get_plugin_funcs(&lua, &plugin_path, vec!["test_function"]);
        assert!(result.is_err());
        
        match result {
            Err(LPError::LuaRuntime(_)) => {},
            _ => panic!("Expected LuaRuntime error"),
        }
        
        std::fs::remove_file(plugin_path).expect("Failed to remove temp file");
    }

    #[test]
    fn test_get_plugin_funcs_missing_function() {
        let lua = Arc::new(Lua::new());
        
        let lua_code = r#"
        function test_function()
            return "hello world"
        end
        "#;
        
        let plugin_path = create_temp_file(lua_code);
        
        let result = get_plugin_funcs(&lua, &plugin_path, vec!["nonexistent_function"]);
        assert!(result.is_err());
        
        match result {
            Err(LPError::LuaRuntime(_)) => {},
            _ => panic!("Expected LuaRuntime error"),
        }
        
        std::fs::remove_file(plugin_path).expect("Failed to remove temp file");
    }

    #[test]
    fn test_get_plugin_funcs_execute_functions() {
        let lua = Arc::new(Lua::new());
        
        let lua_code = r#"
        function add(a, b)
            return a + b
        end

        function concat(a, b)
            return a .. b
        end
        "#;
        
        let plugin_path = create_temp_file(lua_code);
        
        let result = get_plugin_funcs(&lua, &plugin_path, vec!["add", "concat"]);
        assert!(result.is_ok());
        
        let funcs = result.unwrap();
        assert_eq!(funcs.len(), 2);
        
        let add_result: i32 = funcs[0].call((5, 7)).expect("Failed to call add function");
        assert_eq!(add_result, 12);
        
        let concat_result: String = funcs[1].call(("hello", " world")).expect("Failed to call concat function");
        assert_eq!(concat_result, "hello world");
        
        std::fs::remove_file(plugin_path).expect("Failed to remove temp file");
    }

    #[test]
    fn test_get_plugin_funcs_empty_function_list() {
        let lua = Arc::new(Lua::new());
        
        let lua_code = r#"
        function test_function()
            return "hello world"
        end
        "#;
        
        let plugin_path = create_temp_file(lua_code);
        
        let result = get_plugin_funcs(&lua, &plugin_path, vec![]);
        assert!(result.is_ok());
        
        let funcs = result.unwrap();
        assert_eq!(funcs.len(), 0);
        
        std::fs::remove_file(plugin_path).expect("Failed to remove temp file");
    }
}

