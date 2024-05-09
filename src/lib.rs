#![allow(unused)]
extern crate core;
// use base64::prelude::*;

// use base64::decode;
// use std::str::FromStr;
// use base64::prelude::*;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
// use hf_hub::{Repo, RepoType};
// use hf_hub::api::sync::Api;
use mlua::prelude::*;
use mlua::{
    String, Result as LuaResult,
    UserData,
    Table as LuaTable,
    Function as LuaFunction
};
// use mlua::Error::RuntimeError as LuaRuntimeError;
// use serde::{Deserialize, Serialize};
// use serde_json::{
//     from_str,
//     // to_string_pretty
// };
use tokenizers;
use tokenizers::{EncodeInput, InputSequence, PaddingParams, Tokenizer};


pub fn encode_text<'lua>(_: &'lua Lua, (config, tokenizer, model): (LuaString, LuaString, LuaString)) -> LuaResult<Vec<f32>> {
    let prompt = "Hello, world!";
    let normalize_embeddings = true;


    let sentences = [prompt];
    let n_sentences = sentences.len();
    let input_text = InputSequence::from(prompt);

    let model_bytes: Vec<u8> = model.clone().as_bytes().to_vec();
    let model_id = "sentence-transformers/all-MiniLM-L6-v2".to_string();
    let revision = "refs/pr/21".to_string();

    let mut config: Config = serde_json::from_str::<Config>(config.to_str()?)
        .map_err(|err|LuaError::external(err))?;

    let mut tokenizer = Tokenizer::from_bytes(tokenizer.as_bytes())
        .map_err(|err|LuaError::external(err))?;
    let pp = PaddingParams {
        strategy: tokenizers::PaddingStrategy::BatchLongest,
        ..Default::default()
    };
    tokenizer.with_padding(Some(pp));
    let vb = unsafe {
        VarBuilder::from_buffered_safetensors(model_bytes, DTYPE, &Device::Cpu)
            .map_err(|err|LuaError::external(err))?
    };
    let model = BertModel::load(vb, &config)
        .map_err(|err|LuaError::external(err))?;

    // let tokens = tokenizer.encode(prompt, true)
    //     .map_err(|err|LuaError::external(err))?
    //     .get_ids()
    //     .to_vec();
    //
    // let token_ids = Tensor::new(&tokens[..], &Device::Cpu)
    //     .map_err(|err|LuaError::external(err))?
    //     .unsqueeze(0)
    //     .map_err(|err|LuaError::external(err))?;
    //
    // let token_type_ids = token_ids.zeros_like()
    //     .map_err(|err|LuaError::external(err))?;
    //
    // println!("Tokens done");
    //
    // let embedding: Tensor = model.forward(&token_ids, &token_type_ids)
    //     .map_err(|err|LuaError::external(err))?;
    // println!("{}\n{:?}", embedding, embedding.dims3());
    // let embedding_as_vec = embedding.iter().collect();
    let tokens = tokenizer
        .encode_batch(sentences.to_vec(), true)
        .map_err(|err|LuaError::external(err))?;
    let token_ids = tokens
        .iter()
        .map(|tokens| {
            let tokens = tokens.get_ids().to_vec();
            Ok(Tensor::new(tokens.as_slice(), &Device::Cpu).map_err(|err|LuaError::external(err))?)
        })
        .collect::<LuaResult<Vec<_>>>()?;

    let token_ids = Tensor::stack(&token_ids, 0)
        .map_err(|err|LuaError::external(err))?;
    let token_type_ids = token_ids.zeros_like()
        .map_err(|err|LuaError::external(err))?;
    println!("running inference on batch {:?}", token_ids.shape());
    let embeddings = model.forward(&token_ids, &token_type_ids)
        .map_err(|err|LuaError::external(err))?;
    println!("generated embeddings {:?}", embeddings.shape());
    // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
    let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()
        .map_err(|err|LuaError::external(err))?;
    let dimsum = embeddings.sum(1).map_err(|err|LuaError::external(err))?;
    let embeddings = (dimsum / (n_tokens as f64))
        .map_err(|err|LuaError::external(err))?;

    let embeddings = if normalize_embeddings {
        normalize_l2(&embeddings)?
    } else {
        embeddings
    };
    println!("pooled embeddings {:?}", embeddings.shape());
    let embeddings_vec = embeddings.flatten_all()
        .map_err(|err|LuaError::external(err))?
        .to_vec1::<f32>()
        .map_err(|err|LuaError::external(err))?;
    // println!("embvec is {:?}", embeddings_vec);
    Ok(embeddings_vec)
}

pub fn normalize_l2(v: &Tensor) -> LuaResult<Tensor> {
    Ok(v.broadcast_div(
        &v.sqr().map_err(|err|LuaError::external(err))?
            .sum_keepdim(1).map_err(|err|LuaError::external(err))?
            .sqrt().map_err(|err|LuaError::external(err))?
        ).map_err(|err|LuaError::external(err))?
    )
}

fn make_exports<'lua>(
    lua: &'lua Lua,
    encode_text: LuaFunction<'lua>,
) -> LuaResult<LuaTable<'lua>> {
    let exports = lua.create_table()?;
    exports.set("encode_text", encode_text.clone())?;
    exports.set("null", lua.null())?;
    Ok(exports)
}

#[mlua::lua_module]
fn transformers_ao(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    // exports.set("encode_text", lua.create_function(encode_text)?)?;
    let encode_text = lua.create_function(encode_text)?;
    make_exports(lua, encode_text)
}