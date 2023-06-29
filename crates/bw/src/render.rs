use bitwarden::secrets_manager::{projects::ProjectResponse, secrets::SecretResponse};
use chrono::DateTime;
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

// We're using const generics for the array lengths to make sure the header count and value count match
pub(crate) trait TableSerialize<const N: usize>: Sized {
    fn get_headers() -> [&'static str; N];
    fn get_values(&self) -> Vec<[String; N]>;
}

// Generic impl for Vec<T> so we can call `serialize_response` with both individual
// elements and lists of elements, like we do with the JSON and YAML cases
impl<T: TableSerialize<N>, const N: usize> TableSerialize<N> for Vec<T> {
    fn get_headers() -> [&'static str; N] {
        T::get_headers()
    }
    fn get_values(&self) -> Vec<[String; N]> {
        let mut values = Vec::new();
        for t in self {
            values.append(&mut t.get_values());
        }
        values
    }
}

fn format_date(date: &str) -> String {
    DateTime::parse_from_rfc3339(date)
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

impl TableSerialize<3> for ProjectResponse {
    fn get_headers() -> [&'static str; 3] {
        ["ID", "Name", "Creation Date"]
    }

    fn get_values(&self) -> Vec<[String; 3]> {
        vec![[
            self.id.to_string(),
            self.name.clone(),
            format_date(&self.creation_date),
        ]]
    }
}

impl TableSerialize<4> for SecretResponse {
    fn get_headers() -> [&'static str; 4] {
        ["ID", "Key", "Value", "Creation Date"]
    }

    fn get_values(&self) -> Vec<[String; 4]> {
        vec![[
            self.id.to_string(),
            self.key.clone(),
            self.value.clone(),
            format_date(&self.creation_date),
        ]]
    }
}
