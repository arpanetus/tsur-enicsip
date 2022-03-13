// The original exercise requires the usage of 2d tuple of size 2x2, 
// but I used a vector.
//   
// I badly wanted to find Dependent/Refined Types for rust, 
// but ended up getting PhantomData, which is also nice.
// Yet I didn't want to use PhantomData, since it would be 
// an overkill for this exercise for me now:
// https://jadpole.github.io/rust/typechecked-matrix
#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    v: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(v: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { v: v }
    }
}

pub fn transpose<T: Clone + Copy>(m: Matrix<T>) -> Matrix<T> {
    let rn = &m.v.len();
    if *rn == 0 as usize {
        return m;
    }

    let cn = &m.v[0].len();
    if *cn == 0 as usize {
        return m;
    }

    let mut ret_m: Vec<Vec<T>> = vec![vec![m.v[0][0]; *rn]; *cn];
    for i in 0..*rn {
        for j in 0..*cn {
            ret_m[j][i] = m.v[i][j]
        }
    }

    return Matrix::new(ret_m);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_line() {
        let matrix: Matrix<i32> = Matrix::new(vec![vec![1, 2, 3]]);
        let expected: Matrix<i32> = Matrix::new(vec![vec![1], vec![2], vec![3]]);
        assert_eq!(transpose(matrix), expected)
    }

    #[test]
    fn test_one_line_but_columns() {
        let matrix: Matrix<i32> = Matrix::new(vec![vec![1], vec![2], vec![3]]);
        let expected: Matrix<i32> = Matrix::new(vec![vec![1, 2, 3]]);
        assert_eq!(transpose(matrix), expected)
    }

    #[test]
    fn test_square() {
        let matrix: Matrix<i32> = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let expected: Matrix<i32> = Matrix::new(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
        assert_eq!(transpose(matrix), expected)
    }

    #[test]
    fn test_rectangle() {
        let matrix: Matrix<i32> = Matrix::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ]);
        let expected: Matrix<i32> = Matrix::new(vec![
            vec![1, 4, 7, 10],
            vec![2, 5, 8, 11],
            vec![3, 6, 9, 12],
        ]);
        assert_eq!(transpose(matrix), expected)
    }

    #[test]
    #[should_panic]
    fn test_matrix_has_to_be_filled() {
        let matrix: Matrix<i32> = Matrix::new(vec![
            vec![1, 2, 3],
            vec![4],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ]);
        transpose(matrix);
    }
}
