This is the code for analyzing and simulating the private mining with reset
scenario. See the paper for details on reducing the private mining scenario
to a form that can be analyzed by code.

# Files

## `src/reset_policy.rs`

A reset policy, as defined by the paper, is characterized by a threshold
function $r : \mathbb{N} \rightarrow \mathbb{N}$ which maps a value of $h_A$ to
the corresponding value of $h'$ above which the adversary should reset.
As described in the paper, $r$ is constant once you pass $h_A \geq k$.
As such, only the first $k + 1$ values of the function are required to
completely describe a reset policy.

Therefore, throughout the code, the `[i64]` type is used to describe a reset
policy, where `policy[i]` takes the value $r(i)$.
This means that the first value of the `[i64]` array is always `0`.

The file `src/reset_policy.rs` provides utilities for creating reset policies.
The function `linear_reset_policy` creates a reset policy for the given
confirmation depth where $r$ takes on `max_tolerable_height_difference` for $h_A
\geq k$, and linearly interpolates the points for $0 < h_A < k$ `PolicyIterator`
is an iterator that, for a given confirmation depth enumerates every single
valid reset policy with thresholds below some maximum value; i.e. every
non-decreasing sequence of size `size`, starting at 0 and not exceeding
`max_value`. The `size` argument taken by `PolicyIterator::new` is one more than
the confirmation depth (to account for the required $0$ at the beginning).

## `src/reset_policy.rs`

This file follows the process described in the paper to create a transition
matrix out of the private mining with reset scenario, and then solves for the
expected value to reach the absorbing win state.

## `src/main.rs`

This file defines a parameter space over $\alpha$, $k$, $\lambda_B\Delta$, and
all reset policies below a certain threshold, and then calculates the expected
number of steps for each set of parameters. The results are printed to standard
output.

## `src/simulation.rs`

This file contains functions for calculating the expected number of steps
using simulations. `simulate_until_win` simulates one trial given $\alpha$ and the reset policy, while `calculate_expected_steps` simulates the given number of trials for the same given parameters and returns the average.

These functions are not used in the actual code; they only exist for testing
purposes to verify the analytic solutions. These functions also do not account
for possible propagation delay (because when I got to that point I was already
confident in the analytic solutions' correctness).

## `plot_results.py`

Given a file named `results.txt` with the standard output of the Rust program,
this file creates the plots seen in the paper. It was run in Google Colab, and
might not work as a headless script.
