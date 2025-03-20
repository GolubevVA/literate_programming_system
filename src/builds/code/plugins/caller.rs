#![forbid(unsafe_code)]

use std::{collections::HashMap, fs, path::Path, rc::Rc};

use crate::error::LPError;
use mlua::{Function, Lua};

use super::plugin::get_plugin_funcs;

/// Calls the functions of the plugins.
pub struct PluginsCaller {
    plugin_import_functions: HashMap<String, Function>,
    plugin_cleaning_functions: HashMap<String, Function>,
}

const PLUGIN_EXTENSION: &str = "lua";
const PLUGIN_IMPORT_CODE_FUNC_NAME: &str = "get_import_code";
const PLUGIN_CLEANIONG_CODE_FUNC_NAME: &str = "clean_code";

impl PluginsCaller {
    /// Scans the plugins directory (`dir`) for files named `*.lua` (without descending into subdirs).
    /// Each plugin file is loaded, the necessary functions are extracted and stored in the struct.
    ///
    /// The `lua` parameter must stay alive as long as the `PluginsCaller` instance is used.
    pub fn new(lua: Rc<Lua>, dir: &Path) -> Result<Self, LPError> {
        let mut plugin_import_functions = HashMap::new();
        let mut plugin_cleaning_functions = HashMap::new();

        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => {
                println!("No plugins found in {}", dir.display());
                return Ok(PluginsCaller {
                    plugin_import_functions,
                    plugin_cleaning_functions,
                });
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

                        let plugin_funcs = get_plugin_funcs(
                            &lua,
                            &path,
                            vec![
                                PLUGIN_IMPORT_CODE_FUNC_NAME,
                                PLUGIN_CLEANIONG_CODE_FUNC_NAME,
                            ],
                        )?;

                        plugin_import_functions.insert(filename.clone(), plugin_funcs[0].clone());
                        plugin_cleaning_functions.insert(filename.clone(), plugin_funcs[1].clone());
                    }
                }
            }
        }

        Ok(PluginsCaller {
            plugin_import_functions,
            plugin_cleaning_functions,
        })
    }

    /// Each plugin correspons to some files extension.
    /// This function calls the function to import code of the plugin that corresponds to the given extension.
    /// Other parameters are passed to the plugin's function.
    ///
    /// `current_path` - path to the file to which an import statement should be added
    ///
    /// `referenced_path` - path to the file, the code from which should be imported
    ///
    /// Both of them should have the same extension
    ///
    /// `code_block` - the code block that should be imported
    pub fn call_plugin_import_func(
        &self,
        extension: &str,
        current_path: &Path,
        referenced_path: &Path,
        code_block: &str,
    ) -> Result<String, LPError> {
        if let Some(plugin_func) = self.plugin_import_functions.get(extension) {
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

    /// Each plugin correspons to some files extension.
    /// This function calls the function to clean code of the plugin that corresponds to the given extension.
    /// Other parameters are passed to the plugin's function.
    ///
    /// `extension` - extension of the file
    ///
    /// `code` - code to clean
    ///
    /// Returns cleaned code
    pub fn call_plugin_cleaning_func(
        &self,
        extension: &str,
        code: &str,
    ) -> Result<String, LPError> {
        if let Some(plugin_func) = self.plugin_cleaning_functions.get(extension) {
            let result: String = plugin_func
                .call(code)
                .map_err(|e| LPError::LuaRuntime(e.to_string()))?;

            Ok(result)
        } else {
            Err(LPError::PluginNotFound(extension.to_string()))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_temp_plugin(dir: &Path, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.join(format!("{}.lua", filename));
        let mut file = std::fs::File::create(&file_path).expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");
        file_path
    }

    #[test]
    fn test_new_with_valid_plugins() {
        let lua = Rc::new(Lua::new());
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let plugin_code = r#"
        function get_import_code(current_path, referenced_path, code_block)
            return "import " .. code_block
        end

        function clean_code(code)
            return "cleaned " .. code
        end
        "#;

        create_temp_plugin(temp_dir.path(), "test_plugin", plugin_code);

        let result = PluginsCaller::new(Rc::clone(&lua), temp_dir.path());
        assert!(result.is_ok());

        let caller = result.unwrap();
        assert_eq!(caller.plugin_import_functions.len(), 1);
        assert_eq!(caller.plugin_cleaning_functions.len(), 1);
        assert!(caller.plugin_import_functions.contains_key("test_plugin"));
        assert!(caller.plugin_cleaning_functions.contains_key("test_plugin"));
    }

    #[test]
    fn test_new_with_empty_directory() {
        let lua = Rc::new(Lua::new());
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let result = PluginsCaller::new(Rc::clone(&lua), temp_dir.path());
        assert!(result.is_ok());

        let caller = result.unwrap();
        assert_eq!(caller.plugin_import_functions.len(), 0);
        assert_eq!(caller.plugin_cleaning_functions.len(), 0);
    }

    #[test]
    fn test_new_with_invalid_plugin() {
        let lua = Rc::new(Lua::new());
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let invalid_plugin_code = "garbage code that will fail";
        create_temp_plugin(temp_dir.path(), "invalid_plugin", invalid_plugin_code);

        let result = PluginsCaller::new(Rc::clone(&lua), temp_dir.path());
        assert!(result.is_err());

        match result {
            Err(LPError::LuaRuntime(_)) => {}
            _ => panic!("Expected LuaRuntime error"),
        }
    }

    #[test]
    fn test_call_plugin_import_func_success() {
        let lua = Rc::new(Lua::new());
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let plugin_code = r#"
        function get_import_code(current_path, referenced_path, code_block)
            return "import " .. current_path .. " " .. referenced_path .. " " .. code_block
        end

        function clean_code(code)
            return "cleaned " .. code
        end
        "#;

        create_temp_plugin(temp_dir.path(), "rs", plugin_code);

        let caller = PluginsCaller::new(Rc::clone(&lua), temp_dir.path()).unwrap();

        let current_path = PathBuf::from("/path/to/current.rs");
        let referenced_path = PathBuf::from("/path/to/referenced.rs");
        let code_block = "fn main() {}";

        let result =
            caller.call_plugin_import_func("rs", &current_path, &referenced_path, code_block);
        assert!(result.is_ok());

        let import_code = result.unwrap();
        assert!(import_code.contains("import"));
        assert!(import_code.contains("/path/to/current.rs"));
        assert!(import_code.contains("/path/to/referenced.rs"));
        assert!(import_code.contains("fn main() {}"));
    }

    #[test]
    fn test_call_plugin_import_func_not_found() {
        let lua = Rc::new(Lua::new());
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let plugin_code = r#"
        function get_import_code(current_path, referenced_path, code_block)
            return "import code"
        end

        function clean_code(code)
            return "cleaned code"
        end
        "#;

        create_temp_plugin(temp_dir.path(), "rust", plugin_code);

        let caller = PluginsCaller::new(Rc::clone(&lua), temp_dir.path()).unwrap();

        let result = caller.call_plugin_import_func(
            "python",
            &PathBuf::from("file.py"),
            &PathBuf::from("other.py"),
            "code",
        );
        assert!(result.is_err());

        match result {
            Err(LPError::PluginNotFound(_)) => {}
            _ => panic!("Expected PluginNotFound error"),
        }
    }

    #[test]
    fn test_call_plugin_cleaning_func_success() {
        let lua = Rc::new(Lua::new());
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let plugin_code = r#"
        function get_import_code(current_path, referenced_path, code_block)
            return "import code"
        end

        function clean_code(code)
            return "cleaned " .. code
        end
        "#;

        create_temp_plugin(temp_dir.path(), "rust", plugin_code);

        let caller = PluginsCaller::new(Rc::clone(&lua), temp_dir.path()).unwrap();

        let result = caller.call_plugin_cleaning_func("rust", "fn main() {}");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "cleaned fn main() {}");
    }

    #[test]
    fn test_call_plugin_cleaning_func_not_found() {
        let lua = Rc::new(Lua::new());
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let plugin_code = r#"
        function get_import_code(current_path, referenced_path, code_block)
            return "import code"
        end

        function clean_code(code)
            return "cleaned code"
        end
        "#;

        create_temp_plugin(temp_dir.path(), "rust", plugin_code);

        let caller = PluginsCaller::new(Rc::clone(&lua), temp_dir.path()).unwrap();

        let result = caller.call_plugin_cleaning_func("py", "def main(): pass");
        assert!(result.is_err());

        match result {
            Err(LPError::PluginNotFound(_)) => {}
            _ => panic!("Expected PluginNotFound error"),
        }
    }

    #[test]
    fn test_multiple_plugins() {
        let lua = Rc::new(Lua::new());
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        create_temp_plugin(
            temp_dir.path(),
            "rust",
            r#"
        function get_import_code(current_path, referenced_path, code_block)
            return "rust import: " .. code_block
        end

        function clean_code(code)
            return "rust cleaned: " .. code
        end
        "#,
        );

        create_temp_plugin(
            temp_dir.path(),
            "python",
            r#"
        function get_import_code(current_path, referenced_path, code_block)
            return "python import: " .. code_block
        end

        function clean_code(code)
            return "python cleaned: " .. code
        end
        "#,
        );

        let caller = PluginsCaller::new(Rc::clone(&lua), temp_dir.path()).unwrap();

        assert_eq!(caller.plugin_import_functions.len(), 2);
        assert_eq!(caller.plugin_cleaning_functions.len(), 2);

        let rust_import = caller
            .call_plugin_import_func(
                "rust",
                &PathBuf::from("file.rs"),
                &PathBuf::from("other.rs"),
                "fn main() {}",
            )
            .unwrap();

        assert_eq!(rust_import, "rust import: fn main() {}");

        let python_cleaned = caller
            .call_plugin_cleaning_func("python", "def main(): pass")
            .unwrap();
        assert_eq!(python_cleaned, "python cleaned: def main(): pass");
    }
}
