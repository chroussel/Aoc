use failure::Error;

pub fn run1(input: &str) ->Result<String, Error> {
    let res: i32 = input.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l|l.parse::<i32>().map_err(From::from))
        .collect::<Result<Vec<i32>, Error>>()?
        .iter()
        .sum();
    Ok(res.to_string())
}

pub fn run2(input: &str) ->Result<String, Error> {
    let res: Vec<i32> = input.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l|l.parse::<i32>().map_err(From::from))
        .collect::<Result<Vec<i32>, Error>>()?;
    let mut hashSet = std::collections::HashSet::new();
    let mut running = 0;
    loop {
        for vi in &res {
            running += vi;
            if hashSet.contains(&running) {
                return Ok(running.to_string())
            } else {
                hashSet.insert(running);
            }
        }
    }
    Ok(String::from("Nothing"))
}