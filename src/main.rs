mod hash_vec;
mod interpreter;
use interpreter::*;

fn main() {
    let mut interpreter = Interpreter::new(String::from(r"F:\SourceOffline\Rust\il_runtime\ILAssembly\TestCsharp.dll")).unwrap();
    interpreter.run();
}