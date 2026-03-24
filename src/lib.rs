use mlua::prelude::*;

use wordninja::{LanguageModel, DEFAULT_MODEL};

fn split(_: &Lua, input: String) -> LuaResult<String> {
    let words: Vec<_> = DEFAULT_MODEL.split(&input);
    let result = words.join(" ");
    Ok(result)
}

fn split_with_model(_: &Lua, (input, model_content): (String, String)) -> LuaResult<String> {
    if model_content.lines().filter(|s| !s.is_empty()).count() == 0 {
        return Err(mlua::Error::RuntimeError(
            "dictionary is empty: at least one word is required".to_string(),
        ));
    }
    let model = LanguageModel::new_model(&model_content);
    let words: Vec<_> = model.split(&input);
    let result = words.join(" ");
    Ok(result)
}

#[mlua::lua_module]
fn wordninja(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("split", lua.create_function(split)?)?;
    exports.set("split_with_model", lua.create_function(split_with_model)?)?;
    Ok(exports)
}
