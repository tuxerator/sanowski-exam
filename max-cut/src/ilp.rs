use good_lp::{
    constraint, default_solver, variable, Expression, ProblemVariables,
    SolverModel, Variable, Solution,
};

use crate::graph::{Graph, Edge};

pub struct MaxCutIlp<'a> {
    graph: &'a Graph,
    edges: Vec<Edge>,
}

impl<'a> MaxCutIlp<'a> {
    pub fn new(graph: &Graph) -> MaxCutIlp {
        MaxCutIlp {
            graph,
            edges: graph.all_edges(),
        }
    }

    pub fn solve(&self) -> Result<Vec<Edge>, good_lp::ResolutionError> {
        let mut problem = ProblemVariables::new();
        let vertex_variables: Vec<Variable> = problem.add_vector(variable().binary(), self.graph.size());
        let edge_variables: Vec<Variable> = problem.add_vector(variable().binary(), self.edges.len());
        let objective: Expression = edge_variables.iter().sum();

        let mut model = problem.maximise(objective).using(default_solver);

        for (var, edge) in edge_variables.iter().zip(self.edges.iter()) {
            model = model.with(constraint!(*var <= vertex_variables[edge.0] + vertex_variables[edge.1]));
            model = model.with(constraint!(*var <= 2 - (vertex_variables[edge.0] + vertex_variables[edge.1])));
        }

        let solution = model.solve()?;

        let mut max_cut: Vec<Edge> = Vec::new();
        for (var, edge) in edge_variables.iter().zip(self.edges.iter()) {
            if solution.value(*var) == 1. {
                max_cut.push(*edge);
            }
        }

        Ok(max_cut)
    }
}
