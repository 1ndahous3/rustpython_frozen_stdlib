use rustpython_vm as vm;
use rustpython_vm::AsObject;


const SCRIPT: &str = r#"
import sys
print(sys.path)

import datetime
"#;


fn main() {

    let mut settings = vm::Settings::default();
    settings.path_list.push("RustPython\\Lib".to_owned());

    let interpreter = vm::Interpreter::with_init(settings, |vm| {
        vm.add_frozen(rustpython_pylib::FROZEN_STDLIB);
    });

    let _ = interpreter.enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        vm.run_code_string(scope, SCRIPT, "<script>".to_owned())
            .map(drop)
            .map_err(|e| {
                let err_str = vm.call_method(e.as_object(), "__str__", ())
                    .ok()
                    .and_then(|s| s.downcast::<rustpython_vm::builtins::PyStr>().ok())
                    .map(|s| s.as_str().to_string())
                    .unwrap_or_else(|| "<unprintable>".into());
                format!("Python error: {}", err_str)
            })
    }).expect("Failed to run script");
    println!("Script executed successfully.");
}
