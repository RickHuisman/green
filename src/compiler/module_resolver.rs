use std::path::Path;
use crate::syntax::parser::ModuleAst;

pub enum ImportModuleError {
    FailedImport, // TODO
}

pub fn get_module_ast(module: &String) -> Result<ModuleAst, ImportModuleError> {
    let module_path = resolve_module_path(module);
    let body = get_file_contents(module_path.to_str().unwrap()).unwrap();
    // let module_ast = EvalParser::parse(&body)?;
    // Ok(module_ast)
    Err(ImportModuleError::FailedImport)
    // let fun = Compiler::compile_module(module_ast);
    // Err(ImportModuleError::FailedImport)
}

fn resolve_module_path(module: &String) -> &Path {
    // let path = Path::new("").to_string();
    // let mut path = "".to_string();
    //
    // for dir in module.split('.') {
    //     let mut test = dir.to_string();
    //     test.push_str("/");
    //     path.push_str(&test);
    // }
    //
    // path.pop();
    // path.push_str(".eval");
    // println!("{}", &path);
    // // println!("{:?}", Path::new(&path));
    // println!("{:?}", get_file_contents(&path));

    &Path::new("/Users/rickhuisman/Documents/rust/eval/lib/io.eval")
}

fn get_file_contents(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}