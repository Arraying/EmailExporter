use std::{borrow::Cow, collections::HashMap, fs::read_to_string};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() <= 1 {
        panic!("Please provide the path to the file as the first argument")
    }

    let path = &args[1];

    let contents = read_to_string(path).unwrap_or_else(|_| panic!("File '{path}' not found"));

    let res = parse(&contents);

    println!("{res}");
}

fn parse(contents: &str) -> String {
    let mut map = HashMap::<&str, Vec<&str>>::new();

    let lines = contents.lines().enumerate().collect::<Vec<_>>();
    let mut relevant_lines: Vec<Cow<str>> = vec![];

    for (index, line) in &lines {
        if line.starts_with("From:") {
            let (_, next_line) = &lines[index + 1];

            // For whatever reason, some emails are written one line later
            if next_line.starts_with('\t') || next_line.starts_with(' ') {
                let combined = (*line).to_string() + &next_line.replacen('\t', " ", 1);
                relevant_lines.push(Cow::Owned(combined));
            } else {
                relevant_lines.push(Cow::Borrowed(*line));
            }
        }
    }

    for line in &relevant_lines {
        if let Some((domain, address)) = parse_line(line) {
            let entry = map.get_mut(domain);

            match entry {
                Some(entry) => {
                    if !entry.contains(&address) {
                        entry.push(address)
                    }
                }
                None => {
                    let _ = map.insert(domain, vec![address]);
                }
            };
        }
    }

    let mut res = String::new();
    map.keys().for_each(|key| {
        res.push_str(key);
        res.push('\n');

        let addresses = map.get(key).unwrap();

        for address in addresses {
            res.push('\t');
            res.push_str(address);
            res.push('\n');
        }
    });

    res
}

/// I will not document this so it looks more mysterious
fn parse_line(input: &str) -> Option<(&str, &str)> {
    if let Some(mid) = input.find(" <") {
        let relevant = &input[mid + 2..input.trim().len() - 1];
        let split = relevant.find('@').unwrap();
        return Some((&relevant[split + 1..], relevant));
    } else if let Some(mid) = input.find('@') {
        return Some((&input[mid + 1..], (input["From:".len()..].trim())));
    }

    None
}

/// The two tests here are made to compare the outputs of the rust and python versions of the
/// project. To run these, 2 files must be created inside the `rust-btw` folder, `tmp_py.txt`
/// and `tmp_rs.txt` containing the output of the python and rust code respectively.
#[cfg(test)]
mod tests {
    #[test]
    fn test_py_not_contained_in_rs() {
        let contents_py = include_str!("../tmp_py.txt");
        let contents_rs = include_str!("../tmp_rs.txt");

        let lines_py = contents_py.trim().lines().collect::<Vec<_>>();

        let lines_rs = contents_rs
            .trim()
            .lines()
            .map(|l| l.to_lowercase())
            .collect::<Vec<_>>();

        let mut py_not_contained_in_rs = vec![];
        for py_line in lines_py {
            if !lines_rs.contains(&py_line.to_string()) {
                py_not_contained_in_rs.push(py_line);
            }
        }

        assert_eq!(py_not_contained_in_rs.len(), 0, "{:?}", py_not_contained_in_rs);
    }

    #[test]
    fn test_rs_not_contained_in_py() {
        let contents_py = include_str!("../tmp_py.txt");
        let contents_rs = include_str!("../tmp_rs.txt");

        let lines_py = contents_py.trim().lines().collect::<Vec<_>>();

        let lines_rs = contents_rs
            .trim()
            .lines()
            .map(|l| l.to_lowercase())
            .collect::<Vec<_>>();

        let mut rs_not_contained_in_py = vec![];
        for rs_line in lines_rs {
            if !lines_py.contains(&rs_line.as_str()) {
                rs_not_contained_in_py.push(rs_line);
            }
        }

        assert_eq!(rs_not_contained_in_py.len(), 0, "{:?}", rs_not_contained_in_py);
    }
}
