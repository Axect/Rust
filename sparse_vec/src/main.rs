fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
pub struct SPCol {
    pub row: usize,
    pub nnz: usize,
    pub row_ics: Vec<usize>,
    pub val: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct SPRow {
    pub col: usize,
    pub nnz: usize,
    pub col_ics: Vec<usize>,
    pub val: Vec<f64>,
}

pub trait SPVec: Default {
    fn len(&self) -> usize;
    fn nnz(&self) -> usize;
    fn ics(&self) -> &Vec<usize>;
    fn val(&self) -> &Vec<f64>;
    fn ics_mut(&mut self) -> &mut Vec<usize>;
    fn val_mut(&mut self) -> &mut Vec<f64>;
    fn resize_mut(&mut self, m: usize, nnz: usize);
}

impl SPCol {
    pub fn new(row: usize, nnz: usize) -> Self {
        SPCol {
            row,
            nnz,
            row_ics: vec![0usize; nnz],
            val: vec![0f64; nnz],
        }
    }
}

impl SPVec for SPCol {
    fn len(&self) -> usize {
        self.row
    }

    fn nnz(&self) -> usize {
        self.nnz
    }

    fn ics(&self) -> &Vec<usize> {
        &self.row_ics 
    }

    fn val(&self) -> &Vec<f64> {
        &self.val
    }

    fn ics_mut(&mut self) -> &mut Vec<usize> {
        &mut self.row_ics
    }

    fn val_mut(&mut self) -> &mut Vec<f64> {
        &mut self.val
    }

    fn resize_mut(&mut self, m: usize, nnz: usize) {
        self.row = m;
        self.nnz = nnz;
        self.row_ics = vec![0usize; nnz];
        self.val = vec![0f64; nnz];
    }
}

impl Default for SPCol {
    fn default() -> Self {
        SPCol {
            row: 0usize,
            nnz: 0usize,
            row_ics: vec![],
            val: vec![],
        }
    }
}

impl SPVec for SPRow {
    fn len(&self) -> usize {
        self.col
    }

    fn nnz(&self) -> usize {
        self.nnz
    }

    fn ics(&self) -> &Vec<usize> {
        &self.col_ics 
    }

    fn val(&self) -> &Vec<f64> {
        &self.val
    }

    fn ics_mut(&mut self) -> &mut Vec<usize> {
        &mut self.col_ics
    }

    fn val_mut(&mut self) -> &mut Vec<f64> {
        &mut self.val
    }

    fn resize_mut(&mut self, m: usize, nnz: usize) {
        self.col = m;
        self.nnz = nnz;
        self.col_ics = vec![0usize; nnz];
        self.val = vec![0f64; nnz];
    }
}

impl Default for SPRow {
    fn default() -> Self {
        SPRow {
            col: 0usize,
            nnz: 0usize,
            col_ics: vec![],
            val: vec![],
        }
    }
}
