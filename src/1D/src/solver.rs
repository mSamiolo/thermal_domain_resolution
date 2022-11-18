use crate::{Mesh, DiscretizationProperties};
use ndarray::{Array,Array2};
use crate::constants::*;

use ndarray::arr2;


pub fn matrix_a(mesh: &Mesh, discr_instructions: &DiscretizationProperties) {
    
    let nodes = mesh.idx.len();
    let a_dims = (nodes,nodes);
    let b_dim = nodes;
    let dx = discr_instructions.discrete_dimension;

    let mut a = Array2::<f64>::zeros(a_dims);

    // Boundaty conditions water 
    a[[1,1]] = - K_PCB / dx;
    a[[1,2]] = ( K_PCB / dx + HTC_H20_PCB );
    // Boundary condition for water side
    

    // a[[nodes, nodes]] = 7.;
    println!("{}",a)


    

    // Array::dim(&self)

}



pub fn thermal_solver(mesh: &Mesh) {
    
        let a = arr2(
                &[[1, 2, 3],
                    [4, 5, 6]]);

        let b = arr2(
                &[[6],
                    [5],
                    [4]]);

        // println!("{}", a.dot(&b));
    }
