#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]



pub mod helpers;

pub mod src {
pub mod autocode {
pub mod breit1;
pub mod deriv3;
pub mod deriv4;
pub mod dkb;
pub mod gaunt1;
pub mod grad1;
pub mod grad2;
pub mod hess;
pub mod int1e_grids1;
pub mod int3c1e;
pub mod int3c2e;
pub mod intor1;
pub mod intor2;
pub mod intor3;
pub mod intor4;
pub mod lresc;
} // mod autocode
pub mod breit;
pub mod c2f;
pub mod cart2sph;
pub mod cint1e;
pub mod cint1e_a;
pub mod cint1e_grids;
pub mod cint2c2e;
pub mod cint2e;
pub mod cint3c1e;
pub mod cint3c1e_a;
pub mod cint3c2e;
pub mod cint_bas;
pub mod eigh;
pub mod fblas;
pub mod find_roots;
pub mod fmt;
pub mod g1e;
pub mod g1e_grids;
pub mod g2c2e;
pub mod g2e;
pub mod g3c1e;
pub mod g3c2e;
pub mod optimizer;
pub mod rys_roots;
pub mod rys_wheeler;
} // mod src
