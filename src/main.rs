#![allow(dead_code)]
#![allow(unused_variables)]

use std::iter::zip;

const ITERS: usize = 5;
fn main() {
    // let row_one = vec![2f64, 1f64, -1f64]; //0
    // let row_two = vec![-3f64, 5f64, 2f64];
    // let row_three = vec![4f64, -6f64, 5f64];
    // let complete = vec![row_one, row_two, row_three];
    // let soln_matrix = vec![8f64, -4f64, 6f64];

    // let val = gauss_seidel(&complete, &soln_matrix);
    // println!("{:?}", val.unwrap());
    println!("The logsqrt of 4 is {}", logsqrt(4f32));
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
            for (indextwo, _) in return_value.iter().enumerate() {
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

fn gauss_jordan(matrix: &mut [Vec<f64>], x: &mut [f64]) -> Option<Vec<f64>> {
    //no pivot solution of a system of matrices
    //assume that no pivot entry is 0
    if !check_for_zero_pivot(matrix) {
        return None;
    }
    let mut res = Vec::new();
    res.extend((0..matrix.len()).map(|_| 0f64));
    for rwindex in 0..matrix.len() {
        for indexone in rwindex + 1..=matrix.len() {
            matrix[indexone][rwindex] /= matrix[rwindex][rwindex];
            for indextwo in rwindex + 1..=matrix.len() {
                matrix[indexone][indextwo] -= matrix[indexone][rwindex] * matrix[rwindex][indextwo];
            }
        }
    }

    //forward elimination
    for indexthree in 0..matrix.len() {
        for indexfour in indexthree..=matrix.len() {
            x[indexfour] -= matrix[indexfour][indexthree] * x[indexthree];
        }
    }

    for indexfive in (0..=matrix.len()).rev() {
        let _current = x[indexfive];
        for _indexsix in indexfive..= 0 {

        }
    }

    Some(res)
}

fn check_for_zero_pivot(matrix: &[Vec<f64>]) -> bool {
    for (i, row) in matrix.iter().enumerate() {
        for (_j, _) in row.iter().enumerate() {
            if matrix[i][i] == 0f64 {
                return false;
            }
        }
    }
    true
}
struct Pair(f64, f64);

impl Pair{
    fn new(x: f64, y: f64) -> Self{
        Pair(x, y)
    }

    fn from_closure<F>(x: f64, f: F) -> Self
    where F: Fn(f64) -> f64,
    {
        Pair(x, f(x))
    }

    fn unpack(self) -> (f64, f64){
        (self.0, self.1)
    }

    fn ref_unpack(&self) -> (f64, f64){
        (self.0, self.1)
    }

    fn getx(&self) -> f64{
        self.0
    }

    fn gety(&self) -> f64{
        self.1
    }

}

fn lagrange_interpolation(vector: &[Pair], point: f64) -> f64{
    let mut res = 0f64;
    //for every pair (x, y) do stuff
    for (index, pair) in vector.iter().enumerate(){
        let mut num_accum = 1f64;
        let mut denum_accum = 1f64;

        for (indextwo, paired) in vector.iter().enumerate(){
            if indextwo == index{
                continue
            }
            num_accum *= point - paired.getx();
            denum_accum *= pair.getx() - paired.getx();
        }

        res += (num_accum/denum_accum) * pair.gety();
    } 
    res
}

fn rref(matrix: &mut [Vec<f64>], x: &mut [f64]) -> Option<Vec<f64>>{
    let mut res: Vec<f64> = Vec::new();
    for (index, _) in matrix.iter().enumerate(){
        if matrix[index][index] == 0f64{
            return None
        }

        for (indextwo, _) in matrix.iter().enumerate(){
            if index == indextwo{
                continue
            }
            let ratio = matrix[indextwo][index]/matrix[index][index];
            for (indexthree, _) in matrix.iter().enumerate(){
                matrix[indextwo][indexthree] = matrix[indextwo][indexthree] - ratio * matrix[index][indexthree];
            }
        }
    }
    Some(res)

}

fn logsqrt(num: f32) -> f32 {
    let lognum = num.log10();
    10.0f32.powf(lognum / 2f32)
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_lagrange(){
        let vector = vec![(0f64, 2f64), (1f64, 4f64), (2f64, 6f64), (3f64, 8f64)];
        let finished = vector.iter().map(|x| Pair::new(x.0, x.1)).collect::<Vec<Pair>>();
        assert_eq!(lagrange_interpolation(&finished, 4f64), 10f64);
        assert_eq!(lagrange_interpolation(&finished, 5f64), 12f64);
        assert_eq!(lagrange_interpolation(&finished, 6f64), 14f64);
        assert_eq!(lagrange_interpolation(&finished, 7f64), 16f64);
    }
}
