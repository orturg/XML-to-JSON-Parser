use pest::Parser;
use anyhow::anyhow;
use XML_to_JSON_Parser::*;
 
// inner_text tests 
#[test]
fn basic_text_test() -> anyhow::Result<()> {
    let test_text = "some text";
    let mut res = Grammar::parse(Rule::inner_text, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn parses_text_numbers_special_symbols() -> anyhow::Result<()> {
    let test_text = "1234 f./.ferf $% ^&*( some text";
    let mut res = Grammar::parse(Rule::inner_text, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn fails_when_open_bracket_appears_at_front() {
    let test_text = "<some text";
    let res = Grammar::parse(Rule::inner_text, test_text);
    assert!(res.is_err(), "Open bracket is in the string");
}

#[test]
fn fails_when_string_is_empty() {
    let test_text = "";
    let res = Grammar::parse(Rule::inner_text, test_text);
    assert!(res.is_err(), "String is empty");
}

// name tests

#[test]
fn basic_name_test() -> anyhow::Result<()> {
    let test_text = "SoMeText123";
    let mut res = Grammar::parse(Rule::name, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn fails_when_name_starts_not_ascii() {
    let test_text = ".SoMeText123";
    let res = Grammar::parse(Rule::name, test_text);
    assert!(res.is_err(), "String starts not from letter");
}

#[test]
fn read_until_name_not_ascii() -> anyhow::Result<()> {
    let test_text = "SoMe_Text-12%3";
    let mut res = Grammar::parse(Rule::name, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), "SoMe_Text-12");
    Ok(())
}

// attribute_value tests

#[test]
fn basic_attribute_value_test() -> anyhow::Result<()> {
    let test_text = "Some text 123";
    let mut res = Grammar::parse(Rule::attribute_value, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn attribute_value_reads_empty_string_if_quote_is_in_the_beginning() -> anyhow::Result<()> {
    let test_text = "\"some text 123";
    let mut res = Grammar::parse(Rule::attribute_value, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), "");
    Ok(())
}

#[test]
fn attribute_value_reads_until_quote() -> anyhow::Result<()> {
    let test_text = "some\" text 123";
    let mut res = Grammar::parse(Rule::attribute_value, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), "some");
    Ok(())
}

#[test]
fn attribute_value_reads_until_quote_in_the_end() -> anyhow::Result<()> {
    let test_text = "some text 123\"";
    let mut res = Grammar::parse(Rule::attribute_value, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), "some text 123");
    Ok(())
}

// attributes_tests

#[test]
fn basic_attribute_test() -> anyhow::Result<()> {
    let test_text = "id=\"1\"";
    let mut res = Grammar::parse(Rule::attribute, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn basic_attribute_test_whitespaces() -> anyhow::Result<()> {
    let test_text = "id = \"1\"";
    let mut res = Grammar::parse(Rule::attribute, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn attribute_without_quotes_test() {
    let test_text = "id = 1";
    let res = Grammar::parse(Rule::attribute, test_text);
    assert!(res.is_err(), "No quotes in attribute value");
}

#[test]
fn attribute_string_value_test() -> anyhow::Result<()> {
    let test_text = "id = \"qwerty123\"";
    let mut res = Grammar::parse(Rule::attribute, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn attribute_without_equal_test() {
    let test_text = "id \"qwerty123\"";
    let res = Grammar::parse(Rule::attribute, test_text);
    assert!(res.is_err(), "No equal in string");
}

#[test]
fn attribute_name_test() {
    let test_text = " = \"qwerty123\"";
    let res = Grammar::parse(Rule::attribute, test_text);
    assert!(res.is_err(), "No attribute name");
}

#[test]
fn complex_attribute_name_test() -> anyhow::Result<()> {
    let test_text = "id_1 = \"qwerty123\"";
    let mut res = Grammar::parse(Rule::attribute, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn attribute_empty_value_test() -> anyhow::Result<()> {
    let test_text = "id_1 = \"\"";
    let mut res = Grammar::parse(Rule::attribute, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

//open tag tests

#[test]
fn basic_open_tag_test() -> anyhow::Result<()> {
    let test_text = "<q>";
    let mut res = Grammar::parse(Rule::open_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn open_tag_with_attributes_test() -> anyhow::Result<()> {
    let test_text = "<q id_1 = \"qwerty123\">";
    let mut res = Grammar::parse(Rule::open_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn open_tag_without_name_test() {
    let test_text = "< id_1 = \"qwerty123\">";
    let res = Grammar::parse(Rule::open_tag, test_text);
    assert!(res.is_err(), "No tag name");
}

#[test]
fn open_tag_without_open_bracket_test() {
    let test_text = "q id_1 = \"qwerty123\">";
    let res = Grammar::parse(Rule::open_tag, test_text);
    assert!(res.is_err(), "No open bracket");
}

#[test]
fn open_tag_without_close_bracket_test() {
    let test_text = "q id_1 = \"qwerty123\"";
    let res = Grammar::parse(Rule::open_tag, test_text);
    assert!(res.is_err(), "No close bracket");
}

#[test]
fn open_tag_with_multiple_attributes_test() -> anyhow::Result<()> {
    let test_text = "<q id_1 = \"qwerty123\" id2 = \"id2\" id3 = \"id3\">";
    let mut res = Grammar::parse(Rule::open_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn open_tag_with_underlines_attributes_test() -> anyhow::Result<()> {
    let test_text = "<q_q id_1 = \"qwerty123\" id2 = \"id2\" id3 = \"id3\">";
    let mut res = Grammar::parse(Rule::open_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn open_tag_with_dashes_attributes_test() -> anyhow::Result<()> {
    let test_text = "<q-q-q id_1 = \"qwerty123\" id2 = \"id2\" id3 = \"id3\">";
    let mut res = Grammar::parse(Rule::open_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn open_tag_with_whitespaces_test() -> anyhow::Result<()> {
    let test_text = "<q-q-q    >";
    let mut res = Grammar::parse(Rule::open_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn open_tag_with_attribute_without_value_test() {
    let test_text = "<q-q-q id = >";
    let res = Grammar::parse(Rule::open_tag, test_text);
    assert!(res.is_err(), "No close bracket");
}

#[test]
fn open_tag_with_attribute_without_name_test() {
    let test_text = "<q-q-q = \"1\">";
    let res = Grammar::parse(Rule::open_tag, test_text);
    assert!(res.is_err(), "No close bracket");
}

//close tag tests

#[test]
fn basic_close_tag_test() -> anyhow::Result<()> {
    let test_text = "</q>";
    let mut res = Grammar::parse(Rule::close_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn close_tag_without_name_test() {
    let test_text = "</>";
    let res = Grammar::parse(Rule::close_tag, test_text);
    assert!(res.is_err(), "No tag name");
}

#[test]
fn close_tag_without_slash_test() {
    let test_text = "<tag>";
    let res = Grammar::parse(Rule::close_tag, test_text);
    assert!(res.is_err(), "No slash");
}

#[test]
fn close_tag_without_open_bracket_test() {
    let test_text = "/q>";
    let res = Grammar::parse(Rule::close_tag, test_text);
    assert!(res.is_err(), "No open bracket");
}

#[test]
fn close_tag_without_close_bracket_test() {
    let test_text = "</q";
    let res = Grammar::parse(Rule::close_tag, test_text);
    assert!(res.is_err(), "No close bracket");
}

#[test]
fn close_tag_with_underlines_name_test() -> anyhow::Result<()> {
    let test_text = "</test_test>";
    let mut res = Grammar::parse(Rule::close_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn close_tag_with_dashes_name_test() -> anyhow::Result<()> {
    let test_text = "</test-test-test>";
    let mut res = Grammar::parse(Rule::close_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn close_tag_with_whitespaces_test() -> anyhow::Result<()> {
    let test_text = "</     test     >";
    let mut res = Grammar::parse(Rule::close_tag, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

// element tests

#[test]
fn basic_element_test() -> anyhow::Result<()> {
    let test_text = "<title>title</title>";
    let mut res = Grammar::parse(Rule::element, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn element_with_attributes_test() -> anyhow::Result<()> {
    let test_text = "<title id =\"1\">title</title>";
    let mut res = Grammar::parse(Rule::element, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn element_with_nested_elements_test() -> anyhow::Result<()> {
    let test_text = r#"<title>
            <title2 id="1">title2</title2>
            <qwerty>qwerty</qwerty>
        </title>"#;
    let mut res = Grammar::parse(Rule::element, test_text)?;
    let pair = res.next().unwrap();
    let parsed = pair.as_str().chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let expected = test_text.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn element_only_with_tags_test() -> anyhow::Result<()> {
    let test_text = "<title></title>";
    let mut res = Grammar::parse(Rule::element, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn element_without_open_tag_test() {
    let test_text = "Title</title>";
    let res = Grammar::parse(Rule::element, test_text);
    assert!(res.is_err(), "No open tag");
}

#[test]
fn element_without_close_tag_test() {
    let test_text = "<title>Title";
    let res = Grammar::parse(Rule::element, test_text);
    assert!(res.is_err(), "No close tag");
}

// xml tests

#[test]
fn basic_xml_test() -> anyhow::Result<()> {
    let test_text = "<title>title</title>";
    let mut res = Grammar::parse(Rule::xml, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn with_attributes_xml_test() -> anyhow::Result<()> {
    let test_text = "<title id = \"1\">title</title>";
    let mut res = Grammar::parse(Rule::xml, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn nested_xml_test() -> anyhow::Result<()> {
    let test_text = "<title id = \"1\"> <title2 id = \"2\">title2</title2></title>";
    let mut res = Grammar::parse(Rule::xml, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn nested_formatted_xml_test() -> anyhow::Result<()> {
    let test_text = r#"<title id = "1">
            <title2 id = "2">title2</title2>
            <qwerty>qwerty</qwerty>
        </title>"#;
    let mut res = Grammar::parse(Rule::xml, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn xml_without_open_tag_test() {
    let test_text = "title</title>";
    let res = Grammar::parse(Rule::xml, test_text);
    assert!(res.is_err(), "No open tag");
}

#[test]
fn xml_without_close_tag_test() {
    let test_text = "<title>title";
    let res = Grammar::parse(Rule::xml, test_text);
    assert!(res.is_err(), "No close tag");
}

#[test]
fn empty_xml_test() -> anyhow::Result<()> {
    let test_text = "<title></title>";
    let mut res = Grammar::parse(Rule::xml, test_text)?;
    let pair = res.next().unwrap();
    assert_eq!(pair.as_str(), test_text);
    Ok(())
}

#[test]
fn empty_string_xml_test() {
    let test_text = "";
    let res = Grammar::parse(Rule::xml, test_text);
    assert!(res.is_err(), "String is empty");
}


