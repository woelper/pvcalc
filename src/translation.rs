use std::collections::HashMap;


// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// struct Dictionary {
//     translations: HashMap<String, HashMap<String>>,
// }

#[macro_export]
macro_rules! tr {

    ($($arg:tt)*) => {{


        if false
        {
            let mut dict : std::collections::HashMap<String, std::collections::HashMap<String,String>> = std::fs::File::open("translations.json").map(|f| serde_json::from_reader(f).unwrap_or_default()).unwrap_or_default();
            let pattern_str = stringify!($($arg)*).to_string().replace("\"","");
            let args =  pattern_str.split(',').nth(0).unwrap_or(&pattern_str);
            println!("Pattern before expansion: {}", args);
            let locale = sys_locale::get_locale().unwrap_or("de".to_string());
            dict.entry(args.to_string()).or_default().insert(locale, args.to_string());
            _ = serde_json::to_writer_pretty(std::fs::File::create("translations.json").unwrap(), &dict);
        }
        
       
        let res = format!($($arg)*);
        // let res = format!($($arg)*);


        // dict.insert(locale, )

        res
    }}
}




// #[macro_export]
// macro_rules! tr {
//     ($($arg:tt)*) => {{
//         let input_str = stringify!($($arg)*);
//         println!("Input before expansion: {}", input_str);
//         let res = format!($($arg)*);
//         res
//     }}
// }