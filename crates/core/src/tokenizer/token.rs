#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    Load,
    Store,
    Add,
    Sub,
    Mul,
    Div,
    Goto,
    Jzero,
    Jnzero,
    End,
    LabelDefinition(String),
    Argument(String),
    Comment(String),
    NewLine,
}
