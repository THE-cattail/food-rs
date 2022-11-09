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
        Parser::new("Postcard", parser_fn!(postcard::from_bytes)),
        Parser::new("CBOR", parser_fn!(ciborium::de::from_reader)),
        Parser::new("YAML", parser_fn!(serde_yaml::from_slice)),
        Parser::new("MessagePack", parser_fn!(rmp_serde::from_slice)),
        Parser::new("TOML", parser_fn!(toml::from_slice)),
        // Parser::new("Pickle", parser_fn!(serde_pickle::from_slice)),
        Parser::new("RON", parser_fn!(ron::de::from_bytes)),
        Parser::new("BSON", parser_fn!(bson::from_slice)),
        // Parser::new("Avro", parser_fn!(apache_avro::from_avro_datum)),
        // Parser::new("JSON5", parser_fn!(json5::from_str)),
        Parser::new("URL", parser_fn!(serde_qs::from_bytes)),
        // Parser::new("Envy", parser_fn!(envy::from_iter)),
        // Parser::new("Envy Store", parser_fn!(envy_store::from_client)),
        // Parser::new("S-expressions", parser_fn!(lexpr::from_slice)),
        // Parser::new("D-Bus", parser_fn!(zvariant::from_slice)),
        Parser::new("FlexBuffers", parser_fn!(flexbuffers::from_slice)),
        // Parser::new("DynamoDB Items", parser_fn!(serde_dynamo::from_item)),
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
