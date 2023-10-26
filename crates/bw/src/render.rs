use bitwarden::admin_console::auth_requests::{
    PendingAuthRequestResponse, PendingAuthRequestsResponse,
};
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use comfy_table::Table;
use serde::Serialize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[allow(clippy::upper_case_acronyms)]
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
            Color::Auto => supports_color::on(supports_color::Stream::Stdout).is_some(),
        }
    }
}

const ASCII_HEADER_ONLY: &str = "     --            ";

pub(crate) fn serialize_response<T: Serialize + TableSerialize<N>, const N: usize>(
    data: T,
    output: Output,
    color: bool,
) {
    match output {
        Output::JSON => {
            let mut text = serde_json::to_string_pretty(&data).unwrap();
            // Yaml/table/tsv serializations add a newline at the end, so we do the same here for consistency
            text.push('\n');
            pretty_print("json", &text, color);
        }
        Output::YAML => {
            let text = serde_yaml::to_string(&data).unwrap();
            pretty_print("yaml", &text, color);
        }
        Output::Table => {
            let mut table = Table::new();
            table
                .load_preset(ASCII_HEADER_ONLY)
                .set_header(T::get_headers())
                .add_rows(data.get_values());

            println!("{table}");
        }
        Output::TSV => {
            println!("{}", T::get_headers().join("\t"));

            let rows: Vec<String> = data
                .get_values()
                .into_iter()
                .map(|row| row.join("\t"))
                .collect();
            println!("{}", rows.join("\n"));
        }
        Output::None => {}
    }
}

fn pretty_print(language: &str, data: &str, color: bool) {
    if color {
        bat::PrettyPrinter::new()
            .input_from_bytes(data.as_bytes())
            .language(language)
            .print()
            .unwrap();
    } else {
        print!("{}", data);
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

fn format_date(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

impl TableSerialize<4> for PendingAuthRequestResponse {
    fn get_headers() -> [&'static str; 4] {
        ["ID", "User ID", "Organization User ID", "Email"]
    }

    fn get_values(&self) -> Vec<[String; 4]> {
        vec![[
            self.id.to_string(),
            self.user_id.to_string(),
            self.organization_user_id.to_string(),
            self.email.clone(),
        ]]
    }
}
