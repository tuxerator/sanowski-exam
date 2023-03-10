use std::fmt;

#[derive(PartialEq, Debug)]
pub struct QuadMatrixBool {
    data: Vec<Vec<bool>>,
}

#[derive(Debug)]
pub struct NotQuadError;

impl std::error::Error for NotQuadError {}

impl fmt::Display for NotQuadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Initial matrix is not quadratic!")
    }
}

impl QuadMatrixBool {
    pub fn new_epmty(n: usize) -> QuadMatrixBool {
        QuadMatrixBool { data: vec![vec![false; n]; n] }
    }

    pub fn new(data: Vec<Vec<bool>>) -> Result<QuadMatrixBool, NotQuadError> {
        if !data.iter().all(|x| x.len() == data.len()) {
            return Err(NotQuadError);
        }

        Ok(QuadMatrixBool { data })
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, i: &(usize, usize)) -> bool {
        self.data[i.0][i.1]
    }

    pub fn set(&mut self, i: &(usize,usize), element: bool) {
        self.data[i.0][i.1] = element;
    }
}
