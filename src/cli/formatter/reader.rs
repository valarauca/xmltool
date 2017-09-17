
use super::super::super::xml::ParserConfig;
use super::super::super::clap::ArgMatches;

/// Construct parser
pub fn build_parser_config(arg: &ArgMatches) -> ParserConfig {
    ParserConfig::new()
        .trim_whitespace(arg.is_present("TRIM-WHITESPACE"))
        .ignore_comments(arg.is_present("IGNORE-COMMENTS"))
}
