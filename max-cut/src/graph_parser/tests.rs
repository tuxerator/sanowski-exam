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

    let mut graph_expected = Graph::new(10);
    graph_expected.add_edges(&[
        Edge::new(5, 6),
        Edge::new(5, 7),
        Edge::new(4, 5),
        Edge::new(4, 6),
        Edge::new(5, 9),
        Edge::new(7, 9),
        Edge::new(7, 8),
        Edge::new(8, 9),
        Edge::new(1, 2),
        Edge::new(6, 7),
        Edge::new(4, 7)
    ]);

    if super::parse_graph(test_str)? == graph_expected {
        Ok(())
    } 
    else {
        Err("Parsed graph did not match expected graph!".to_owned())
    }
}
