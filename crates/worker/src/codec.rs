use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// Message Encoding and Decoding Format
pub trait Codec {
    /// Encode an input to JsValue
    fn encode<I>(input: I) -> JsValue
    where
        I: Serialize;

    /// Decode a message to a type
    fn decode<O>(input: JsValue) ->Option<O> 
    where
        O: for<'de> Deserialize<'de>;
}

/// Default message encoding with [bincode].
#[derive(Debug)]
pub struct Bincode;

impl Codec for Bincode {
    fn encode<I>(input: I) -> JsValue
    where
        I: Serialize,
    {
        let buf = bincode::serialize(&input);
        match buf {
            Ok(data) => {
                Uint8Array::from(data.as_slice()).into()
            }
            Err(_err) => {
                JsValue::NULL
            }
        }
    }

    fn decode<O>(input: JsValue) -> Option<O>
    where
        O: for<'de> Deserialize<'de>,
    {
        let data = Uint8Array::from(input).to_vec();
        let result:Result<O,bincode::Error> = bincode::deserialize(&data);
        match result {
            Ok(value) => Some(value),
            Err(_err) => {
                None
            } 
        }
    }
}
