use std::fmt::Debug;
use std::fmt::{Formatter, Result};
use crate::{NX, NY /*HTC, FuelCell, CP_H2O */};

pub struct Mesh {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub idx: Vec<usize>,
}

pub enum MeshingError {
    NotEnoughtXNodes,
    NotEnoughtYNodes,
}

// Implementationof the debug trait to display the struct correctly and in a mainingfull way
impl Debug for Mesh {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "The mesh contains {} nodes, with coord:\n
index[0] = {}  ...  index[{}] = {} \n
    x[0] = {}m ...   x[{}] = {}m  \n
    y[0] = {}m ...   y[{}] = {}m  \n ",
            self.x.len(),
            self.idx[0],
            self.idx.len() - 1,
            self.idx.last().unwrap(),
            self.x[0],
            self.x.len() - 1,
            self.x.last().unwrap(),
            self.y[0],
            self.y.len() - 1,
            self.y.last().unwrap()
        )
    }
}

impl Mesh {
    pub fn mesh_gen(x_dim: f64, y_dim: f64) -> Self {
        // let temp_air_in = 65.;
        let nx: usize = NX;
        let ny: usize = NY;
        let dx = x_dim / (nx as f64 - 1.);
        let dy = y_dim / (nx as f64 - 1.);

        // Define the spatial field
        let mut x: Vec<f64> = Vec::new();
        let mut y: Vec<f64> = Vec::new();
        let mut idx: Vec<usize> = Vec::new();

        // Define value of the space
        for x_idx in 0..nx {
            for y_idx in 0..ny {
                let index = y_idx + x_idx * nx;
                idx.push(index);
                x.push(x_idx as f64 * dx);
                y.push(y_idx as f64 * dy);
            }
        }

        Self { x, y, idx }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mesh_dimension_check() {
        // v3 series dimension
        let x_dim = 2.04E-01;
        let y_dim = 1.44E-01;
        let domain = Mesh::mesh_gen(x_dim, y_dim);
        assert_eq!(domain.idx[0], 0);
        assert_eq!(domain.x[1], 4.08E-2);
        assert_eq!(domain.y[2], 5.76E-2);
    }
}