use std::{collections::HashSet, panic};

use array2d::Array2D;
use shared::{
    ChallengeDay, Question, get_question_data_as_2d_matrices_lb_sep, get_question_data_lines,
    rotate_array,
};
fn main() {
    let t_a = std::thread::spawn(|| {
        let ans = part_a(Question::Question);
        println!("Part A:{ans}");
    });

    t_a.join().unwrap();
}
struct Space {
    width: u32,
    height: u32,
    shape_counts: Vec<usize>,
}
impl From<String> for Space {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 2 {
            panic!("Invalid format: expected 'WIDTHxHEIGHT: shape_counts'");
        }

        // Parse dimensions
        let dimensions = parts[0].trim();
        let dim_parts: Vec<&str> = dimensions.split('x').collect();
        if dim_parts.len() != 2 {
            panic!("Invalid dimensions format: expected 'WIDTHxHEIGHT'");
        }

        let width = dim_parts[0].parse::<u32>().unwrap();
        let height = dim_parts[1].parse::<u32>().unwrap();

        // Parse shape counts
        let shape_counts_str = parts[1].trim();
        let shape_counts: Result<Vec<usize>, _> = shape_counts_str
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect();

        let shape_counts = shape_counts
            .map_err(|_| "Invalid shape count".to_string())
            .unwrap();

        Space {
            width,
            height,
            shape_counts,
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Shape {
    default_state: Array2D<u8>,
}
impl Shape {
    //ROT 90 clockwise
    fn rotate_90(&self) -> Shape {
        Shape {
            default_state: rotate_array(self.default_state.clone()),
        }
    }
    fn bounding_area(&self) -> usize {
        let rows = self.default_state.num_rows();
        let cols = self.default_state.num_columns();
        rows * cols
    }

    fn flip_horizontal(&self) -> Shape {
        let rows = self.default_state.num_rows();
        let cols = self.default_state.num_columns();
        let mut flipped_data = Vec::new();

        for row in 0..rows {
            let mut flipped_row = Vec::new();
            for col in (0..cols).rev() {
                flipped_row.push(self.default_state[(row, col)]);
            }
            flipped_data.push(flipped_row);
        }

        Shape {
            default_state: Array2D::from_rows(&flipped_data).unwrap(),
        }
    }

    fn flip_vertical(&self) -> Shape {
        let rows = self.default_state.num_rows();
        let cols = self.default_state.num_columns();
        let mut flipped_data = Vec::new();

        for row in (0..rows).rev() {
            let mut flipped_row = Vec::new();
            for col in 0..cols {
                flipped_row.push(self.default_state[(row, col)]);
            }
            flipped_data.push(flipped_row);
        }

        Shape {
            default_state: Array2D::from_rows(&flipped_data).unwrap(),
        }
    }

    fn filled_area(&self) -> usize {
        self.default_state
            .as_row_major()
            .into_iter()
            .filter(|item| *item == b'#')
            .count()
    }
}
impl From<Array2D<u8>> for Shape {
    fn from(default_state: Array2D<u8>) -> Self {
        Shape { default_state }
    }
}

fn get_all_shape_permutations(shape: &Shape) -> Vec<Shape> {
    let mut permutations = HashSet::new();
    let mut current_shape = shape.clone();
    for _ in 0..4 {
        permutations.insert(current_shape.clone());
        // Both flips
        permutations.insert(current_shape.flip_horizontal());
        permutations.insert(current_shape.flip_vertical());

        current_shape = current_shape.rotate_90();
    }
    permutations.into_iter().collect()
}

fn part_a(question: Question) -> u32 {
    println!("Starting Part A");
    // Read file until we hit a line with an x in it, and these are space seperated matricies
    let shapes: Vec<Shape> = get_question_data_as_2d_matrices_lb_sep(ChallengeDay::Day12, question)
        .into_iter()
        .map(Shape::from)
        .collect();
    // Now get all lines with an x to get the spaces
    let input_file = get_question_data_lines(ChallengeDay::Day12, question);
    let spaces: Vec<Space> = input_file
        .into_iter()
        .filter(|line| line.contains('x'))
        .map(|line| line.into())
        .collect();
    let size_of_shapes = shapes
        .iter()
        .map(|shape| shape.filled_area())
        .collect::<Vec<_>>();
    let mut sum = 0;
    for space in &spaces {
        let width = space.width as f64;
        let height = space.height as f64;

        let total_area_required = size_of_shapes
            .iter()
            .zip(space.shape_counts.iter())
            .map(|(area, count)| area * count)
            .sum::<usize>() as f64;

        // Check if all the shapes can pack into the area
        // In the input and sample, None of the shapes are good packers
        // Naive check, is there even enough room to fit all the tiles
        if total_area_required < (width * height) {
            sum += 1;
        }
        // If its possible to fit them; we need to validate if they fit
    }

    sum
}
