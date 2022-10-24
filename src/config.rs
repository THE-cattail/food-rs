use anyhow::Result;
use serde::Deserialize;

type ParserFn<'de, T> = Box<dyn Fn(&'de [u8]) -> Result<T>>;

macro_rules! parser_fn {
    ( $expr:expr ) => {
        Box::new(|buf| Ok($expr(buf)?))
    };
}

struct Parser<'de, T>
where
    T: Deserialize<'de>,
{
    name: &'static str,
    f: ParserFn<'de, T>,
}

impl<'de, T> Parser<'de, T>
where
    T: Deserialize<'de>,
{
    fn new(name: &'static str, f: ParserFn<'de, T>) -> Self {
        Self { name, f }
    }
}

pub fn parse<'de, T>(buf: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let parsers: Vec<Parser<'de, T>> = vec![
        Parser::new("toml", parser_fn!(toml::from_slice)),
        Parser::new("json", parser_fn!(serde_json::from_slice)),
        Parser::new("serde", parser_fn!(serde_yaml::from_slice)),
    ];

    for parser in &parsers {
        match (parser.f)(buf) {
            Ok(r) => {
                log::debug!("Parsed config as {}", parser.name);
                return Ok(r);
            }
            Err(e) => log::debug!("Parsing config as {} error: {}", parser.name, e),
        }
    }

    Err(anyhow::anyhow!(
        "unknown config file format, only support: [{}]",
        parsers
            .into_iter()
            .map(|parser| parser.name)
            .collect::<Vec<&'static str>>()
            .join(", ")
    ))
}
