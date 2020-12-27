use crate::{error, trace};
use crate::{Client, Endpoint, Error, Result};
use serde::{Serialize, Serializer};

// "https://od-api.oxforddictionaries.com/api/v2/<endpoint>/<language_code>/<word_id>"
impl Client {
    /// Request definitions, pronunciations, example sentences,
    /// grammatical information, word origins.
    /// ## Arguments
    /// `word_id` - should be dictionary headword (e.g. crab, not crabs)
    /// ## Misuse
    /// You should not duplicate [`Param`](Param) in this
    /// function call
    pub async fn entries<'s, D>(&'s self, word_id: &'s str, params: &'s [Param<'s>]) -> Result<D>
    where
        D: serde::de::DeserializeOwned,
    {
        use std::io::Write;
        trace!("request to entries api");
        if !self.source_language.entries_api() {
            error!("lang {} doesn't support Entries API", self.source_language);
            return Err(Error::UnsupportedApi {
                lang: self.source_language,
                api: "entries".to_owned(),
            });
        }

        let endpoint = Endpoint::Entries.as_str();
        let mut path = Vec::with_capacity(
            endpoint.len() + self.source_language.code().len() + word_id.len() + 64,
        );
        write!(
            &mut path,
            "{}/{}/{}",
            Endpoint::Entries.as_str(),
            self.source_language.code(),
            word_id
        )
        .unwrap();
        write!(&mut path, "?").unwrap();
        for (i, q) in params.iter().enumerate() {
            serde_qs::to_writer(q, &mut path).unwrap();
            if i != params.len() - 1 {
                write!(&mut path, "&").unwrap();
            }
        }
        let mut resp = self
            .get_request(std::str::from_utf8(&path).unwrap())
            .send()
            .await?;
        Client::handle_api_error(&mut resp).await?;
        Ok(resp.body_json().await?)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Param<'eq> {
    #[serde(serialize_with = "fields_serializer")]
    Fields(&'eq [Field]),
    #[serde(serialize_with = "comma_seperated_serializer")]
    GrammaticalFeatures(&'eq [&'eq str]),
    #[serde(serialize_with = "comma_seperated_serializer")]
    LexicalCategory(&'eq [&'eq str]),
    #[serde(serialize_with = "comma_seperated_serializer")]
    Domains(&'eq [&'eq str]),
    #[serde(serialize_with = "comma_seperated_serializer")]
    Registers(&'eq [&'eq str]),
    #[serde(serialize_with = "comma_seperated_serializer")]
    StrictMatch(&'eq [&'eq str]),
}

fn fields_serializer<S: Serializer>(
    fields: &[Field],
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    let fields = fields
        .iter()
        .map(|x| x.as_str())
        .collect::<Box<[&'static str]>>();
    let joined = fields.join(",");
    s.serialize_str(&joined)
}

fn comma_seperated_serializer<S: Serializer>(
    text: &[&str],
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    let joined = text.join(",");
    s.serialize_str(&joined)
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Field {
    Definitions,
    Domains,
    Etymologies,
    Examples,
    Pronunciations,
    Regions,
    Registers,
    VariantForms,
}

impl Field {
    const fn as_str(&self) -> &'static str {
        match self {
            Field::Definitions => "definitions",
            Field::Domains => "domains",
            Field::Etymologies => "etymologies",
            Field::Examples => "examples",
            Field::Regions => "regions",
            Field::Registers => "registers",
            Field::VariantForms => "variantForms",
            Field::Pronunciations => "pronunciations",
        }
    }
}
