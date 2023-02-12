use self::{ JsRuntime, JsRuntimeParams };

fn main(){
    JsRuntime::init();
    let options = None;

    let mut runtime = JsRuntime::new(JsRuntimeParams::default());
    let code = r#"
        function  hello(){
            function hello(){
                return {
                    status: 200,
                    message: 'success'
                }
            };
        }
        hello();
    "#;
    let result = runtime.execute_script(code);
    println!("Result is: {:?}", result);
}