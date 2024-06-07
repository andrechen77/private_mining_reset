use nalgebra::DMatrix;

/// creates a transition matrix of *transient nodes only* for the Markov
/// chain represented by the given alpha, rate-delay product, and reset policy
fn create_transition_matrix(alpha: f64, rate_delay_product: f64, reset_policy: &[i64]) -> DMatrix<f64> {
    // alpha is chance of transition in attacker's preferred direction
    // beta is the overall chance transition in honest miner's preferred direction
    // gamma is the overall chance of honest miners getting a block but it being a fork
    let chance_of_fork = 1.0 - f64::exp(-rate_delay_product);
    let gamma = (1.0 - alpha) * chance_of_fork;
    let beta = 1.0 - alpha - gamma;

    let confirmation_depth = reset_policy.len() as i64 - 1;
    let num_transient_nodes = (reset_policy.iter().sum::<i64>() + confirmation_depth * (confirmation_depth + 1) / 2) as usize;
    let mut matrix = DMatrix::<f64>::zeros(num_transient_nodes, num_transient_nodes);

	// num_nodes_before_x[x] is the number of nodes whose x value is less than x
    let num_nodes_before_x: Vec<usize> = reset_policy
        .iter()
        .enumerate()
        .scan(0, |state, (i, &threshold)| {
            let before_this_x = *state;
            let in_this_x = threshold as usize + i + 1;
            *state += in_this_x;
            Some(before_this_x)
        }).collect();
    let coords_to_node_id = |x: i64, y: i64| -> usize {
        if x == 0 {
            return 0;
        }

        let x = std::cmp::min(x, confirmation_depth); // calculate the effective x value
        let threshold_at_x = reset_policy[x as usize];
        if y > threshold_at_x {
            // wrap back around to the origin
            0
        } else if x == confirmation_depth && y <= 0 {
            // it's the win node
            num_transient_nodes
        } else {
            (num_nodes_before_x[x as usize] as isize + threshold_at_x as isize - y as isize) as usize
        }
    };

    let node_coordinates = {
        let mut result = vec![];
        for (x, &threshold) in reset_policy.iter().enumerate() {
			let x: i64 = x.try_into().unwrap();
            for y in (1..=threshold).rev().chain((0..=x).map(|y| -y)) {
                if x == confirmation_depth && y <= 0 { break; }
                result.push((x, y));
            }
        }
        result
    };
    for (x, y) in node_coordinates {
        let node_id = coords_to_node_id(x, y);

        // add an edge in attacker's preferred direction
        let dest_node_id = coords_to_node_id(x + 1, y - 1);
        if dest_node_id < num_transient_nodes { // exclude edges to the win state
            matrix[(node_id, dest_node_id)] += alpha;
        }
        // add an edge in honest miner's preferred direction
        let dest_node_id = coords_to_node_id(x, y + 1);
        matrix[(node_id, dest_node_id)] += beta;
        // add an edge to self for chance of natural fork
        matrix[(node_id, node_id)] += gamma;
    }
    matrix
}

pub fn calculate_expected_steps(alpha: f64, rate_delay_product: f64, reset_policy: &[i64]) -> f64 {
    let q_matrix = create_transition_matrix(alpha, rate_delay_product, reset_policy);
    let identity = DMatrix::<f64>::identity(q_matrix.nrows(), q_matrix.ncols());
    let inverse = (identity - q_matrix).try_inverse().expect("matrix should be invertible");
    let sums = inverse.column_sum();
    sums[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_gamma() {
        let matrix = create_transition_matrix(0.25, 1.0, &[0, 1, 2, 3]);
        println!("{:.2}", matrix);
    }
}
