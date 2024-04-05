use crate::utils::enum_display_impl;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Kanjidic2 {
    pub header: Header,
    pub character: Vec<Character>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub file_version: String,
    pub database_version: String,
    pub date_of_creation: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub literal: char,
    pub codepoint: CodePoint,
    pub radical: Radical,
    pub misc: Misc,
    pub dic_number: Option<DicNumber>,
    pub query_code: Option<QueryCodes>,
    pub reading_meaning: Option<ReadingMeaning>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodePoint {
    #[serde(rename(deserialize = "$value", serialize = "values"))]
    pub values: Vec<CodePointValue>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodePointValue {
    #[serde(rename(deserialize = "@cp_type", serialize = "cp_type"))]
    pub cp_type: CodePointType,
    #[serde(rename(deserialize = "$text", serialize = "value"))]
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CodePointType {
    Ucs,
    Jis208,
    Jis212,
    Jis213,
}

enum_display_impl!(CodePointType);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Radical {
    #[serde(rename(deserialize = "$value", serialize = "values"))]
    pub values: Vec<RadicalValue>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RadicalValue {
    #[serde(rename(deserialize = "@rad_type", serialize = "rad_type"))]
    pub rad_type: RadicalType,
    #[serde(rename(deserialize = "$text", serialize = "value"))]
    pub value: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RadicalType {
    Classical,
    NelsonC,
}

enum_display_impl!(RadicalType);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Misc {
    pub grade: Option<u8>,
    pub stroke_count: Vec<u8>,
    #[serde(default)]
    pub variant: Vec<KanjiVariant>,
    pub freq: Option<u16>,
    #[serde(default)]
    pub rad_name: Vec<String>,
    pub jlpt: Option<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KanjiVariant {
    #[serde(rename(deserialize = "@var_type", serialize = "var_type"))]
    pub var_type: KanjiVariantType,
    #[serde(rename(deserialize = "$text", serialize = "value"))]
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KanjiVariantType {
    Jis208,
    Jis212,
    Jis213,
    Deroo,
    Njecd,
    SH,
    NelsonC,
    Oneill,
    Ucs,
}

enum_display_impl!(KanjiVariantType);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DicNumber {
    #[serde(rename(deserialize = "$value", serialize = "values"))]
    pub values: Vec<DicRef>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DicRef {
    #[serde(rename(deserialize = "@dr_type", serialize = "dr_type"))]
    pub dr_type: DicRefType,
    #[serde(rename(deserialize = "@m_vol", serialize = "m_vol"))]
    pub m_vol: Option<u8>,
    #[serde(rename(deserialize = "@m_page", serialize = "m_page"))]
    pub m_page: Option<u16>,
    #[serde(rename(deserialize = "$text", serialize = "value"))]
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DicRefType {
    NelsonC,
    NelsonN,
    HalpernNjecd,
    HalpernKkd,
    HalpernKkld,
    #[serde(rename = "halpern_kkld_2ed")]
    HalpernKkld2ed,
    Heisig,
    Gakken,
    OneillNames,
    OneillKk,
    Moro,
    Henshall,
    ShKk,
    ShKk2,
    Sakade,
    JfCards,
    Henshall3,
    TuttCards,
    Crowley,
    KanjiInContext,
    BusyPeople,
    KodanshaCompact,
    Maniette,
    Heisig6,
}

enum_display_impl!(DicRefType);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryCodes {
    #[serde(rename(deserialize = "$value", serialize = "values"))]
    pub values: Vec<QueryCode>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryCode {
    #[serde(rename(deserialize = "@qc_type", serialize = "qc_type"))]
    pub qc_type: QueryCodeType,
    #[serde(rename(deserialize = "@skip_misclass", serialize = "skip_misclass"))]
    pub skip_misclass: Option<SkipMisclass>,
    #[serde(rename(deserialize = "$value", serialize = "value"))]
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryCodeType {
    Skip,
    ShDesc,
    FourCorner,
    Deroo,
    Misclass,
}

enum_display_impl!(QueryCodeType);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkipMisclass {
    Posn,
    StrokeCount,
    StrokeAndPosn,
    StrokeDiff,
}

enum_display_impl!(SkipMisclass);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadingMeaning {
    pub rmgroup: Option<ReadingMeaningGroup>,
    #[serde(default)]
    pub nanori: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadingMeaningGroup {
    #[serde(default)]
    pub reading: Vec<KanjiReading>,
    #[serde(default)]
    pub meaning: Vec<KanjiMeaning>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KanjiReading {
    #[serde(rename(deserialize = "@r_type", serialize = "r_type"))]
    pub r_type: KanjiReadingType,
    #[serde(rename(deserialize = "$text", serialize = "value"))]
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KanjiReadingType {
    Pinyin,
    KoreanR,
    KoreanH,
    JaOn,
    JaKun,
    Vietnam,
}

enum_display_impl!(KanjiReadingType);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KanjiMeaning {
    #[serde(
        rename(deserialize = "@m_lang", serialize = "m_lang"),
        default = "default_lang"
    )]
    pub m_lang: String,
    #[serde(rename(deserialize = "$text", serialize = "value"))]
    pub value: String,
}

fn default_lang() -> String {
    "en".to_string()
}
