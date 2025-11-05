use pest::Parser;
use anyhow::anyhow;
use XML_to_JSON_Parser::*;
 
#[test]
fn basic_text_test() -> anyhow::Result<()> {
    let test_text = "some text";
    let mut res = Grammar::parse(Rule::inner_text, "some text")?;
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