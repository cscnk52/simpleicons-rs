use std::{
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

use serde_json::Value;
use unicode_normalization::UnicodeNormalization;

use crate::constants::{REMOVE_REGEX, TITLE_TO_SLUG_REPLACEMENTS};

/// convert simple icons title to slug
/// see [Contribution](https://github.com/simple-icons/simple-icons/blob/develop/CONTRIBUTING.md#6-name-the-icon) doc for more information
pub fn title_to_slug(title: &str) -> String {
    let mut replaced: String = String::with_capacity(title.len());

    for c in title.to_lowercase().chars() {
        if let Some(replacement) = TITLE_TO_SLUG_REPLACEMENTS.get(&c) {
            replaced.push_str(replacement);
        } else {
            replaced.push(c);
        }
    }

    let decomposed: String = replaced.nfd().collect::<String>();

    REMOVE_REGEX.replace_all(&decomposed, "").to_string()
}

// #[cfg(test)]
// mod title_to_slug_test {
//     use crate::simple_icons::title_to_slug;

//     #[test]
//     fn use_lowercase_letters_without_whitespace() {
//         assert_eq!(title_to_slug("Adobe Photoshop"), "adobephotoshop");

//         assert_eq!(title_to_slug("AAAAA"), "aaaaa");
//         assert_eq!(title_to_slug("     "), "");
//     }

//     #[test]
//     fn only_use_latin_letters() {
//         assert_eq!(title_to_slug("Citroën"), "citroen");

//         assert_eq!(title_to_slug("Café"), "cafe");
//         assert_eq!(title_to_slug("Straße"), "strasse");
//         assert_eq!(title_to_slug("Málaga"), "malaga");
//         assert_eq!(title_to_slug("Crème brûlée"), "cremebrulee");
//     }

//     #[test]
//     fn replace_symbols_with_alias() {
//         assert_eq!(title_to_slug("+"), "plus");
//         assert_eq!(title_to_slug("."), "dot");
//         assert_eq!(title_to_slug("&"), "and");
//         assert_eq!(title_to_slug(".net"), "dotnet");

//         assert_eq!(title_to_slug("C++"), "cplusplus");
//     }
// }

pub fn file_to_json(json_file_path: PathBuf) -> Result<Value, Error> {
    let file: File = File::open(json_file_path)?;
    let reader: BufReader<&File> = BufReader::new(&file);
    let json: Value = serde_json::from_reader(reader)?;

    Ok(json)
}
