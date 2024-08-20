use crate::{dimensions::*, Completed, IntoCompleted, NVec};

macro_rules! implement {
    ($dim:tt) => {
        impl<T, E> IntoCompleted<$dim, T> for Vec<E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn into_completed(self, complete_with: T) -> Completed<Self, $dim, T> {
                Completed::new(self, complete_with)
            }
        }

        impl<T, E> IntoCompleted<$dim, T> for &Vec<E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn into_completed(self, complete_with: T) -> Completed<Self, $dim, T> {
                Completed::new(self, complete_with)
            }
        }

        impl<T, E> IntoCompleted<$dim, T> for &mut Vec<E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn into_completed(self, complete_with: T) -> Completed<Self, $dim, T> {
                Completed::new(self, complete_with)
            }
        }
    };
}

implement!(D3);
implement!(D4);
implement!(D5);
implement!(D6);
implement!(D7);
implement!(D8);
