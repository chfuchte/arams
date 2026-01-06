use crate::lsp::errors::LSPError;

#[derive(Clone, Debug)]
pub struct LSPToken {
    kind: LSPTokenKind,
    lexeme: String,
    errors: Vec<LSPError>,
}

impl LSPToken {
    pub fn new(kind: LSPTokenKind, lexeme: String, errors: Vec<LSPError>) -> Self {
        Self {
            kind,
            lexeme,
            errors,
        }
    }

    pub fn kind(&self) -> &LSPTokenKind {
        &self.kind
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn errors(&self) -> &Vec<LSPError> {
        &self.errors
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LSPTokenKind {
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
    LabelDefinition,
    JumpArgument,
    ImmediateArgument,
    IndirectAddressArgument,
    DirectAddressArgument,
    Comment,
    NewLine,
    Unknown,
}

impl std::fmt::Display for LSPTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LSPTokenKind::Load => write!(f, "load"),
            LSPTokenKind::Store => write!(f, "store"),
            LSPTokenKind::Add => write!(f, "add"),
            LSPTokenKind::Sub => write!(f, "sub"),
            LSPTokenKind::Mul => write!(f, "mul"),
            LSPTokenKind::Div => write!(f, "div"),
            LSPTokenKind::Goto => write!(f, "goto"),
            LSPTokenKind::Jzero => write!(f, "jzero"),
            LSPTokenKind::Jnzero => write!(f, "jnzero"),
            LSPTokenKind::End => write!(f, "end"),
            LSPTokenKind::LabelDefinition => write!(f, "label_definition"),
            LSPTokenKind::JumpArgument => write!(f, "jump_argument"),
            LSPTokenKind::ImmediateArgument => write!(f, "immediate_argument"),
            LSPTokenKind::IndirectAddressArgument => write!(f, "indirect_address_argument"),
            LSPTokenKind::DirectAddressArgument => write!(f, "direct_address_argument"),
            LSPTokenKind::Comment => write!(f, "comment"),
            LSPTokenKind::NewLine => write!(f, "newline"),
            LSPTokenKind::Unknown => write!(f, "unknown"),
        }
    }
}

impl LSPTokenKind {
    pub fn get_about_text(&self) -> String {
        match &self {
            LSPTokenKind::Load => "Syntax: `load <operand>`\nLoads the value of the operand into the accumulator.".to_string(),
            LSPTokenKind::Store => "Syntax: `store <operand>`\nStores the value of the accumulator into the operand.".to_string(),
            LSPTokenKind::Add => "Syntax: `add <operand>`\nAdds the value of the operand to the accumulator.".to_string(),
            LSPTokenKind::Sub => "Syntax: `sub <operand>`\nSubtracts the value of the operand from the accumulator.".to_string(),
            LSPTokenKind::Mul => "Syntax: `mul <operand>`\nMultiplies the accumulator by the value of the operand.".to_string(),
            LSPTokenKind::Div => "Syntax: `div <operand>`\nDivides the accumulator by the value of the operand.".to_string(),
            LSPTokenKind::Goto => "Syntax: `goto <label>`\nUnconditionally jumps to the instruction marked with the given label.".to_string(),
            LSPTokenKind::Jzero => "Syntax: `jzero <label>`\nJumps to the instruction marked with the given label if the accumulator is zero.".to_string(),
            LSPTokenKind::Jnzero => "Syntax: `jnzero <label>`\nJumps to the instruction marked with the given label if the accumulator is not zero.".to_string(),
            LSPTokenKind::End => "Syntax: `end`\nEnds the program.".to_string(),
            LSPTokenKind::LabelDefinition => "Syntax: `<label_name>:`\nA label marks a position in the program that can be jumped to.".to_string(),
            LSPTokenKind::JumpArgument => "A label".to_string(),
            LSPTokenKind::ImmediateArgument => "Syntax: `#<value>`\nUses a constant value directly.".to_string(),
            LSPTokenKind::IndirectAddressArgument => "Syntax: `*<register_address>`\nUses the value stored in the given register as a register address. This is equivalent to a pointer with a depth of 1.".to_string(),
            LSPTokenKind::DirectAddressArgument => "Syntax: `<register_address>`\nUses the value at the address stored in the given register.".to_string(),
            _ => "".to_string(),
        }
    }
}
