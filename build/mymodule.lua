package.cpath = package.cpath .. ";/src/build/lib?.so"
package.path = package.path .. ";/src/build/?.lua"

local bert = require("transformers_ao")

local function read_string(path)
    local file = io.open(path, "r")
    if file == nil then
    	print(path .. " is nil")
    	return nil
    end
    local content = file:read("*a")
    file:close()
    return content
end

local function main()
    local config = read_string("../src/models/bert/config.json")
    if config == nil then
    	error("config is nil")
    end
    local tokenizer = read_string("../src/models/bert/tokenizer.json")
    if tokenizer == nil then
    	error("tokenizer is nil")
    end
    local model = read_string("../src/models/bert/model.safetensors")
    if model == nil then
    	error("model is nil")
    end
    local emb = bert.encode_text(config, tokenizer, model)
    print(emb)
end

main()