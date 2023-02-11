use serde::{ Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Data {
    status: usize,
    message: String
}
fn main(){
    //  init v8 engine
    init();
    // create an isolate
    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let source = r#"
        function hello(){
            return {
                status: 200,
                message: 'success'
            }
        };
        hello();
    "#;
    
    let code = v8::String::new(scope, source).unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope));
    
    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();

    let value: Data = serde_v8::from_v8(scope, result).unwrap();
    println!("Result: {value:?}");
}

fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}
