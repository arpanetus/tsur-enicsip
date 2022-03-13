use matrix_transposition::*;


fn main() {
    let matrix: Matrix<i32> = Matrix::new(vec!(vec![1, 2, 3], vec!(4, 5, 6), vec!(7, 8, 9), vec!(10, 11, 12)));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}