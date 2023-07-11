use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub(crate) enum Output {
    JSON,
    YAML,
    Table,
    TSV,
    None,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub(crate) enum Color {
    No,
    Yes,
    Auto,
}

impl Color {
    pub(crate) fn is_enabled(self) -> bool {
        match self {
            Color::No => false,
            Color::Yes => true,
            Color::Auto => {
                if std::env::var("NO_COLOR").is_ok() {
                    false
                } else {
                    atty::is(atty::Stream::Stdout)
                }
            }
        }
    }
}
