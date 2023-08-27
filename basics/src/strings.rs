pub fn strings_example() {
    println!("\n\n\nstrings and stuff.");

    let some_string: &str = "The main said \"Hello world\"";
    let some_raw_string = r#"The main said "Hello world""#;
    let some_raw_string_more_hashes = r##"The main said# "Hello world""##;
    let some_raw_string_2 = r"The main said Hello world\n\t";
    let some_json_string = r#"{
        "glossary": {
            "title": "example glossary",
            "GlossDiv": {
                "title": "S",
                "GlossList": {
                    "GlossEntry": {
                        "ID": "SGML",
                        "SortAs": "SGML",
                        "GlossTerm": "Standard Generalized Markup Language",
                        "Acronym": "SGML",
                        "Abbrev": "ISO 8879:1986",
                        "GlossDef": {
                            "para": "A meta-markup language, used to create markup languages such as DocBook.",
                            "GlossSeeAlso": ["GML", "XML"]
                        },
                        "GlossSee": "markup"
                    }
                }
            }
        }
    }"#;

    println!("{some_string}\n{some_raw_string}\n{some_raw_string_2}\n{some_json_string}");

    // concatenation
    let s1: String = String::from("Hello");
    let s2: &str = " world";
    let s3 = s1 + s2; // this is now in mem location of s1
    println!("we can use the '+' operator to concatenate String + &str to a String.\n{s3}");

    let string_1 = String::from("Hello");
    let string_2 = String::from(" world");
    let string_3 = string_1 + &string_2;
    println!("We can concatenate two String values using '+' by taking on of the strings as a reference:\n{string_3}");
}
