use crate::{dimensions::*, Completed, IntoCompleted, NVec};

impl<T, E> IntoCompleted<D2, T> for Vec<E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
    // TODO: the constraint must be E: NVec<<D2 as Dim>::PREVIOUS, T> OR NVecRef<<D2 as Dim>::PREVIOUS, T>,
{
    fn into_completed(self, complete_with: T) -> Completed<Self, D2, T> {
        Completed::new(self, complete_with)
    }
}

impl<T, E> IntoCompleted<D2, T> for &Vec<E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn into_completed(self, complete_with: T) -> Completed<Self, D2, T> {
        Completed::new(self, complete_with)
    }
}

impl<T, E> IntoCompleted<D2, T> for &mut Vec<E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn into_completed(self, complete_with: T) -> Completed<Self, D2, T> {
        Completed::new(self, complete_with)
    }
}
