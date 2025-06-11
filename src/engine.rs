use pest_derive::Parser;
use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct QueryParser;

enum Subject {
    Iri(String),
    Blank(String),
    Variable(String)
}
enum Predicate {
    Iri(String),
    A
}
enum Object {
    Iri(String),
    Blank(String),
    Variable(String),
    StringLiteral(String)
}
type Triple = (Subject, Predicate, Object);
type TripleList = Vec<Triple>;
type VariableList = Vec<String>;
enum Query {
    Select(VariableList, TripleList),
    Insert(TripleList)
}

fn build_ast(pair: Pair<Rule>) -> Result<Query, String> {
    return Err("Unimplemented".to_string())
}

fn parse(input: &str) -> Result<Query, String> {
    // Parse AST using PEST parser
    let mut pairs = match QueryParser::parse(Rule::query, input) {
        Ok(p) => p,
        Err(e) => return Err(e.to_string())
    };
    // Construct query
    let top_level_pair = pairs.next().ok_or("Parsing error: PEST generated empty parse tree".to_string())?;
    // Check only one top node
    if pairs.next().is_some() {
        return Err("Parsing error: PEST generated multiple top-level nodes".to_string())
    }
    // Build AST from pair object
    build_ast(top_level_pair)
}