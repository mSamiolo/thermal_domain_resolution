use std::fmt::{Debug, Formatter, Result};

pub struct DiscretizationProperties {
    pub nx: usize,
    pub x_dim: f64,
    pub discrete_dimension: f64,
}

pub struct Mesh {
    pub x: Vec<f64>,
    pub idx: Vec<usize>,
}

impl Mesh {
    /// Set the physical location and index of the mesh
    pub fn mesh_gen(discr_instructions: &DiscretizationProperties) -> Self {
        // let temp_air_in = 65.;
        let dx = discr_instructions.x_dim / (discr_instructions.nx as f64 - 1.);

        // Define the spatial field
        let mut x: Vec<f64> = Vec::new();
        let mut idx: Vec<usize> = Vec::new();

        // Define value of the space
        for x_idx in 0..discr_instructions.nx {
            let index = x_idx * discr_instructions.nx;
            idx.push(index);
            x.push(x_idx as f64 * dx);
        }
        Self { x, idx }
    }
}

impl DiscretizationProperties {
    /// Parsing mesh to get values to feed on equations
    pub fn get_discretization_intructions(nx: usize, x_dim: f64) -> DiscretizationProperties {
        let nodes = nx;
        let delta_x_i = x_dim / nodes as f64;

        DiscretizationProperties {
            nx,
            x_dim,
            discrete_dimension: delta_x_i,
        }
    }
}

// Implementation of the debug trait to display the struct correctly and in a mainingfull way
impl Debug for Mesh {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "The mesh contains {} nodes, with coord:\n
index[0] = {}  ...  index[{}] = {} \n
    x[0] = {}m ...   x[{}] = {}m  \n ",
            self.x.len(),
            self.idx[0],
            self.idx.len() - 1,
            self.idx.last().unwrap(),
            self.x[0],
            self.x.len() - 1,
            self.x.last().unwrap(),
        )
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn mesh_dimension_check() {
//         // v3 series dimension
//         use crate::constants::{NX, NY};

//         let x_dim = 2.04E-01;
//         let y_dim = 1.44E-01;
//         let prop: DiscretizationProperties = Mesh::get_discretization_intruction_2D(NX, NY, x_dim, y_dim);
//         let domain = Mesh::mesh_gen(&prop);
//         assert_eq!(domain.idx[0], 0);
//         assert_eq!(domain.x[NX], 5.1E-2);
//         assert_eq!(domain.y[1], 3.6E-2);
//     }
// }
