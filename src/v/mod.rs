mod new_v1;
mod new_v2;
mod new_v3;
mod new_v4;

pub use new_v1::NewV1;
pub use new_v2::NewV2;
pub use new_v3::NewV3;
pub use new_v4::NewV4;

/// Builders for multi-dimensional vectors.
pub struct V;

impl V {
    /// `V1<T>`` (`NVec<D1, T>``) builder.
    pub fn d1(self) -> NewV1 {
        NewV1
    }

    /// `V2<T>`` (`NVec<D2, T>``) builder.
    pub fn d2(self) -> NewV2 {
        NewV2
    }

    /// `V3<T>`` (`NVec<D3, T>``) builder.
    pub fn d3(self) -> NewV3 {
        NewV3
    }

    /// `V4<T>`` (`NVec<D4, T>``) builder.
    pub fn d4(self) -> NewV4 {
        NewV4
    }
}
