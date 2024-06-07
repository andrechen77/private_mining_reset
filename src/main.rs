use itertools::iproduct;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

mod simulation;
mod analysis;
mod reset_policy;

fn explore_cartesian_parameter_space() {
    // define the parameter space
    // let alphas = std::iter::once(0.3);
    let alphas = (1..=9).map(|x| x as f64 / 10 as f64);
    let rate_delay_products = (0..=10).map(|x| x as f64 / 10 as f64);


    let max_tolerable_height_diff = 5;
    let conf_depths = std::iter::once(8);
    let reset_policies = conf_depths
        .map(|k| reset_policy::PolicyIterator::new(k + 1, max_tolerable_height_diff))
        .flatten();
    // let reset_policies = std::iter::once(vec![0, 2]);
    let parameters: Vec<_> = iproduct!(alphas, rate_delay_products, reset_policies).collect();

    // run once over every parameter
    parameters.par_iter().for_each(|(alpha, rate_delay_product, reset_policy)| {
        let analysis_result = analysis::calculate_expected_steps(*alpha, *rate_delay_product, reset_policy);

        println!("{:.4}, {:.4}, {:?}, {:.5}", alpha, rate_delay_product, reset_policy, analysis_result);
    });
}

fn main() {
    explore_cartesian_parameter_space();
}
