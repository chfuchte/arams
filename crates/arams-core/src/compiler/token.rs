#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Load { line_number: usize },
    Store { line_number: usize },
    Add { line_number: usize },
    Sub { line_number: usize },
    Mul { line_number: usize },
    Div { line_number: usize },
    Goto { line_number: usize },
    Jzero { line_number: usize },
    Jnzero { line_number: usize },
    End { line_number: usize },
    LabelDefinition { line_number: usize, value: String },
    Argument { line_number: usize, value: String },
    Comment { line_number: usize, value: String },
    NewLine { line_number: usize },
}
