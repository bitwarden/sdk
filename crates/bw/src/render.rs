use bitwarden_cli::Color;
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

const ASCII_HEADER_ONLY: &str = "     --            ";

pub(crate) struct OutputSettings {
    pub(crate) output: Output,
    pub(crate) color: Color,
}

impl OutputSettings {
    pub(crate) fn new(output: Output, color: Color) -> Self {
        OutputSettings { output, color }
    }
}

pub(crate) fn serialize_response<T: Serialize + TableSerialize<N>, const N: usize>(
    data: T,
    output_settings: OutputSettings,
) {
    match output_settings.output {
        Output::JSON => {
            let mut text =
                serde_json::to_string_pretty(&data).expect("Serialize should be infallible");
            // Yaml/table/tsv serializations add a newline at the end, so we do the same here for
            // consistency
            text.push('\n');
            pretty_print("json", &text, output_settings.color);
        }
        Output::YAML => {
            let text = serde_yaml::to_string(&data).expect("Serialize should be infallible");
            pretty_print("yaml", &text, output_settings.color);
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

fn pretty_print(language: &str, data: &str, color: Color) {
    if color.is_enabled() {
        bat::PrettyPrinter::new()
            .input_from_bytes(data.as_bytes())
            .language(language)
            .print()
            .expect("Input is valid");
    } else {
        print!("{}", data);
    }
}

// We're using const generics for the array lengths to make sure the header count and value count
// match
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
