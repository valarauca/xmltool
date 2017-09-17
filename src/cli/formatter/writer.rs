
use super::super::super::xml::EmitterConfig;
use super::super::super::clap::ArgMatches;

use std::borrow::Cow;

fn to_line_sep(arg: &ArgMatches) -> Cow<'static, str> {
    match arg.value_of("EOL") {
        Option::Some("CR") => Cow::Borrowed("\x0D"),
        Option::Some("LF") => Cow::Borrowed("\x0A"),
        Option::Some("CRLF") => Cow::Borrowed("\x0D\x0A"),
        Option::Some("LFCR") => Cow::Borrowed("\x0A\x0D"),
        Option::Some("NONE") => Cow::Borrowed(""),
        Option::Some("CRLF") |
        _ => Cow::Borrowed("\x0D\x0A")
    }
}

fn to_indent(arg: &ArgMatches) -> Cow<'static, str> {
    match arg.value_of("INDENT") {
        Option::Some("NONE") |
        Option::Some("0") => Cow::Borrowed(""),
        Option::Some("1") => Cow::Borrowed(" "),
        Option::Some("2") => Cow::Borrowed("  "),
        Option::Some("3") => Cow::Borrowed("   "),
        Option::Some("4") => Cow::Borrowed("    "),
        Option::Some("5") => Cow::Borrowed("     "),
        Option::Some("TAB") |
        _ => Cow::Borrowed("\x09")
    }
}

fn perform_indents(arg: &ArgMatches) -> bool {
    match arg.value_of("INDENT") {
        Option::Some("NONE") |
        Option::Some("0") => false,
        _ => true
    }
}

pub fn build_emitter_config(arg: &ArgMatches) -> EmitterConfig {
    EmitterConfig::new()
        .line_separator(to_line_sep(arg))
        .indent_string(to_indent(arg))
        .perform_indent(perform_indents(arg))
        .normalize_empty_elements(arg.is_present("NORMALIZE"))
        .autopad_comments(arg.is_present("PAD-COMMENTS"))
        .cdata_to_characters(arg.is_present("CDATA-OUT"))
        .write_document_declaration(!arg.is_present("DOC-DEC"))
}
