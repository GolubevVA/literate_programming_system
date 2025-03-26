function get_import_code(current_path, referenced_path_str, code_block)
    local definitions = {}
    for line in code_block:gmatch("[^\r\n]+") do

        local class_name = line:match("^class%s+([%w_]+)")
        local function_name = line:match("^def%s+([%w_]+)")
        if class_name then
            table.insert(definitions, class_name)
        elseif function_name then
            table.insert(definitions, function_name)
        end
    end

    local referenced_module = referenced_path_str
        :gsub("%.py$", "")
        :gsub("[/\\]+", ".")

    if #definitions == 0 then
        return ""
    end
    return "from " .. referenced_module .. " import " .. table.concat(definitions, ", ")
end

function clean_code(code)
    local lines = {}
    for line in code:gmatch("[^\r\n]+") do
        table.insert(lines, line)
    end
    
    local seen_imports = {}
    local seen_from_imports = {}
    local clean_lines = {}
    
    for _, line in ipairs(lines) do
        local trimmed = line:match("^%s*(.-)%s*$")
        
        local import_module = trimmed:match("^import%s+([^%s,]+)")

        local from_module, imports = trimmed:match("^from%s+([%w_.]+)%s+import%s+(.+)")

        if import_module then
            if not seen_imports[import_module] then
                seen_imports[import_module] = true
                table.insert(clean_lines, line)
            end
        elseif from_module and imports then
            local key = from_module

            if not seen_from_imports[key] then
                seen_from_imports[key] = {}
            end

            local import_list = {}
            local has_new = false

            for item in imports:gmatch("([^,]+)") do
                local name = item:match("^%s*([%w_]+)")
                
                if name and not seen_from_imports[key][name] then
                    seen_from_imports[key][name] = true
                    table.insert(import_list, item:match("^%s*(.-)%s*$"))
                    has_new = true
                end
            end

            if has_new and #import_list > 0 then
                local new_line = "from " .. from_module .. " import " .. table.concat(import_list, ", ")
                table.insert(clean_lines, new_line)
            end
        else
            table.insert(clean_lines, line)
        end
    end
    
    return table.concat(clean_lines, "\n")
end
