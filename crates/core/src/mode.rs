#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Visual,
    VisualLine,
    Command,
    Search {
        forward: bool,
    },
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::Visual => write!(f, "VISUAL"),
            Mode::VisualLine => write!(f, "V-LINE"),
            Mode::Command => write!(f, "COMMAND"),
            Mode::Search { forward: true } => write!(f, "SEARCH /"),
            Mode::Search { forward: false } => write!(f, "SEARCH ?"),
        }
    }
}
