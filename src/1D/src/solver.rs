use crate::{Mesh, DiscretizationProperties};
use ndarray::{Array,Array2};
use crate::constants::{K_PCB, HTC_H20_PCB, HTC_AIR_PCB, ALPHA_PCB, T_WATER, T_AIR};

use ndarray::arr2;


pub fn matrix_a(mesh: &Mesh, discr_instructions: &DiscretizationProperties) {
    
    let nodes = mesh.idx.len();
    let dx = discr_instructions.discrete_dimension;

    let a_dims = (nodes,nodes);
    let mut a = Array2::<f64>::zeros(a_dims);

    let b_dim = nodes;
    let mut b = Array::<f64, _>::zeros(b_dim);


    // Boundary conditions water 
    a[[0,0]] = - K_PCB / dx;
    a[[0,1]] = K_PCB / dx + HTC_H20_PCB;
    b[0] = HTC_H20_PCB / T_WATER;

    // Domain development
    for i in 1..b_dim-1 {
        a[[i, i + 1]] = ALPHA_PCB / dx.powi(2); 
        a[[i, i]] = - 2. * ALPHA_PCB / dx.powi(2);
        a[[i, i - 1]] = ALPHA_PCB / dx.powi(2);
    } 

    // Condition for air side 
    a[[b_dim - 1, b_dim - 1]] = K_PCB / dx;
    a[[b_dim - 1, b_dim - 2]] = - K_PCB / dx + HTC_AIR_PCB;
    b[b_dim - 1] = HTC_AIR_PCB / T_AIR;


    println!("{}", a)
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
