mod interpreter;
use interpreter::*;

fn main() {
    let mut interpreter = Interpreter::new(r"F:\SourceOffline\Rust\il_runtime\ILAsm\TestCsharp.dll").unwrap();
    interpreter.run();
}