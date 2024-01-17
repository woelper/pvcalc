use std::collections::HashMap;


// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// struct Dictionary {
//     translations: HashMap<String, HashMap<String>>,
// }

#[macro_export]
macro_rules! tr {

    ($($arg:tt)*) => {{

        // let mut dict : HashMap<String,HashMap<String,String>> = std::fs::File::open("translations.json").map(|f| serde_json::from_reader(f).unwrap_or_default()).unwrap_or_default();

        let res = format!($($arg)*);
        // let res = format!($($arg)*);
        // let locale = sys_locale::get_locale().unwrap_or("de".to_string());
        // dict.entry(res.clone()).or_default().insert(locale, res.clone());

        // _ = serde_json::to_writer_pretty(std::fs::File::create("translations.json").unwrap(), &dict);

        // dict.insert(locale, )

        res
    }}
}
