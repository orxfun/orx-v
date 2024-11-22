mod matrix;
mod matrix_col_major;
mod matrix_mut;
mod matrix_row_major;
mod v1;
mod v2;

pub use matrix::Matrix;
pub use matrix_col_major::{MatrixColMajor, MatrixColMajorMut};
pub use matrix_mut::MatrixMut;
pub use matrix_row_major::{MatrixRowMajor, MatrixRowMajorMut};
pub use v1::{V1AsMatrix, V1Matrix, V1MatrixColMajor, V1MatrixLayout, V1MatrixRowMajor};
pub use v2::{V2AsMatrix, V2MatrixColMajor, V2MatrixRowMajor};
