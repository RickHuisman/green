use crate::syntax::parser::{EvalParser, ModuleAst};
use std::env::current_dir;
use std::path::Path;

#[derive(Debug)]
pub enum ImportModuleError {
    FailedImport, // TODO
}

pub fn get_module_ast(module: &String) -> Result<ModuleAst, ImportModuleError> {
    let module_path = resolve_module_path(module);
    let body = get_file_contents(module_path.to_str().unwrap()).unwrap();
    let module_ast = EvalParser::parse(&body).unwrap();
    Ok(module_ast)
}

fn resolve_module_path(module: &String) -> Box<Path> {
    let mut path = current_dir().unwrap();
    path.push(Path::new("lib"));
    for dir in module.split('.') {
        path.push(Path::new(dir))
    }

    path.set_extension(Path::new("eval"));

    path.into_boxed_path()
}

fn get_file_contents(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
