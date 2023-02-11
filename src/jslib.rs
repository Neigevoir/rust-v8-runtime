use v8::{Isolate, CreateParams};

pub struct  JsRuntime{
    isolate: Isolate,
}

#[derive(Default)]
pub struct  JsRuntimeParams(CreateParams);

impl JsRuntimeParams{
    pub fn new(snapshot: Option<Vec<u8>>) -> Self {
        JsRuntimeParams(CreateParams::default())
    }
}

impl JsRuntime {

    pub fn init(){
        todo!();
    }

    pub fn new(params: JsRuntimeParams) -> Self{
        todo!();
    }

    pub fn execute_script(&mut self, code: impl AsRef<str>) -> Result<serde_json::Value, serde_json::Value> {
        todo!();
    }

    pub fn createSnapShot() -> Vec<u8> {
        todo!();
    }
}