function get_import_code(current_path, referenced_path_str, code_block)
    local definitions = {}
    for line in code_block:gmatch("[^\r\n]+") do
        local export_function_name = line:match("^export%s+function%s+([%w_]+)%(")
        local export_class_name = line:match("^export%s+class%s+([%w_]+)")
        -- not all of the cases, just for the demostration

        if export_function_name then
            table.insert(definitions, export_function_name)
        elseif export_class_name then
            table.insert(definitions, export_class_name)
        end
    end

    if #definitions == 0 then
        return ""
    end

    local referenced_module = referenced_path_str
        :gsub("%.js$", "")
        :gsub("[\\]+", "/")

    return "const { " .. table.concat(definitions, ", ") .. " } = require('./" .. referenced_module .. "');"
end

function clean_code(code)
    local lines = {}
    for line in code:gmatch("[^\r\n]+") do
        table.insert(lines, line)
    end
    
    local seen_requires = {}
    local seen_imports = {}
    local clean_lines = {}

    for _, line in ipairs(lines) do
        local trimmed = line:match("^%s*(.-)%s*$")

        local require_module = trimmed:match("^const%s+[%w_%{},= ]-%s*=%s*require%(['\"]([^'\"]+)['\"]%)")

        local import_module = trimmed:match("^import.*from%s+['\"]([^'\"]+)['\"]")

        if require_module then
            if not seen_requires[require_module] then
                seen_requires[require_module] = true
                table.insert(clean_lines, line)
            end
        elseif import_module then
            if not seen_imports[import_module] then
                seen_imports[import_module] = true
                table.insert(clean_lines, line)
            end
        else
            table.insert(clean_lines, line)
        end
    end

    return table.concat(clean_lines, "\n")
end
