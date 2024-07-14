use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "../oml.pest"]
pub struct OmlParser;

#[derive(Debug)]
pub enum OmlValue {
    Bool(bool),
    Int64(i64),
    Float64(f64),
    String(String),
    Array(Vec<OmlValue>),
    Map(HashMap<String, OmlValue>),
}

impl OmlValue {
    pub fn new_map() -> Self {
        Self::Map(HashMap::new())
    }

    pub fn from_str(content: &str) -> Result<OmlValue, String> {
        match OmlParser::parse(Rule::oml, content) {
            Ok(mut root) => Self::parse_oml(root.next().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn apply(&mut self, val: OmlValue) {
        // TODO
    }

    fn parse_oml(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlValue, String> {
        let mut ret = Self::new_map();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::group_block => {
                    let val = Self::parse_block(root_item)?;
                    ret.apply(val);
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        Ok(ret)
    }

    fn parse_block(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlValue, String> {
        let mut ret = HashMap::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::group_head => (),       //TODO
                Rule::group_array_head => (), //TODO
                Rule::assign_pair => (),      //TODO
                _ => unreachable!(),
            }
        }
        Ok(Self::Map(ret))
    }
}
