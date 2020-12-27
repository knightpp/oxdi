use anyhow::{Context, Result};
use argh::FromArgs;
use oxdi::apis::entries::{Field, Param};
//use tracing::{info, Level};
//use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Debug, FromArgs)]
/// Search in Oxford Dictionary from CLI!
struct Args {
    #[argh(option, short = 'w', from_str_fn(parse_word))]
    /// case sensetive word to query
    word: String,

    #[argh(switch)]
    /// print version
    version: bool,

    #[argh(
        option,
        short = 's',
        from_str_fn(parse_lang),
        default = "oxdi::Language::EnGb"
    )]
    /// source language, by default `en-gb`
    language: oxdi::Language,

    #[argh(option, short = 't', from_str_fn(parse_lang))]
    /// source language
    target_language: Option<oxdi::Language>,
    // #[argh(option, from_str_fn(parse_vec_string))]
    // /// filter by grammatical features
    // grammatical_features: Option<Vec<String>>,
}

// fn parse_vec_string(value: &str) -> Result<Vec<String>, String> {
//     if !value.chars().all(|c| c.is_alphabetic() || c == ',') {
//         return Err(
//             "expected words seperated by comma, allowed only alphabetic and comma".to_owned(),
//         );
//     }
//     let v = value
//         .split(",")
//         .map(|x| x.trim().to_owned())
//         .collect::<Vec<String>>();
//     if v.is_empty() {
//         Err("must be non empty".to_owned())
//     } else {
//         Ok(v)
//     }
// }
fn parse_lang(value: &str) -> Result<oxdi::Language, String> {
    if let Some(lang) = oxdi::languages::Language::from_str(&value.to_lowercase()) {
        Ok(lang)
    } else {
        Err("invalid lang code, see documentation for valid language codes".to_owned())
    }
}
fn parse_word(value: &str) -> Result<String, String> {
    if value.is_empty() {
        Err("must be non empty".to_owned())
    // if at least one char is non alphabetic -> error
    } else if value.chars().all(|x| x.is_alphabetic()) == false {
        Err("must contain only alphabetic characters".to_owned())
    } else {
        Ok(value.to_owned())
    }
}

fn main() -> Result<()> {
    futures::executor::block_on(run())
}

async fn run() -> Result<()> {
    dotenv::dotenv().ok();
    let args: Args = argh::from_env();
    if args.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let word = args.word.as_str();

    #[cfg(not(feature = "embed-keys"))]
    let (id, key) = (
        std::env::var("OXFORD_API_ID").unwrap(),
        std::env::var("OXFORD_API_KEY").unwrap(),
    );
    #[cfg(feature = "embed-keys")]
    let (id, key) = (
        dotenv_codegen::dotenv!("OXFORD_API_ID").to_owned(),
        dotenv_codegen::dotenv!("OXFORD_API_KEY").to_owned(),
    );
    let client = oxdi::Client::new(id, key, oxdi::Language::EnGb);
    // let subscriber = FmtSubscriber::builder()
    //     .pretty()
    //     .with_env_filter(EnvFilter::from_default_env())
    //     // .with_max_level(Level::TRACE)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let resp: oxdi::models::RetrieveEntry = client
        .entries(
            word,
            &[Param::Fields(&[
                Field::Pronunciations,
                Field::Examples,
                Field::Definitions,
            ])],
        )
        .await
        .context("request to HTTP API failed")?;

    for headword_entry in resp.results.context("no results returned")? {
        // let word_type = headword_entry.type_.unwrap();
        for lexical_entry in headword_entry.lexical_entries {
            let text = lexical_entry.text;
            let category = lexical_entry.lexical_category.text;
            let prety_print = match lexical_entry.lexical_category.id.as_str() {
                "noun" => {
                    |text, category| bunt::println!("{[underline]}, {[cyan]}", text, category)
                }
                "verb" => {
                    |text, category| bunt::println!("{[underline]}, {[magenta]}", text, category)
                }
                _ => |text, category| bunt::println!("{[underline]}, {}", text, category),
                // other => todo!("{}", other),
            };
            prety_print(&text, &category);
            //
            println!("Pronunciations:");
            // println!("\t{}:", "Pronunciations");
            for entry in lexical_entry
                .entries
                .context("no entries in lexical entry")?
            {
                for pron in entry.pronunciations.context("no pronunciations")? {
                    bunt::println!(
                        "\t/{[blue]}/, {}",
                        pron.phonetic_spelling.context("no phonetic spelling")?,
                        pron.dialects.context("no dialects")?.join(", ")
                    );
                }
                if let Some(senses) = entry.senses.as_ref() {
                    bunt::println!("\t{$green}Definitions:{/$}");
                    for (i, sense) in senses.iter().enumerate() {
                        for (i_defi, defi) in sense
                            .definitions
                            .as_ref()
                            .context("no definitions")?
                            .iter()
                            .enumerate()
                        {
                            println!("\t{}.{}) {}", i + 1, i_defi + 1, defi);
                        }
                    }
                    bunt::println!("\t{$bold}Examples:{/$}");
                    for (i, sense) in senses.iter().enumerate() {
                        if let Some(examples) = sense.examples.as_ref() {
                            for (i_example, example) in examples.iter().enumerate() {
                                println!("\t{}.{}) {}", i + 1, i_example + 1, example.text);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
