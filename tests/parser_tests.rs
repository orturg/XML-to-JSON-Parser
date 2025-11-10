use anyhow::anyhow;
use XML_to_JSON_Parser::*;
use serde_json::json;

#[test]
fn basic_parser_test() -> anyhow::Result<()> {
    let test_text = r#"
     <parser>
        <title>XML_to_JSON</title>
        <author>Artur Nozhenko</author>
    </parser>
    "#;

    let parsed = start_parser(test_text)?;
    let expected = json!({
        "parser": {
            "title": "XML_to_JSON",
            "author": "Artur Nozhenko"
        }
    });

    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn without_nested_elements_test() -> anyhow::Result<()> {
    let test_text = r#"<parser>XML_to_JSON</parser>"#;
    let parsed = start_parser(test_text)?;
    let expected = json!({ "parser": "XML_to_JSON" });
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn different_tags_test() {
    let test_text = r#"
     <parser>
        <title>XML_to_JSON</title>
        <author>Artur Nozhenko</author>
    </qwertyr>
    "#;

    let parsed = start_parser(test_text);
    assert!(parsed.is_err())
}

#[test]
fn different_nested_tags_test() {
    let test_text = r#"
     <parser>
        <title>XML_to_JSON</qwerty>
        <author>Artur Nozhenko</author>
    </parser>
    "#;

    let parsed = start_parser(test_text);
    assert!(parsed.is_err())
}

#[test]
fn empty_text_test() {
    let test_text = "";

    let parsed = start_parser(test_text);
    assert!(parsed.is_err())
}

#[test]
fn tags_without_text_test() -> anyhow::Result<()> {
    let test_text = r#"
     <parser></parser>
    "#;

    let parsed = start_parser(test_text)?;
    let expected = json!({ "parser": {} });
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn underline_attributes_test() -> anyhow::Result<()> {
    let test_text = r#"<parser id = "1" author = "Artur Nozhenko"></parser>"#;
    let parsed = start_parser(test_text)?;
    let expected = json!({ 
        "parser": {
            "_id": "1",
            "_author": "Artur Nozhenko"
        } 
    });
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn underline_attributes_and_text_test() -> anyhow::Result<()> {
    let test_text = r#"<parser id = "1" author = "Artur Nozhenko">XML_to_JSON</parser>"#;
    let parsed = start_parser(test_text)?;
    let expected = json!({ 
        "parser": {
            "_id": "1",
            "_author": "Artur Nozhenko",
            "_text": "XML_to_JSON"
        } 
    });
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn without_open_tag_test() {
    let test_text = r#"XML_to_JSON</parser>"#;
    let parsed = start_parser(test_text);
    assert!(parsed.is_err())
}

#[test]
fn without_close_tag_test() {
    let test_text = r#"<parser>XML_to_JSON"#;
    let parsed = start_parser(test_text);
    assert!(parsed.is_err())
}

#[test]
fn bad_tag_name_test() {
    let test_text = r#"<parser<vfvew>XML_to_JSON</parser<vfvew>"#;
    let parsed = start_parser(test_text);
    assert!(parsed.is_err())
}

#[test]
fn big_nested_json_test() -> anyhow::Result<()> {
    let test_text = r#"<parser><parser1><parser2><parser3>XML_to_JSON</parser3></parser2></parser1></parser>"#;
    let parsed = start_parser(test_text)?;
    let expected = json!({ 
        "parser": {
            "parser1": {
                "parser2": {
                    "parser3": "XML_to_JSON"
                }
            }
        } 
    });
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn empty_attributes_value_test() -> anyhow::Result<()> {
    let test_text = r#"<parser id = "" author = "">XML_to_JSON</parser>"#;
    let parsed = start_parser(test_text)?;
    let expected = json!({ 
        "parser": {
            "_id": "",
            "_author": "",
            "_text": "XML_to_JSON"
        } 
    });
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn attributes_value_without_quotes_test() {
    let test_text = r#"<parser id = 1>XML_to_JSON</parser>"#;
    let parsed = start_parser(test_text);
    assert!(parsed.is_err())
}

#[test]
fn tag_with_numbers_name_test() -> anyhow::Result<()> {
    let test_text = r#"<parser111>XML_to_JSON</parser111>"#;

    let parsed = start_parser(test_text)?;
    let expected = json!({ "parser111": "XML_to_JSON" });
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn tag_name_starts_with_number_test() {
    let test_text = r#"<1parser>XML_to_JSON</1parser>"#;
    let parsed = start_parser(test_text);
    assert!(parsed.is_err())
}