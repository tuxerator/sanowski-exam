use crate::graph::edge::Edge;
use crate::graph::Graph;

#[cfg(test)]
mod tests;


#[derive(PartialEq, Debug)]
enum ParsedLine<'a> {
    Graph(usize, usize),
    Edge(usize, usize),
    Comment(&'a str),
}



pub fn parse_graph(raw: &str) -> Result<Graph, String> {
    let lines = raw.lines();

    let mut lines = lines.filter_map(|line| {
        match parse_line(&line) {
            line @ Ok(ParsedLine::Graph(..)) => Some(line),
            line @ Ok(ParsedLine::Edge(..)) => Some(line),
            Ok(_) => None,
            err => Some(err),
        }
    });

    lines.clone().try_for_each(|x| match x {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    })?;

    let graph_desc = lines
        .find_map(|line| match line {
            Ok(ParsedLine::Graph(n, m)) => Some((n.to_owned(), m.to_owned())),
            _ => None,
        }).ok_or_else(|| "no p-line".to_owned())?;

    let mut graph = Graph::new(graph_desc.0);

    for line in lines {
        match line {
            Ok(ParsedLine::Edge(x, y)) => graph.add_edge(&Edge::new(x, y)),
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
            let (x, y) = (x.parse::<usize>(), y.parse::<usize>());
            if x.is_ok() && y.is_ok() {
                Ok(ParsedLine::Edge(x.unwrap() - 1, y.unwrap() - 1))
            }
            else {
                Err(format!("unrecognized line: \'{}\'", elems.join(" ").to_owned()))
            }
        },
        _ => Err(format!("unrecognized line: \'{}\'",elems.join(" ")).to_owned()),
    }
}
