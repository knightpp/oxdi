use serde::Deserialize;
use std::fmt;
use surf::Url;
use tracing::{error, trace};

/// Version of the Oxford API with which the crate was tested
pub static API_VERSION: &str = "2.5.0";
pub mod apis;
pub mod languages;
pub mod models;
pub use languages::Language;

type Result<T> = std::result::Result<T, Error>;

use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ErrorExplanation {
    pub error: String,
}

impl fmt::Display for ErrorExplanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("http error: {0}")]
    SurfError(surf::Error),
    #[error("api request with id `{x_request_id}` returned error: {explanation}")]
    ApiReturnedError {
        explanation: ErrorExplanation,
        x_request_id: String,
    },
    #[error("language `{lang}` does not support `{api}`")]
    UnsupportedApi { lang: Language, api: String },
}

impl From<surf::Error> for Error {
    fn from(serr: surf::Error) -> Self {
        Error::SurfError(serr)
    }
}

const API_BASE_URL: &str = "https://od-api.oxforddictionaries.com/api/v2/";
#[derive(Debug, Clone, Copy)]
/// All available endpoints
enum Endpoint {
    Entries,
    // Lemmas,
    // Search,
    // Translations,
    // Thesaurus,
    // Sentences,
    // Utility,
    // Words,
    // Inflections,
    GrammaticalFeatures,
}

impl Endpoint {
    const fn as_str(&self) -> &'static str {
        match self {
            Endpoint::Entries => "entries",
            // Endpoint::Lemmas => "lemmas",
            // Endpoint::Search => "search",
            // Endpoint::Translations => "translations",
            // Endpoint::Thesaurus => "thesaurus",
            // Endpoint::Sentences => "sentences",
            // Endpoint::Utility => "utility",
            // Endpoint::Words => "words",
            // Endpoint::Inflections => "inflections",
            Endpoint::GrammaticalFeatures => "grammaticalFeatures",
        }
    }
}

// impl<'eq> EntriesQuery<'eq> {
//     const fn as_str(&self) -> &'static str {
//         match self {
//             EntriesQuery::Fields(_) => "fields",
//             EntriesQuery::GrammaticalFeatures(_) => "grammaticalFeatures",
//             EntriesQuery::LexicalCategory(_) => "lexicalCategory",
//             EntriesQuery::Domains(_) => "domains",
//             EntriesQuery::Registers(_) => "registers",
//             EntriesQuery::StrictMatch(_) => "strictMatch",
//         }
//     }
// }

#[derive(Debug)]
/// Main interface to the API wrapper
/// ## Example
/**```no_run
# fn main(){
# futures::executor::block_on(run());
# }
# async fn run(){
use oxdi::{apis, Language, apis::entries::{Param, Field}};
let client = oxdi::Client::new("id".to_owned(), "key".to_owned(),
    Language::EnGb);
// or
// let resp: serde_json::Value
let resp: oxdi::models::RetrieveEntry = client.entries("crab", 
    &[Param::Fields(&[
            Field::Pronunciations,
        ])
    ]).await.expect("handle errors");
# }
```*/
pub struct Client {
    app_id: String,
    app_key: String,
    source_language: languages::Language,
    http_client: surf::Client,
}

impl Client {
    /// Creates new instance of `Client`
    pub fn new(app_id: String, app_key: String, source_language: Language) -> Self {
        let mut http_client = surf::Client::new();
        http_client.set_base_url(Url::parse(API_BASE_URL).unwrap());
        Self {
            app_id,
            app_key,
            source_language,
            http_client,
        }
    }

    /// Query all available grammatical features of
    /// the language  
    pub async fn grammatical_features(&self) -> Result<serde_json::Value> {
        let endpoint = Endpoint::GrammaticalFeatures;
        let path = format!("{}/{}", endpoint.as_str(), self.source_language.code());
        let mut resp = self.get_request(&path).send().await?;
        trace!(?endpoint, ?path, "requesting grammatical_features API");

        Client::handle_api_error(&mut resp).await?;
        Ok(resp.body_json().await?)
    }
    /// Builds [`RequestBuilder`](surf::RequestBuilder) and 
    /// sets up headers 
    fn get_request(&self, uri: &str) -> surf::RequestBuilder {
        trace!("GET request to: {}", uri);
        self.http_client
            .get(uri)
            .header("app_id", &self.app_id)
            .header("app_key", &self.app_key)
            .header("Accept", "application/json")
    }
    async fn handle_api_error(resp: &mut surf::Response) -> Result<()> {
        trace!("checking for API errors");
        let status = resp.status();
        if [400, 404, 414, 500].contains(&(status as u16)) {
            // FIXME: add path to the error?
            let explanation: ErrorExplanation = resp.body_json().await?;
            let x_request_id = resp.header("X-Request-Id").unwrap().to_string();
            error!(?explanation, ?x_request_id, ?status);
            return Err(Error::ApiReturnedError {
                explanation,
                x_request_id,
            });
        }
        Ok(())
    }
}
