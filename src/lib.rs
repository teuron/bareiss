use core::ops::Div;
use core::ops::Mul;
use core::ops::Sub;

pub fn bareiss_determinant<T>(matrix: &mut Vec<T>, rows: usize) -> Result<T, &'static str>
where
    T: Sub<Output = T> + Div<Output = T> + Mul<Output = T> + Copy,
{
    if matrix.len() != rows * rows {
        return Err("Not a square matrix");
    }

    let index = |x, y| x * rows + y;

    for i in 0..(rows - 1) {
        for j in (i + 1)..rows {
            for k in (i + 1)..rows {
                let jki = matrix[index(j, k)] * matrix[index(i, i)];
                let jik = matrix[index(j, i)] * matrix[index(i, k)];
                matrix[index(j, k)] = jki - jik;
                if i != 0 {
                    matrix[index(j, k)] = matrix[index(j, k)] / matrix[index(i - 1, i - 1)];
                }
            }
        }
    }
    Ok(matrix[index(rows - 1, rows - 1)])
}

#[cfg(test)]
mod tests {
    use super::bareiss_determinant;
    #[test]
    fn calculates_right_determinant() {
        let mut matrix = vec![1, 2, 3, 10];
        assert_eq!(bareiss_determinant(&mut matrix, 2).unwrap(), 4);
    }

    #[test]
    #[should_panic]
    fn panic_on_non_square_matrix() {
        let mut matrix = vec![1, 2, 3, 10];
        bareiss_determinant(&mut matrix, 4).unwrap();
    }
}
