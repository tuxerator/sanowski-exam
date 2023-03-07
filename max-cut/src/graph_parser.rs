use crate::graph::edge::Edge;
use crate::graph::Graph;

enum ParsedLine<'a> {
    Graph(usize, usize),
    Edge(usize, usize),
    Comment(&'a str),
}

fn parse_graph(raw: &str) -> Result<Graph, &'static str> {
    let lines = raw.lines();

    let mut lines = lines.filter_map(|line| {
        let line = parse_line(&line).unwrap();
        match line {
            line @ ParsedLine::Graph(..) => Some(line),
            line @ ParsedLine::Edge(..) => Some(line),
            _ => None,
        }
    });

    let graph_desc = lines
        .find_map(|line| match line {
            ParsedLine::Graph(n, m) => Some((n.to_owned(), m.to_owned())),
            _ => None,
        })
        .expect("Could not find p-line.");

    let mut graph = Graph::new(graph_desc.0);

    for line in lines {
        match line {
            ParsedLine::Edge(x, y) => graph.add_edge(&Edge::new(x, y)),
            _ => continue,
        };
    }

    Ok(graph)
}

fn parse_line(line: &str) -> Result<ParsedLine, &str> {
    // Split line into it's elements
    let elems: Vec<&str> = line.split(' ').collect();

    match elems[..] {
        ["p", _, n, m] => Ok(ParsedLine::Graph(n.parse().unwrap(), m.parse().unwrap())),
        ["c", comment] => Ok(ParsedLine::Comment(comment)),
        [x, y] => Ok(ParsedLine::Edge(x.parse().unwrap(), y.parse().unwrap())),
        _ => Err("Unrecognized line"),
    }
}
