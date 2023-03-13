use crate::graph::Graph;

#[derive(PartialEq, Debug)]
enum ParsedLine<'a> {
    Graph(usize, usize),
    Edge(usize, usize),
    Comment(&'a str),
}

pub fn parse_pace_graph(raw: &str) -> Result<Graph, String> {
    let lines = raw.lines();

    let mut lines = lines.filter_map(|line| match parse_line(line) {
        line @ Ok(ParsedLine::Graph(..)) => Some(line),
        line @ Ok(ParsedLine::Edge(..)) => Some(line),
        Ok(_) => None,
        err => Some(err),
    });

    lines.clone().try_for_each(|x| match x {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    })?;

    let graph_desc = lines
        .find_map(|line| match line {
            Ok(ParsedLine::Graph(n, m)) => Some((n.to_owned(), m.to_owned())),
            _ => None,
        })
        .ok_or_else(|| "no p-line".to_owned())?;

    let mut graph = Graph::new_empty(graph_desc.0);

    for line in lines {
        match line {
            Ok(ParsedLine::Edge(x, y)) => graph.add_edge(&(x, y)),
            _ => continue,
        };
    }

    Ok(graph)
}

fn parse_line(line: &str) -> Result<ParsedLine, String> {
    // Split line into it's elements
    let elems: Vec<&str> = line.split(' ').collect();

    match elems[..] {
        ["p", _, n, m] => Ok(ParsedLine::Graph(n.parse().unwrap(), m.parse().unwrap())),
        ["c", comment] => Ok(ParsedLine::Comment(comment)),
        [x, y] => {
            let edge = (x.parse::<usize>(), y.parse::<usize>());
            match edge {
                (Ok(x), Ok(y)) => Ok(ParsedLine::Edge(x - 1, y - 1)),
                _ => Err(format!("unrecognized line: \'{}\'", elems.join(" "))),
            }
        }
        _ => Err(format!("unrecognized line: \'{}\'", elems.join(" "))),
    }
}

pub fn parse_rudy(raw: &str) -> Result<Graph, String> {
    let mut lines = raw.lines();

    let first_line = match lines.next() {
        Some(line) => line,
        None => return Err("emtry file".to_string()),
    };

    let mut graph = Graph::new_empty(
        first_line
            .split(' ')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    );

    lines.try_for_each(|line| {
        let tokens: Vec<&str> = line.split(' ').collect();

        match tokens[..] {
            [v, u, _] => {
                graph.add_edge(&(v.parse::<usize>().unwrap() - 1, u.parse::<usize>().unwrap() - 1));
                Ok(())
            }
            _ => Err(format!("unrecognized line: \'{}\'", tokens.join(" "))),
        }
    })?;
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_graph() {
        let test_line = "p test 4 6";
        let graph_expected = ParsedLine::Graph(4, 6);

        assert_eq!(parse_line(test_line).unwrap(), graph_expected);
    }

    #[test]
    fn parse_line_edge() {
        let test_line = "4 30";
        let edge_expected = ParsedLine::Edge(4 - 1, 30 - 1);

        assert_eq!(parse_line(test_line).unwrap(), edge_expected);
    }

    #[test]
    fn parse_line_err() {
        let test_line = "This line is not recognizable";
        assert!(parse_line(test_line).is_err());
    }

    #[test]
    fn parse_graph() -> Result<(), String> {
        let test_str = "p cep 10 11\n\
                        6 7\n\
                        6 8\n\
                        5 6\n\
                        5 7\n\
                        6 10\n\
                        8 10\n\
                        8 9\n\
                        9 10\n\
                        2 3\n\
                        7 8\n\
                        5 8";

        let mut graph_expected = Graph::new_empty(10);
        graph_expected.add_edges(&[
            (5usize, 6usize),
            (5usize, 7usize),
            (4usize, 5usize),
            (4usize, 6usize),
            (5usize, 9usize),
            (7usize, 9usize),
            (7usize, 8usize),
            (8usize, 9usize),
            (1usize, 2usize),
            (6usize, 7usize),
            (4usize, 7usize),
        ]);

        if super::parse_pace_graph(test_str)? == graph_expected {
            Ok(())
        } else {
            Err("Parsed graph did not match expected graph!".to_owned())
        }
    }

    #[test]
    fn parse_rudy() {
        let test_str = "10 11\n\
                        6 7 1\n\
                        6 8 1\n\
                        5 6 1\n\
                        5 7 1\n\
                        6 10 1\n\
                        8 10 1\n\
                        8 9 1\n\
                        9 10 1\n\
                        2 3 1\n\
                        7 8 1\n\
                        5 8 1";

        let mut graph_expected = Graph::new_empty(10);
        graph_expected.add_edges(&[
            (5usize, 6usize),
            (5usize, 7usize),
            (4usize, 5usize),
            (4usize, 6usize),
            (5usize, 9usize),
            (7usize, 9usize),
            (7usize, 8usize),
            (8usize, 9usize),
            (1usize, 2usize),
            (6usize, 7usize),
            (4usize, 7usize),
        ]);

        assert_eq!(super::parse_rudy(test_str).unwrap(), graph_expected);
    }
}
