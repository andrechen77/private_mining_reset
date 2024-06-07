use rand::distributions::{Bernoulli, Distribution};
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};
use std::cmp::min;

// Returns the number of steps required to get into the win zone
fn simulate_until_win(alpha: f64, reset_policy: &[i64]) -> i64 {
    let confirmation_depth = reset_policy.len() as i64 - 1;
    let mut rng = rand::thread_rng();
    let bernoulli = Bernoulli::new(alpha).expect("alpha is between 0 and 1");
    let samples = bernoulli.sample_iter(&mut rng);

    // attacker chain height
    let mut height_a = 0;
    // honest chain height
    let mut height_b = 0;

    // simulate
    let mut num_steps: i64 = 0;
    for sample in samples {
		// we are allowed to add this before checking for win because we know
		// that the attacker will never start in a winning state
		num_steps += 1;
        if sample {
            // the attacker wins the blcok
            height_a += 1;
            // println!("A mined: {} {}", height_a, height_b);
            if height_b <= height_a && height_a >= confirmation_depth {
                // the attacker has won
                // println!("A wins in {} steps", i);
                break;
            }
        } else {
            // the honests win the block
            height_b += 1;
            // println!("B mined: {} {}", height_a, height_b);
            if height_b - height_a > reset_policy[min(height_a, confirmation_depth) as usize] {
                // the attacker wants to reset
                height_a = 0;
                height_b = 0;
                // println!("A reset: {} {}", height_a, height_b);
            }
        }
    }
    num_steps
}

pub fn calculate_expected_steps(num_trials: usize, alpha: f64, reset_policy: &[i64]) -> f64 {
    let trial_iter = (0..num_trials).into_par_iter().map(|i| {
        let result = simulate_until_win(alpha, &reset_policy);
        result
    });
    let expected_value = trial_iter.map(|num_steps| num_steps as f64).sum::<f64>() / num_trials as f64;
    expected_value
}
