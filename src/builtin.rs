#[derive(Clone, Debug, PartialEq)]
pub enum Builtin {
    Put,
    Get,
    Type,
    Len,
    Panic,
    Push,
    Pop
}