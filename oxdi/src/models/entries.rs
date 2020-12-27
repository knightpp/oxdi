use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct RetrieveEntry {
    pub metadata: Option<Value>,
    pub results: Option<Vec<HeadwordEntry>>,
}
#[derive(Debug, Deserialize)]
pub struct HeadwordEntry {
    pub id: String,
    pub language: String,
    #[serde(rename = "lexicalEntries")]
    pub lexical_entries: Vec<LexicalEntry>,
    pub pronunciations: Option<Vec<Pronunciation>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[deprecated = "TODO: find out reason"]
    pub word: String,
}

#[derive(Debug, Deserialize)]
pub struct LexicalEntry {
    pub compounds: Option<Vec<RelatedEntry>>,
    #[serde(rename = "derivativeOf")]
    pub derivative_of: Option<Vec<RelatedEntry>>,
    pub derivatives: Option<Vec<RelatedEntry>>,
    pub entries: Option<Vec<Entry>>,
    #[serde(rename = "grammaticalFeatures")]
    pub grammatical_features: Option<Vec<GrammaticalFeature>>,
    pub language: String,
    #[serde(rename = "lexicalCategory")]
    pub lexical_category: LexicalCategory,
    pub notes: Option<Vec<CategorizedText>>,
    #[serde(rename = "phrasalVerbs")]
    pub phrasal_verbs: Option<Vec<RelatedEntry>>,
    pub phrases: Option<Vec<RelatedEntry>>,
    pub pronunciations: Option<Vec<Pronunciation>>,
    pub root: Option<String>,
    pub text: String,
    #[serde(rename = "variantForms")]
    pub variant_forms: Option<Vec<VariantForm>>,
}

#[derive(Debug, Deserialize)]
pub struct Pronunciation {
    #[serde(rename = "audioFile")]
    pub audio_file: Option<String>,
    pub dialects: Option<Vec<String>>,
    #[serde(rename = "phoneticNotation")]
    pub phonetic_notation: Option<String>,
    #[serde(rename = "phoneticSpelling")]
    pub phonetic_spelling: Option<String>,
    pub regions: Option<Vec<Region>>,
    pub registers: Option<Vec<Register>>,
}

#[derive(Debug, Deserialize)]
pub struct RelatedEntry {
    pub domains: Option<Vec<Domain>>,
    pub id: String,
    pub language: Option<String>,
    pub regions: Option<Vec<Region>>,
    pub registers: Option<Vec<Register>>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(rename = "crossReferenceMarkers")]
    pub cross_reference_markers: Option<Vec<String>>,
    #[serde(rename = "crossReferences")]
    pub cross_references: Option<Vec<CrossReference>>,
    pub etymologies: Option<Vec<String>>,
    #[serde(rename = "grammaticalFeatures")]
    pub grammatical_features: Option<Vec<GrammaticalFeature>>,
    #[serde(rename = "homographNumber")]
    pub homograph_number: Option<String>,
    pub inflections: Option<Vec<InflectedForm>>,
    pub notes: Option<Vec<CategorizedText>>,
    pub pronunciations: Option<Vec<Pronunciation>>,
    pub senses: Option<Vec<Sense>>,
    #[serde(rename = "variantForms")]
    pub variant_forms: Option<Vec<VariantForm>>,
}

#[derive(Debug, Deserialize)]
pub struct GrammaticalFeature {
    pub id: String,
    pub text: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct LexicalCategory {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct CategorizedText {
    pub id: Option<String>,
    pub text: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct VariantForm {
    pub domains: Option<Vec<Domain>>,
    pub notes: Option<Vec<CategorizedText>>,
    pub pronunciations: Option<Vec<Pronunciation>>,
    pub regions: Option<Vec<Region>>,
    pub registers: Option<Vec<Register>>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct CrossReference {
    pub id: String,
    pub text: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct InflectedForm {
    pub domains: Option<Vec<Domain>>,
    #[serde(rename = "grammaticalFeatures")]
    pub grammatical_features: Option<Vec<GrammaticalFeature>>,
    #[serde(rename = "inflectedForm")]
    pub inflected_form: String,
    #[serde(rename = "lexicalCategory")]
    pub lexical_category: Option<LexicalCategory>,
    pub pronunciations: Option<Vec<Pronunciation>>,
    pub regions: Option<Vec<Region>>,
    pub registers: Option<Vec<Register>>,
}

#[derive(Debug, Deserialize)]
pub struct Sense {
    pub antonyms: Option<Vec<SynonymAntonym>>,
    pub constructions: Option<Vec<InlineModel2>>,
    #[serde(rename = "crossReferenceMarkers")]
    pub cross_reference_markers: Option<Vec<String>>,
    #[serde(rename = "crossReferences")]
    pub cross_references: Option<Vec<CrossReference>>,
    pub definitions: Option<Vec<String>>,
    #[serde(rename = "domainClasses")]
    pub domain_classes: Option<Vec<DomainClass>>,
    pub domains: Option<Vec<Domain>>,
    pub etymologies: Option<Vec<String>>,
    pub examples: Option<Vec<Example>>,
    pub id: Option<String>,
    pub inflections: Option<Vec<InflectedForm>>,
    pub notes: Option<Vec<CategorizedText>>,
    pub pronunciations: Option<Vec<Pronunciation>>,
    pub regions: Option<Vec<Region>>,
    pub registers: Option<Vec<Register>>,
    #[serde(rename = "semanticClasses")]
    pub semantic_classes: Option<Vec<SemanticClass>>,
    #[serde(rename = "shortDefinitions")]
    pub short_definitions: Option<Vec<String>>,
    pub subsenses: Option<Vec<Sense>>,
    pub synonyms: Option<Vec<SynonymAntonym>>,
    #[serde(rename = "thesaurusLinks")]
    pub thesaurus_links: Option<Vec<ThesaurusLink>>,
    #[serde(rename = "variantForms")]
    pub variant_forms: Option<Vec<VariantForm>>,
}
#[derive(Debug, Deserialize)]
pub struct InlineModel2 {
    pub domains: Option<Vec<Domain>>,
    pub examples: Option<Vec<ExampleText>>,
    pub notes: Option<Vec<CategorizedText>>,
    pub regions: Option<Vec<Region>>,
    pub registers: Option<Vec<Register>>,
    pub text: String,
}
#[derive(Debug, Deserialize)]
pub struct SynonymAntonym {
    pub domains: Option<Vec<Domain>>,
    pub id: Option<String>,
    pub language: Option<String>,
    pub regions: Option<Vec<Region>>,
    pub registers: Option<Vec<Register>>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ExampleText(Vec<String>);

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Region {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Register {
    pub id: String,
    pub text: String,
}
#[derive(Debug, Deserialize)]
pub struct DomainClass {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Example {
    pub definitions: Option<Vec<String>>,
    pub domains: Option<Vec<Domain>>,
    pub notes: Option<Vec<CategorizedText>>,
    pub regions: Option<Vec<Region>>,
    pub registers: Option<Vec<Register>>,
    #[serde(rename = "senseIds")]
    pub sense_ids: Option<Vec<String>>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct SemanticClass {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ThesaurusLink {
    pub entry_id: String,
    pub sense_id: String,
}
