use ndarray::*;
use simplex_method::Table;
use branch_bound_method::bnb_optimise;

fn main() {
    env_logger::init();
    let constr_coeff =
        array![[4f64, 4f64, 1f64],
         [1f64, 2f64, 0f64],
         [0f64, 0.5f64, 4f64]];
    let func_coeff =
         array![6f64, 6f64, 6f64];
    let constr_val =
         array![5f64, 3f64, 8f64];
    let table = Table::new(constr_coeff, constr_val, func_coeff, false);
    let result = bnb_optimise(table).expect("Horrid smell");
    println!("{}", result);
}
