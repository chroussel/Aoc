mod r2018_1;

use failure::Error;
use std::collections;
use std::collections::HashMap;
use std::env::VarError::NotPresent;

#[derive(Debug, PartialEq, Eq, Hash)]
struct InputDay {
    year: String,
    day: String,
    part: String
}

impl InputDay {
    fn new(year: &str, day: &str, part: &str) -> Self {
        InputDay {
            year: year.to_string(),
            day:day.to_string(),
            part: part.to_string()
        }
    }
}

type RunFn = fn(&str) -> Result<String, Error>;

lazy_static! {
    static ref RUN_MAP: HashMap<InputDay, RunFn> = init_run_map();
}

fn init_run_map() -> HashMap<InputDay, fn(&str) ->Result<String, Error>>{
    let mut hash_map = collections::HashMap::new();
    hash_map.insert(InputDay::new("2018", "1", "1"), r2018_1::run1 as RunFn);
    hash_map.insert(InputDay::new("2018", "1", "2"), r2018_1::run2 as RunFn);
    hash_map
}

pub fn run(year: &str, day: &str, part: &str, input: &str) -> Result<String, Error> {
    match RUN_MAP.get(&InputDay::new(year, day, part)) {
        Some(f)=> f(input),
        None => Err(Error::from(NotPresent))
    }
}