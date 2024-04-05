use rusqlite::{Connection, Transaction};

use crate::{
    kanjidic2::{Header, KanjiReadingType, Kanjidic2},
    Result,
};

const MIGRATION_QUERY: &str = include_str!("../../queries/migrations/kanjidic2.sql");
const METADATA_QUERY: &str = "INSERT INTO metadata VALUES (?, ?)";
const CHARACTER_QUERY: &str = "INSERT INTO characters VALUES (?, ?, ?, ?, ?)";
const CODEPOINT_QUERY: &str = "INSERT INTO codepoints (type, value, character) VALUES (?, ?, ?)";
const VARIANT_QUERY: &str = "INSERT INTO variants (type, value, character) VALUES (?, ?, ?)";
const CHAR_RAD_QUERY: &str = "INSERT INTO character_radical VALUES (?, ?)";
const ON_QUERY: &str = "INSERT INTO on_readings (reading, character) VALUES (?, ?)";
const KUN_QUERY: &str = "INSERT INTO kun_readings (reading, character) VALUES (?, ?)";
const NANORI_QUERY: &str = "INSERT INTO nanori_readings (reading, character) VALUES (?, ?)";
const PINYIN_QUERY: &str = "INSERT INTO pinyin_readings (reading, character) VALUES (?, ?)";
const KOREANR_QUERY: &str = "INSERT INTO korean_r_readings (reading, character) VALUES (?, ?)";
const KOREANH_QUERY: &str = "INSERT INTO korean_h_readings (reading, character) VALUES (?, ?)";
const VIETNAM_QUERY: &str = "INSERT INTO vietnam_readings (reading, character) VALUES (?, ?)";
const MEANING_QUERY: &str = "INSERT INTO meanings (meaning, lang, character) VALUES (?, ?, ?)";
const QCODE_QUERY: &str =
    "INSERT INTO query_codes (type, skip_misclass, code, character) VALUES (?, ?, ?, ?)";
const DIC_QUERY: &str =
    "INSERT INTO dic_numbers (type, volume, page, reference, character) VALUES (?, ?, ?, ?, ?)";

pub fn create_sqlite_db(kanjidc2: Kanjidic2, path: &str) -> Result<()> {
    let mut conn = Connection::open(path)?;

    conn.execute_batch(MIGRATION_QUERY)?;

    let tx = conn.transaction()?;

    prepare_transaction(&tx, kanjidc2)?;

    tx.commit()?;

    Ok(())
}

fn prepare_transaction(tx: &Transaction, kanjidc2: Kanjidic2) -> Result<()> {
    let Header {
        file_version,
        database_version,
        date_of_creation,
    } = kanjidc2.header;

    tx.execute(METADATA_QUERY, ("file_version", file_version))?;
    tx.execute(METADATA_QUERY, ("database_version", database_version))?;
    tx.execute(METADATA_QUERY, ("date_of_creation", date_of_creation))?;

    let mut char_stmt = tx.prepare(CHARACTER_QUERY)?;
    let mut codepoint_stmt = tx.prepare(CODEPOINT_QUERY)?;
    let mut variant_stmt = tx.prepare(VARIANT_QUERY)?;
    let mut dic_stmt = tx.prepare(DIC_QUERY)?;
    let mut qcode_stmt = tx.prepare(QCODE_QUERY)?;
    let mut char_rad_stmt = tx.prepare(CHAR_RAD_QUERY)?;
    let mut on_stmt = tx.prepare(ON_QUERY)?;
    let mut kun_stmt = tx.prepare(KUN_QUERY)?;
    let mut nanori_stmt = tx.prepare(NANORI_QUERY)?;
    let mut pinyin_stmt = tx.prepare(PINYIN_QUERY)?;
    let mut korean_r_stmt = tx.prepare(KOREANR_QUERY)?;
    let mut korean_h_stmt = tx.prepare(KOREANH_QUERY)?;
    let mut vietnam_stmt = tx.prepare(VIETNAM_QUERY)?;
    let mut meaning_stmt = tx.prepare(MEANING_QUERY)?;

    for ch in kanjidc2.character {
        let literal = ch.literal.to_string();

        char_stmt.execute((
            &literal,
            ch.misc.freq,
            ch.misc.grade,
            ch.misc.stroke_count,
            ch.misc.jlpt,
        ))?;

        for codepoint in ch.codepoint.values {
            codepoint_stmt.execute((codepoint.cp_type.to_string(), codepoint.value, &literal))?;
        }

        for variant in ch.misc.variant {
            variant_stmt.execute((variant.var_type.to_string(), variant.value, &literal))?;
        }

        for radical in ch.radical.values {
            char_rad_stmt.execute((&literal, radical.value))?;
        }

        if let Some(dic_number) = ch.dic_number {
            for dic in dic_number.values {
                dic_stmt.execute((
                    dic.dr_type.to_string(),
                    dic.m_vol,
                    dic.m_page,
                    dic.value,
                    &literal,
                ))?;
            }
        }

        if let Some(query_code) = ch.query_code {
            for code in query_code.values {
                qcode_stmt.execute((
                    code.qc_type.to_string(),
                    code.skip_misclass.map(|m| m.to_string()),
                    code.value,
                    &literal,
                ))?;
            }
        }

        let Some(rm) = ch.reading_meaning else { continue };

        for nanori in rm.nanori {
            nanori_stmt.execute((nanori, &literal))?;
        }

        let Some(group) = rm.rmgroup else { continue };

        for reading in group.reading {
            let stmt = match reading.r_type {
                KanjiReadingType::JaOn => &mut on_stmt,
                KanjiReadingType::JaKun => &mut kun_stmt,
                KanjiReadingType::Pinyin => &mut pinyin_stmt,
                KanjiReadingType::KoreanR => &mut korean_r_stmt,
                KanjiReadingType::KoreanH => &mut korean_h_stmt,
                KanjiReadingType::Vietnam => &mut vietnam_stmt,
            };

            stmt.execute((reading.value, &literal))?;
        }

        for meaning in group.meaning {
            meaning_stmt.execute((meaning.value, meaning.m_lang, &literal))?;
        }
    }

    Ok(())
}
