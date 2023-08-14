use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Output {
    JSON,
    YAML,
    Table,
    TSV,
    None,
}
