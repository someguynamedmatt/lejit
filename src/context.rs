extern crate Boa;
use Boa::syntax::ast;

/// Ctx (Context) is what contains the compiled code (functions, objects, etc.)
pub struct Ctx {
    ast: u8,
}
