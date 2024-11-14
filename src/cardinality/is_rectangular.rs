use crate::{Dim, NVecCore, D1, D2, D3, D4};

pub trait IsRectangular: Dim {
    fn is_rectangular<T>(vec: &impl NVecCore<Self, T>) -> bool;
}

impl IsRectangular for D1 {
    fn is_rectangular<T>(_: &impl NVecCore<Self, T>) -> bool {
        true
    }
}

impl IsRectangular for D2 {
    fn is_rectangular<T>(vec: &impl NVecCore<Self, T>) -> bool {
        match vec.core_card([]) {
            0 => true,
            n1 => {
                let n2 = vec.core_card([0]);
                for i1 in 1..n1 {
                    if vec.core_card([i1]) != n2 {
                        return false;
                    }
                }
                true
            }
        }
    }
}

impl IsRectangular for D3 {
    fn is_rectangular<T>(vec: &impl NVecCore<Self, T>) -> bool {
        let mut equal_n3 = None;
        match vec.core_card([]) {
            0 => true,
            n1 => {
                let n2 = vec.core_card([0]);
                for i1 in 0..n1 {
                    if vec.core_card([i1]) != n2 {
                        return false;
                    }

                    if n2 > 0 {
                        let n3 = match equal_n3 {
                            None => {
                                let n3 = vec.core_card([i1, 0]);
                                equal_n3 = Some(n3);
                                n3
                            }
                            Some(n3) => n3,
                        };

                        for i2 in 0..n2 {
                            if vec.core_card([i1, i2]) != n3 {
                                return false;
                            }
                        }
                    }
                }
                true
            }
        }
    }
}

impl IsRectangular for D4 {
    fn is_rectangular<T>(vec: &impl NVecCore<Self, T>) -> bool {
        let mut equal_n3 = None;
        let mut equal_n4 = None;
        match vec.core_card([]) {
            0 => true,
            n1 => {
                let n2 = vec.core_card([0]);
                for i1 in 0..n1 {
                    if vec.core_card([i1]) != n2 {
                        return false;
                    }

                    if n2 > 0 {
                        let n3 = match equal_n3 {
                            None => {
                                let n3 = vec.core_card([i1, 0]);
                                equal_n3 = Some(n3);
                                n3
                            }
                            Some(n3) => n3,
                        };
                        for i2 in 0..n2 {
                            if vec.core_card([i1, i2]) != n3 {
                                return false;
                            }

                            if n3 > 0 {
                                let n4 = match equal_n4 {
                                    None => {
                                        let n4 = vec.core_card([i1, i2, 0]);
                                        equal_n4 = Some(n4);
                                        n4
                                    }
                                    Some(n4) => n4,
                                };

                                for i3 in 0..n3 {
                                    if vec.core_card([i1, i2, i3]) != n4 {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
                true
            }
        }
    }
}
