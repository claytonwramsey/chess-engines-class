/*
  Tomato, a UCI-compatible chess engine.
  Copyright (C) 2022 Clayton Ramsey.

  Tomato is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  Tomato is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

//! The tuner for the Tomato chess engine.
//! This file exists to create a binary which can be used to generate weights from an annotated EPD
//! file.
//!
//! The tuner operates by using gradient descent on logistic regression to classify the results of a
//! given position.

#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    thread::scope,
    time::Instant,
};

use tomato::base::{Board, Color, Piece, Square};
use tomato::engine::evaluate::{material, phase_of, pst::PST};

/// Construct the gradient vector for a subset of the input data.
///
/// # Inputs
///
/// - `observations`: array of sample features paired with their expected evaluations.
///   For each observation, the first element is a sparse feature vector containing `(index, value)`
///   pairs for each nonzero element of the feature vector, and the second element is the expected
///   evaluation of the feature vector.
///   If Black wins, the evaluation is 0.0; if White wins, the evaluation is 1.0, and if it's a
///   draw, the evaluation is 0.5.
/// - `weights`: The weight vector.
///
/// # Returns
///
/// Returns a pair containing the gradient vector of error with respect to weights and the
/// sum-squared error across this epoch.
fn compute_gradient(observations: &[(Vec<(usize, f32)>, f32)], weights: &[f32]) -> (Vec<f32>, f32) {
    let mut grad = vec![0.; weights.len()];
    let mut sum_se = 0.;
    for (features, sigm_expected) in observations {
        let sigm_eval = sigmoid(features.iter().map(|&(idx, val)| val * weights[idx]).sum());
        let err = sigm_expected - sigm_eval;
        let coeff = -sigm_eval * (1. - sigm_eval) * err;
        // construct the gradient
        for &(idx, feat_val) in features {
            grad[idx] += feat_val * coeff;
        }
        sum_se += err * err;
    }

    (grad, sum_se)
}

#[allow(clippy::similar_names)]
/// Run the main training function.
///
/// The first command line argument must the the path of the file containing training data.
///
/// # Panics
///
/// This function will panic if the EPD training data is not specified or does not exist.
pub fn main() {
    let args: Vec<String> = env::args().collect();
    // first argument is the name of the binary
    let path_str = &args[1..].join(" ");
    let mut weights = load_weights();
    // fuzz(&mut weights, 0.05);
    let learn_rate = 10.;

    let nthreads = 14;
    let tic = Instant::now();

    // construct the datasets.
    // Outer vector: each element for one datum
    // Inner vector: each element for one feature-quantity pair
    let input_sets = extract_epd(path_str).unwrap();

    let toc = Instant::now();
    println!("extracted data in {} secs", (toc - tic).as_secs());
    for i in 0..10_000 {
        weights = train_step(&input_sets, &weights, learn_rate, nthreads).0;
        println!("iteration {i}...");
    }

    print_weights(&weights);
}

#[allow(clippy::type_complexity)]
/// Expand an EPD file into a set of features that can be used for training.
fn extract_epd(
    location: &str,
) -> Result<Vec<(Vec<(usize, f32)>, f32)>, Box<dyn std::error::Error>> {
    let file = File::open(location)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let mut split_line = line.split('"');
        // first part of the split is the FEN, second is the score, last is just a semicolon
        let fen = split_line.next().ok_or("no FEN given")?;
        let b = Board::from_fen(fen)?;
        let features = extract(&b);
        let score_str = split_line.next().ok_or("no result given")?;
        let score = match score_str {
            "1/2-1/2" => 0.5,
            "0-1" => 0.,
            "1-0" => 1.,
            _ => Err("unknown score string")?,
        };
        data.push((features, score));
    }

    Ok(data)
}

#[allow(clippy::similar_names, clippy::cast_precision_loss)]
/// Perform one step of PST training, and update the weights to reflect this.
/// Returns the weight vector MSE of the current epoch.
///
/// Inputs:
/// * `inputs`: a vector containing the input vector and the expected evaluation.
/// * `weights`: the weight vector to train on.
/// * `learn_rate`: a coefficient on the speed at which the engine learns.
///
/// Each element of `inputs` must be the same length as `weights`.
fn train_step(
    inputs: &[(Vec<(usize, f32)>, f32)],
    weights: &[f32],
    learn_rate: f32,
    nthreads: usize,
) -> (Vec<f32>, f32) {
    let tic = Instant::now();
    let chunk_size = inputs.len() / nthreads;
    let mut new_weights: Vec<f32> = weights.to_vec();
    let mut sum_se = 0.;
    scope(|s| {
        let mut grads = Vec::new();
        for thread_id in 0..nthreads {
            // start the parallel work
            let start = chunk_size * thread_id;
            grads.push(s.spawn(move || compute_gradient(&inputs[start..][..chunk_size], weights)));
        }
        for grad_handle in grads {
            let (sub_grad, se) = grad_handle.join().unwrap();
            sum_se += se;
            for i in 0..new_weights.len() {
                new_weights[i] -= learn_rate * sub_grad[i] / inputs.len() as f32;
            }
        }
    });
    let toc = Instant::now();
    let mse = sum_se / inputs.len() as f32;
    println!(
        "{} samples in {:?}: {:.0} samples/sec; mse {}",
        inputs.len(),
        (toc - tic),
        inputs.len() as f32 / (toc - tic).as_secs_f32(),
        mse
    );

    (new_weights, mse)
}

#[inline(always)]
/// Compute the  sigmoid function of a variable.
/// `beta` is the horizontal scaling of the sigmoid.
///
/// The sigmoid function here is given by the LaTeX expression
/// `f(x) = \frac{1}{1 - \exp (- \beta x)}`.
fn sigmoid(x: f32) -> f32 {
    1. / (1. + (-x).exp())
}

/// Load the weight value constants from the ones defined in the PST evaluation.
fn load_weights() -> Vec<f32> {
    let mut weights = Vec::new();
    for pt in Piece::NON_KING {
        let val = material::value(pt);
        weights.push(val.mg.float_val());
        weights.push(val.eg.float_val());
    }

    for pt in Piece::ALL {
        for rank in 0..8 {
            for file in 0..8 {
                let sq_idx = 8 * rank + file;
                let score = PST[pt as usize][sq_idx];
                weights.push(score.mg.float_val());
                weights.push(score.eg.float_val());
            }
        }
    }

    weights
}

#[allow(clippy::cast_possible_truncation, clippy::similar_names)]
/// Print out a weights vector so it can be used as code.
fn print_weights(weights: &[f32]) {
    let material_val = |name: &str, start: usize| {
        println!(
            "{name} => Score::centipawns({}, {})",
            (weights[start] * 100.) as i16,
            (weights[start + 1] * 100.) as i16,
        );
    };

    // print material values
    material_val("Piece::Pawn", 0);
    material_val("Piece::Bishop", 2);
    material_val("Piece::Rook", 4);
    material_val("Piece::Queen", 6);
    material_val("Piece::Pawn", 8);
    println!("-----");

    let offset = 10;

    // print PST
    println!("pub const PST: Pst = unsafe {{ transmute([");
    for pt in Piece::ALL {
        println!("    [ // {pt}");
        let pt_idx = offset + (128 * pt as usize);
        for rank in 0..8 {
            print!("        ");
            for file in 0..8 {
                let sq = Square::new(rank, file).unwrap();
                let mg_idx = pt_idx + (2 * sq as usize);
                let eg_idx = mg_idx + 1;
                let mg_val = (weights[mg_idx] * 100.) as i16;
                let eg_val = (weights[eg_idx] * 100.) as i16;
                print!("({mg_val}, {eg_val}), ");
            }
            println!();
        }
        println!("    ],");
    }
    println!("]) }};");
    println!("-----");
}
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::similar_names
)]
/// Extract a feature vector from a board.
/// The resulting vector will have dimension 1118.
/// The PST values can be up to 1 for a white piece on the given PST square, -1 for a black piece,
/// or 0 for both or neither.
/// The PST values are then pre-blended by game phase.
///
/// The elements of the vector are listed by their indices as follows:
///
/// * 0..2: Knight quantity
/// * 2..4: Bishop quantity
/// * 4..6: Rook quantity
/// * 6..8: Queen quantity
/// * 8..10: Pawn quantity
/// * 10..138: Knight PST, paired (midgame, endgame) element-wise
/// * 138..266: Bishop PST
/// * 266..394: Rook PST
/// * 394..522: Queen PST
/// * 522..650: Pawn PST.
///     Note that the indices for the first and eight ranks do not matter.
/// * 650..778: King PST
///
/// Ranges given above are lower-bound inclusive.
/// The representation is sparse, so each usize corresponds to an index in the true vector.
/// Zero entries will not be in the output.
fn extract(b: &Board) -> Vec<(usize, f32)> {
    let mut features = Vec::with_capacity(28);
    let phase = phase_of(b);
    // Indices 0..8: non-king piece values
    for pt in Piece::NON_KING {
        let n_white = (b[pt] & b[Color::White]).len() as i8;
        let n_black = (b[pt] & b[Color::Black]).len() as i8;
        let net = n_white - n_black;
        if net != 0 {
            let idx = 2 * (pt as usize);
            features.push((idx, phase * f32::from(net)));
            features.push((idx + 1, (1. - phase) * f32::from(net)));
        }
    }
    // just leave indices 9 and 10 unoccupied, I guess

    let offset = 10; // offset added to PST positions

    let bocc = b[Color::Black];
    let wocc = b[Color::White];

    // Get piece-square quantities
    for pt in Piece::ALL {
        let pt_idx = pt as usize;
        for sq in b[pt] {
            let alt_sq = sq.opposite();
            let increment = match (wocc.contains(sq), bocc.contains(alt_sq)) {
                (true, false) => 1.,
                (false, true) => -1.,
                _ => continue,
            };
            let idx = offset + 128 * pt_idx + 2 * (sq as usize);
            features.push((idx, phase * increment));
            features.push((idx + 1, (1. - phase) * increment));
        }
    }

    features
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_data() {
        assert_eq!(compute_gradient(&[], &[]), (Vec::new(), 0.0));
    }

    #[test]
    fn perfect_results() {
        let res = compute_gradient(&[(vec![(0, 1e9)], 1.0), (vec![(0, -1e9)], 0.0)], &[1e9]);
        assert_eq!(res, (vec![0.0], 0.0));
    }

    #[test]
    fn get_bigger() {
        let res = compute_gradient(&[(vec![(0, 10.0)], 1.0)], &[0.0]);

        assert_eq!(res.0.len(), 1);
        assert!((res.0[0] + 1.25).abs() < 0.01);
        assert!((res.1 - 0.25).abs() < 0.01);
    }

    #[test]
    fn zero_data() {
        let res = compute_gradient(&[(Vec::new(), 1.0)], &[2.0, -1.0]);

        assert_eq!(res.0, vec![0.0; 2]);
        assert!((res.1 - 0.25) < 0.01);
    }

    #[test]
    fn evens_out() {
        let res = compute_gradient(&[(vec![(0, 1.0)], 1.0), (vec![(0, 1.0)], 0.0)], &[0.0]);

        assert_eq!(res.0.len(), 1);
        assert!(res.0[0].abs() < 0.01);
        assert!((res.1 - 0.5).abs() < 0.01);
    }
}
