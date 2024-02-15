#![allow(dead_code)]
use std::iter::zip;

const ITERS: usize = 20;
fn main() {
    let row_one = vec![7f64, -2f64]; //0
    let row_two = vec![5f64, 1f64]; //1
    let complete = vec![row_one, row_two];
    let soln_matrix = vec![45f64, 37f64];

    let val = gauss_seidel(&complete, &soln_matrix);
    println!("{:?}", val.unwrap());
}

fn euclidean_error(prev: &[f64], curr: &[f64]) -> Option<f64> {
    let mut res = 1f64;
    if prev.len() != curr.len() {
        eprintln!(
            "The dimensions of the previous matrix [{}] is not equal to the current matrix [{}]",
            prev.len(),
            curr.len()
        );
        return None;
    }
    //calculate the euclidean distance between the 2 points. So for each element in the prev and curr, we square them and add to res
    for (previous, current) in zip(prev, curr) {
        res += (current - previous).powi(2);
    }
    //return the sqrt
    Some(res.sqrt())
}

fn check_squareness(matrix: &[Vec<f64>], x: &[f64]) -> bool {
    let rownum = matrix.len();
    for rows in matrix {
        if rows.len() != rownum {
            return false;
        }
    }
    if x.len() != matrix.len() {
        return false;
    }
    true
}
//Will only work for diagonally dominant matrices
//or symmetric matrices
fn gauss_seidel(matrix: &[Vec<f64>], x: &[f64]) -> Option<Vec<f64>> {
    //check for square matrix
    match check_squareness(matrix, x) {
        true => {}
        false => return None,
    }
    let matrix_len = matrix.len();
    let mut count = 0;
    //initialize return_value with 0s
    let mut return_value = Vec::new();
    return_value.extend((0..matrix_len).map(|_| 0f64));
    //iterations itself
    while count < ITERS {
        //updating each value in `return_value`
        for index in 0..matrix_len {
            let mut current = 0f64;
            //the actual calculation of each updated value
            for (indextwo, _) in return_value.iter().enumerate(){
                if indextwo == index {
                    continue;
                }
                current += (-matrix[index][indextwo] * return_value[indextwo]) + x[index];
                current /= matrix[index][index]; //correct
            }
            return_value[index] = current;
        }
        count += 1;
    }
    Some(return_value)
}


fn gauss_jordan(_matrix: &[Vec<f64>], _x: &[f64]) -> Option<Vec<f64>>{
    todo!()
}
