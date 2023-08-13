use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub(crate) enum Output {
    JSON,
    YAML,
    Table,
    TSV,
    None,
}
