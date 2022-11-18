// Mesh parameters 
pub const NX: usize = 5;
pub const NY: usize = 5;

// Phisics parameters
/// Heat transfer coeff for water 
pub const HTC_H20_PCB: f64 = 100.;
/// Heat transfer coeff for water 
pub const HTC_AIR_PCB: f64 = 100.;

///Thermal conductivity of PCB - [W/mK]
pub const K_PCB: f64 = 0.3; 

///Thermal conductivity of air - [W/mK]
pub const K_AIR: f64 = 2.62E-02; 
/// Air density [kg/m^2] at 150kPa
pub const RHO_AIR: f64 = 2.38; 
/// Specific heat air [J/K]
pub const CP_AIR: f64 = 1006.;

///Thermal conductivity of water - [W/mK]
pub const K_H20: f64 = 0.651; 
/// Water density [kg/m^2] at 65 C degree 
pub const RHO_H20: f64 = 983.2; 
/// Specific heat water [J/K]
pub const CP_H20: f64 = 4190.;


// Boundary condition

/// Power dissipated from membrane
pub const W: f64 = 220.;
/// Velocity of air in [m/s]
pub const U: f64 = 5.0;
/// Velocity of water in [m/s]
pub const V: f64 = 5.0;


// Geometric conditions
pub const PCB_CORE: f64 = 0.65E-03; 
