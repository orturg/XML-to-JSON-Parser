use pest::Parser;
use anyhow::*;
use pest::iterators::Pair;
use serde_json::{json, Value};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

pub fn start_parser(input: &str) -> Result<Value> {
    if input.trim().is_empty() { return Err(anyhow!("XML is empty")); }
    let parsed = Grammar::parse(Rule::xml, input)?.next().ok_or_else(|| anyhow!("Invalid XML"))?;

    let json = parse_element(parsed.into_inner().next().unwrap());

    return json;
}

pub fn parse_open_tag(pair: Pair<Rule>) -> (String, Vec<(String, String)>) {
    let mut attributes = vec![];

    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();

    for attribute in inner {
        if attribute.as_rule() == Rule::attribute {
            let mut a = attribute.into_inner();
            let key = a.next().unwrap().as_str().to_string();
            let value = a.next().unwrap().as_str().to_string();
            attributes.push((key, value));
        }
    }

    (name, attributes)
}

pub fn parse_element(pair: Pair<Rule>) -> Result<Value> {
    let mut inner = pair.into_inner();
    let open_tag = inner.next().unwrap();
    let mut close_tag = None;
    let (name, attributes) = parse_open_tag(open_tag);

    let mut nested_content: Vec<Value> = vec![];
    let mut text_content = String::new();

    while let Some(next) = inner.next() {
        match next.as_rule() {
            Rule::element => nested_content.push(parse_element(next)?),
            Rule::inner_text => {
                let text = next.as_str().trim();
                if !text.is_empty() { text_content.push_str(text); }
            }
            Rule::close_tag => {
                close_tag = Some(next.into_inner().next().unwrap().as_str().to_string());
            }
            _ => {}
        }
    }

    if let Some(close_tag_name) = close_tag {
        if close_tag_name != name { return Err(anyhow!("There are different open and close tags names")); }
    } else { 
        return Err(anyhow!("There are different open and close tags names"));
     }

    let mut json = serde_json::Map::new();

    for (attribute_name, attribute_value) in attributes {
        json.insert(format!("_{}", attribute_name), Value::String(attribute_value));
    }

    if !nested_content.is_empty() {
        for content in nested_content {
            let (object_name, object_content) = content.as_object().unwrap().iter().next().unwrap();
            json.insert(object_name.clone(), object_content.clone());
        }
    } else if !text_content.is_empty() {
        if json.is_empty() { return Ok(json!({ name: text_content })); }
        else { json.insert("_text".to_string(), Value::String(text_content)); }
    }

    let completed_json = json!({ name: json });
    Ok(completed_json)
}