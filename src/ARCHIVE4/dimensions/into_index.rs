use super::{dim::*, LowerThan};

pub trait IntoIndex<D: Dim>: Sized + Copy {
    fn into_index(self) -> D::Idx;

    fn split(self) -> (usize, <D::PREVIOUS as Dim>::Idx)
    where
        D::PREVIOUS: LowerThan<D>;
}

// arrays

impl IntoIndex<D0> for [usize; 0] {
    #[inline(always)]
    fn into_index(self) -> [usize; 0] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D1 as Dim>::PREVIOUS as Dim>::Idx) {
        (usize::MAX, [])
    }
}

impl IntoIndex<D1> for [usize; 1] {
    #[inline(always)]
    fn into_index(self) -> [usize; 1] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D1 as Dim>::PREVIOUS as Dim>::Idx) {
        (self[0], [])
    }
}

impl IntoIndex<D2> for [usize; 2] {
    #[inline(always)]
    fn into_index(self) -> [usize; 2] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D2 as Dim>::PREVIOUS as Dim>::Idx) {
        (self[0], [self[1]])
    }
}

impl IntoIndex<D3> for [usize; 3] {
    #[inline(always)]
    fn into_index(self) -> [usize; 3] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D3 as Dim>::PREVIOUS as Dim>::Idx) {
        (self[0], [self[1], self[2]])
    }
}

impl IntoIndex<D4> for [usize; 4] {
    #[inline(always)]
    fn into_index(self) -> [usize; 4] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D4 as Dim>::PREVIOUS as Dim>::Idx) {
        (self[0], [self[1], self[2], self[3]])
    }
}

impl IntoIndex<D5> for [usize; 5] {
    #[inline(always)]
    fn into_index(self) -> [usize; 5] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D5 as Dim>::PREVIOUS as Dim>::Idx) {
        (self[0], [self[1], self[2], self[3], self[4]])
    }
}

impl IntoIndex<D6> for [usize; 6] {
    #[inline(always)]
    fn into_index(self) -> [usize; 6] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D6 as Dim>::PREVIOUS as Dim>::Idx) {
        (self[0], [self[1], self[2], self[3], self[4], self[5]])
    }
}

impl IntoIndex<D7> for [usize; 7] {
    #[inline(always)]
    fn into_index(self) -> [usize; 7] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D7 as Dim>::PREVIOUS as Dim>::Idx) {
        (
            self[0],
            [self[1], self[2], self[3], self[4], self[5], self[6]],
        )
    }
}

impl IntoIndex<D8> for [usize; 8] {
    #[inline(always)]
    fn into_index(self) -> [usize; 8] {
        self
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D8 as Dim>::PREVIOUS as Dim>::Idx) {
        (
            self[0],
            [
                self[1], self[2], self[3], self[4], self[5], self[6], self[7],
            ],
        )
    }
}

// tuples

impl IntoIndex<D1> for usize {
    #[inline(always)]
    fn into_index(self) -> [usize; 1] {
        [self]
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D1 as Dim>::PREVIOUS as Dim>::Idx) {
        (self, [])
    }
}

impl IntoIndex<D2> for (usize, usize) {
    #[inline(always)]
    fn into_index(self) -> [usize; 2] {
        [self.0, self.1]
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D2 as Dim>::PREVIOUS as Dim>::Idx) {
        (self.0, [self.1])
    }
}

impl IntoIndex<D3> for (usize, usize, usize) {
    #[inline(always)]
    fn into_index(self) -> [usize; 3] {
        [self.0, self.1, self.2]
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D3 as Dim>::PREVIOUS as Dim>::Idx) {
        (self.0, [self.1, self.2])
    }
}

impl IntoIndex<D4> for (usize, usize, usize, usize) {
    #[inline(always)]
    fn into_index(self) -> [usize; 4] {
        [self.0, self.1, self.2, self.3]
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D4 as Dim>::PREVIOUS as Dim>::Idx) {
        (self.0, [self.1, self.2, self.3])
    }
}

impl IntoIndex<D5> for (usize, usize, usize, usize, usize) {
    #[inline(always)]
    fn into_index(self) -> [usize; 5] {
        [self.0, self.1, self.2, self.3, self.4]
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D5 as Dim>::PREVIOUS as Dim>::Idx) {
        (self.0, [self.1, self.2, self.3, self.4])
    }
}

impl IntoIndex<D6> for (usize, usize, usize, usize, usize, usize) {
    #[inline(always)]
    fn into_index(self) -> [usize; 6] {
        [self.0, self.1, self.2, self.3, self.4, self.5]
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D6 as Dim>::PREVIOUS as Dim>::Idx) {
        (self.0, [self.1, self.2, self.3, self.4, self.5])
    }
}

impl IntoIndex<D7> for (usize, usize, usize, usize, usize, usize, usize) {
    #[inline(always)]
    fn into_index(self) -> [usize; 7] {
        [self.0, self.1, self.2, self.3, self.4, self.5, self.6]
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D7 as Dim>::PREVIOUS as Dim>::Idx) {
        (self.0, [self.1, self.2, self.3, self.4, self.5, self.6])
    }
}

impl IntoIndex<D8> for (usize, usize, usize, usize, usize, usize, usize, usize) {
    #[inline(always)]
    fn into_index(self) -> [usize; 8] {
        [
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        ]
    }

    #[inline(always)]
    fn split(self) -> (usize, <<D8 as Dim>::PREVIOUS as Dim>::Idx) {
        (
            self.0,
            [self.1, self.2, self.3, self.4, self.5, self.6, self.7],
        )
    }
}
