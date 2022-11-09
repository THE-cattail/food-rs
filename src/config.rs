use color_eyre::eyre;
use eyre::Result;
use serde::Deserialize;

type ParserFn<'de, T> = Box<dyn Fn(&'de [u8]) -> Result<T>>;

macro_rules! parser_fn {
    ($expr:expr) => {
        Box::new(|buf| Ok($expr(buf)?))
    };
}

struct Parser<'de, T>
where
    T: Deserialize<'de>,
{
    name: &'static str,
    f:    ParserFn<'de, T>,
}

impl<'de, T> Parser<'de, T>
where
    T: Deserialize<'de>,
{
    fn new(name: &'static str, f: ParserFn<'de, T>) -> Self {
        Self { name, f }
    }
}

/// # Errors
///
/// Will return `Err` if parsing for all parsers failed.
pub fn parse<'de, T>(buf: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let parsers: Vec<Parser<'de, T>> = vec![
        Parser::new("JSON", parser_fn!(serde_json::from_slice)),
        Parser::new("YAML", parser_fn!(serde_yaml::from_slice)),
        Parser::new("MessagePack", parser_fn!(rmp_serde::from_slice)),
        Parser::new("TOML", parser_fn!(toml::from_slice)),
        Parser::new("BSON", parser_fn!(bson::from_slice)),
        Parser::new("URL", parser_fn!(serde_qs::from_bytes)),
    ];
    let mut errors = Vec::new();

    for parser in &parsers {
        match (parser.f)(buf) {
            Ok(r) => {
                tracing::debug!("Parsed config as {}", parser.name);
                return Ok(r);
            },
            Err(e) => errors.push(format!("as {}: {e}", parser.name)),
        }
    }

    eyre::bail!(errors.join("\n"))
}
