use crate::{Completed, IntoCompleted, D1};

impl<T> IntoCompleted<D1, T> for Vec<T> {
    fn into_completed(self, complete_with: T) -> Completed<Self, D1, T> {
        Completed::new(self, complete_with)
    }
}

impl<T> IntoCompleted<D1, T> for &Vec<T> {
    fn into_completed(self, complete_with: T) -> Completed<Self, D1, T> {
        Completed::new(self, complete_with)
    }
}

impl<T> IntoCompleted<D1, T> for &mut Vec<T> {
    fn into_completed(self, complete_with: T) -> Completed<Self, D1, T> {
        Completed::new(self, complete_with)
    }
}
