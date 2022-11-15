use std::fmt::Debug;
use std::fmt::{Formatter, Result};

pub struct Mesh {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub idx: Vec<usize>,
}

pub struct DiscretizationProperties {
    nx: usize,
    ny: usize,
    pub x_dim: f64,
    pub y_dim: f64,
    pub area_i: f64
}

pub enum MeshingError {
    NotEnoughtXNodes,
    NotEnoughtYNodes,
}

impl Mesh {
    /// Set the physical location and index of the mesh
    pub fn mesh_gen(prop: DiscretizationProperties) -> Self {

        // let temp_air_in = 65.;

        let dx = prop.x_dim / (prop.nx as f64 - 1.);
        let dy = prop.y_dim / (prop.nx as f64 - 1.);

        // Define the spatial field
        let mut x: Vec<f64> = Vec::new();
        let mut y: Vec<f64> = Vec::new();
        let mut idx: Vec<usize> = Vec::new();

        // Define value of the space
        for x_idx in 0..prop.nx {
            for y_idx in 0..prop.ny {
                let index = x_idx * prop.nx + y_idx;
                idx.push(index);
                x.push(x_idx as f64 * dx);
                y.push(y_idx as f64 * dy);
            }
        }

        Self {x, y, idx }
    }
    
    /// Parsing mesh to get values to feed on equations
    pub fn get_discretization_intruction(nx : usize, ny: usize, x_dim: f64, y_dim: f64) -> DiscretizationProperties {
        let nodes = nx * ny;
        let area = x_dim * y_dim;
        let area_i = area / nodes as f64;
        
        DiscretizationProperties {
            nx,
            ny,
            x_dim,
            y_dim,
            area_i 
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mesh_dimension_check() {
        // v3 series dimension
        use crate::constants::{NX, NY};

        let x_dim = 2.04E-01;
        let y_dim = 1.44E-01;
        let prop: DiscretizationProperties = Mesh::get_discretization_intruction(NX, NY, x_dim, y_dim);
        let domain = Mesh::mesh_gen(prop);
        assert_eq!(domain.idx[0], 0);
        assert_eq!(domain.x[NX], 5.1E-2);
        assert_eq!(domain.y[1], 3.6E-2);
    }
}