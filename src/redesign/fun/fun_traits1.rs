pub trait FunWithData1<D, T, Fr> {
    fn exe(&self, data: &D, i: usize) -> T;
}
