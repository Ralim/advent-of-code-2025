use array2d::Array2D;
use rayon::prelude::*;
use shared::{ChallengeDay, Question, get_question_data_lines};
use std::collections::HashMap;

fn main() {
    let t_a = std::thread::spawn(|| {
        let ans = part_a(Question::Question);
        println!("Part A:{ans}");
    });
    let t_b = std::thread::spawn(|| {
        let ans = part_b(Question::Question);
        println!("Part B:{ans}");
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}

#[derive(Debug)]
struct Machine {
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<i128>>,
    voltages: Vec<i128>,
}

struct SearchParams<'a> {
    free_vars: &'a [usize],
    matrix: &'a Array2D<i128>,
    pivot_col_to_row: &'a HashMap<usize, usize>,
    num_buttons: usize,
}

fn machine_from_line(line: &str) -> Machine {
    use regex::Regex;

    let re = Regex::new(r"\[([.#]*)\]\s+(.+)\s+\{([0-9,]+)\}").unwrap();
    let caps = re.captures(line).unwrap();

    let indicator_part = &caps[1];
    let middle_part = &caps[2];
    let end_part = &caps[3];

    let indicator_lights: Vec<bool> = indicator_part.chars().map(|c| c == '#').collect();

    let button_re = Regex::new(r"\(([0-9,]+)\)").unwrap();
    let buttons: Vec<Vec<i128>> = button_re
        .captures_iter(middle_part)
        .map(|cap| {
            cap[1]
                .split(',')
                .map(|s| s.parse::<i128>().unwrap())
                .collect()
        })
        .collect();
    let voltages: Vec<i128> = end_part
        .split(',')
        .map(|s| s.parse::<i128>().unwrap())
        .collect();

    Machine {
        indicator_lights,
        buttons,
        voltages,
    }
}

fn load_machines(question: Question) -> Vec<Machine> {
    get_question_data_lines(ChallengeDay::Day10, question)
        .into_iter()
        .map(|line| machine_from_line(&line))
        .collect()
}

impl Machine {
    fn find_optimal_button_presses_count(&self) -> usize {
        let rows = self.indicator_lights.len();
        let cols = self.buttons.len();

        // Build augmented matrix [A | b]
        let mut matrix = Array2D::filled_with(0i128, rows, cols + 1);

        // Set coefficients where button c toggles light r
        for (c, button) in self.buttons.iter().enumerate().take(cols) {
            for &light_idx in button {
                if light_idx >= 0 && (light_idx as usize) < rows {
                    matrix[(light_idx as usize, c)] = 1;
                }
            }
        }

        // Set target column (desired light states)
        for (r, &light_state) in self.indicator_lights.iter().enumerate().take(rows) {
            matrix[(r, cols)] = if light_state { 1 } else { 0 };
        }

        // Perform GF(2) Gaussian elimination
        let pivot_cols = self.perform_elimination(&mut matrix, true);

        // Check consistency
        self.check_consistency(&matrix);

        // Find minimum solution for GF(2)
        self.find_minimum_gf2_solution(&matrix, &pivot_cols)
    }

    fn find_optimal_button_presses_for_voltage(&self) -> usize {
        let num_requirements = self.voltages.len();
        let num_buttons = self.buttons.len();

        // Build coefficient matrix
        let coeff_matrix = {
            let buttons: &[Vec<i128>] = &self.buttons;
            let mut matrix = Array2D::filled_with(0i128, num_requirements, buttons.len());

            for (c, button) in buttons.iter().enumerate() {
                for &req_idx in button {
                    if req_idx >= 0 && (req_idx as usize) < num_requirements {
                        matrix[(req_idx as usize, c)] = 1;
                    }
                }
            }

            matrix
        };

        // Create augmented matrix [A | b]
        let mut matrix = Array2D::filled_with(0i128, num_requirements, num_buttons + 1);

        // Copy coefficient matrix and add target values
        for r in 0..num_requirements {
            for c in 0..num_buttons {
                matrix[(r, c)] = coeff_matrix[(r, c)];
            }
            matrix[(r, num_buttons)] = self.voltages[r];
        }

        // Perform integer Gaussian elimination
        let pivot_col_to_row = self.perform_elimination(&mut matrix, false);

        // Check consistency
        self.check_consistency(&matrix);

        // Find minimum solution for integer system
        self.find_minimum_integer_solution(&matrix, &pivot_col_to_row)
    }

    fn perform_elimination(
        &self,
        matrix: &mut Array2D<i128>,
        is_gf2: bool,
    ) -> HashMap<usize, usize> {
        let rows = matrix.num_rows();
        let cols = matrix.num_columns() - 1; // Exclude augmented column
        let mut pivot_row = 0;
        let mut pivot_map = HashMap::new();
        //print_array(&matrix);
        //
        for c in 0..cols {
            if pivot_row >= rows {
                break;
            }

            // Find pivot row, which is the lower right corner
            if let Some(r_idx) = (pivot_row..matrix.num_rows()).find(|&r| matrix[(r, c)] != 0) {
                // Swap rows
                if r_idx != pivot_row {
                    for k in 0..matrix.num_columns() {
                        let temp = matrix[(pivot_row, k)];
                        matrix[(pivot_row, k)] = matrix[(r_idx, k)];
                        matrix[(r_idx, k)] = temp;
                    }
                }

                if is_gf2 {
                    // GF(2) elimination (XOR operations)
                    for r in 0..rows {
                        if r != pivot_row && matrix[(r, c)] == 1 {
                            for k in c..matrix.num_columns() {
                                matrix[(r, k)] ^= matrix[(pivot_row, k)];
                            }
                        }
                    }
                } else {
                    // Integer elimination (fraction-free Gaussian elimination)
                    let pivot_val = matrix[(pivot_row, c)];
                    for r in pivot_row + 1..rows {
                        if matrix[(r, c)] != 0 {
                            let factor = matrix[(r, c)];
                            for k in c..matrix.num_columns() {
                                matrix[(r, k)] =
                                    matrix[(r, k)] * pivot_val - matrix[(pivot_row, k)] * factor;
                            }
                        }
                    }
                }

                pivot_map.insert(c, pivot_row);
                pivot_row += 1;
            }
        }

        pivot_map
    }

    fn check_consistency(&self, matrix: &Array2D<i128>) {
        let rows = matrix.num_rows();
        let cols = matrix.num_columns() - 1; // Exclude augmented column

        for r in 0..rows {
            let is_all_zeros = (0..cols).all(|c| matrix[(r, c)] == 0);
            if is_all_zeros && matrix[(r, cols)] != 0 {
                panic!("No solution for machine: {self:?}");
            }
        }
    }

    fn find_minimum_gf2_solution(
        &self,
        matrix: &Array2D<i128>,
        pivot_cols: &HashMap<usize, usize>,
    ) -> usize {
        let cols = matrix.num_columns() - 1;
        let free_vars: Vec<usize> = (0..cols).filter(|c| !pivot_cols.contains_key(c)).collect();

        let mut min_presses = usize::MAX;
        let num_free = free_vars.len();
        let combinations = 1 << num_free;

        for i in 0..combinations {
            let mut x = vec![0i128; cols];

            // Assign free variables
            for (bit_idx, &col_idx) in free_vars.iter().enumerate() {
                if (i >> bit_idx) & 1 == 1 {
                    x[col_idx] = 1;
                }
            }

            // Back-substitution
            let mut pivot_cols_vec: Vec<_> = pivot_cols.iter().collect();
            pivot_cols_vec.sort_by_key(|(_, row)| *row);

            for &(&p_col, &r_idx) in pivot_cols_vec.iter().rev() {
                let mut sum = 0i128;
                for c in (p_col + 1)..cols {
                    if matrix[(r_idx, c)] == 1 && x[c] == 1 {
                        sum ^= 1;
                    }
                }
                x[p_col] = matrix[(r_idx, cols)] ^ sum;
            }

            let presses: usize = x.iter().map(|&v| v as usize).sum();
            min_presses = min_presses.min(presses);
        }

        min_presses
    }

    fn find_minimum_integer_solution(
        &self,
        matrix: &Array2D<i128>,
        pivot_col_to_row: &HashMap<usize, usize>,
    ) -> usize {
        let num_buttons = matrix.num_columns() - 1;
        let free_vars: Vec<usize> = (0..num_buttons)
            .filter(|c| !pivot_col_to_row.contains_key(c))
            .collect();

        let mut min_total: Option<usize> = None;
        let mut current_free_vals = vec![0i64; free_vars.len()];

        let search_params = SearchParams {
            free_vars: &free_vars,
            matrix,
            pivot_col_to_row,
            num_buttons,
        };

        self.search_integer_solutions(0, &mut current_free_vals, &search_params, &mut min_total);

        min_total.unwrap()
    }

    fn search_integer_solutions(
        &self,
        idx: usize,
        free_vals: &mut [i64],
        params: &SearchParams,
        min_total: &mut Option<usize>,
    ) {
        // Once past the end of the optimisation effort, stop searching and if valid, yeet
        if idx == params.free_vars.len() {
            if let Some(total) = self.evaluate_solution(
                params.free_vars,
                free_vals,
                params.matrix,
                params.pivot_col_to_row,
                params.num_buttons,
            ) && (min_total.is_none() || total < min_total.unwrap())
            {
                *min_total = Some(total);
            }
            return;
        }

        let optimisation_effort = if params.free_vars.len() > 1 {
            200
        } else {
            2000
        };
        for v in 0..=optimisation_effort {
            free_vals[idx] = v;
            self.search_integer_solutions(idx + 1, free_vals, params, min_total);
        }
    }

    fn evaluate_solution(
        &self,
        free_vars: &[usize],
        free_vals: &[i64],
        matrix: &Array2D<i128>,
        pivot_col_to_row: &HashMap<usize, usize>,
        num_buttons: usize,
    ) -> Option<usize> {
        let mut x = vec![0i128; num_buttons];

        // Set free variables
        for (i, &fv) in free_vars.iter().enumerate() {
            x[fv] = free_vals[i] as i128;
        }

        // Back substitution
        let mut row_to_pivot_col = HashMap::new();
        for (&c, &r) in pivot_col_to_row {
            row_to_pivot_col.insert(r, c);
        }

        let num_pivots = pivot_col_to_row.len();
        for r in (0..num_pivots).rev() {
            let pc = row_to_pivot_col[&r];
            let pivot_val = matrix[(r, pc)];

            let mut rhs = matrix[(r, num_buttons)];
            for k in pc + 1..num_buttons {
                rhs -= matrix[(r, k)] * x[k];
            }

            if rhs % pivot_val != 0 || rhs / pivot_val < 0 {
                return None;
            }

            x[pc] = rhs / pivot_val;
        }

        let sum: i128 = x.iter().sum();
        if sum >= 0 && sum < i64::MAX as i128 {
            Some(sum as usize)
        } else {
            None
        }
    }
}

fn part_a(question: Question) -> usize {
    println!("Starting Part A");
    load_machines(question)
        .into_par_iter()
        .map(|machine| machine.find_optimal_button_presses_count())
        .sum()
}

fn part_b(question: Question) -> usize {
    println!("Starting Part B");
    load_machines(question)
        .into_par_iter()
        .map(|machine| {
            let presses = machine.find_optimal_button_presses_for_voltage();
            assert!(presses > 0);
            presses
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 7);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 33);
    }

    #[test]
    fn test_machine_from_line() {
        let input = "[.#.] (0,2)(1) {10,20,30}";
        let machine = machine_from_line(input);

        assert_eq!(machine.indicator_lights, vec![false, true, false]);
        assert_eq!(machine.buttons, vec![vec![0, 2], vec![1]]);
        assert_eq!(machine.voltages, vec![10, 20, 30]);
    }
}
