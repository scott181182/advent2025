use std::ops::{Add, Div};

macro_rules! impl_vector3_dist2 {
    ($t:ty) => {
        impl Vector3<$t> {
            pub fn dist2(&self, other: &Vector3<$t>) -> $t {
                let dx = self.x - other.x;
                let dy = self.y - other.y;
                let dz = self.z - other.z;
                dx * dx + dy * dy + dz * dz
            }
        }
    };
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl_vector3_dist2!(i64);

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
macro_rules! impl_op_add {
    ($left:ty, $right:ty, $output:ty) => {
        impl Add<$right> for $left {
            type Output = $output;

            fn add(self, other: $right) -> Self::Output {
                Vector3 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }
    };
}
impl_op_add!(Vector3<i64>, Vector3<i64>, Vector3<i64>);
impl_op_add!(Vector3<i64>, &Vector3<i64>, Vector3<i64>);
impl_op_add!(&Vector3<i64>, Vector3<i64>, Vector3<i64>);
impl_op_add!(&Vector3<i64>, &Vector3<i64>, Vector3<i64>);

macro_rules! impl_op_div_scalar {
    ($left:ty, $right:ty, $output:ty) => {
        impl Div<$right> for $left {
            type Output = $output;

            fn div(self, other: $right) -> Self::Output {
                Vector3 {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                }
            }
        }
    };
}
impl_op_div_scalar!(Vector3<i64>, i64, Vector3<i64>);
impl_op_div_scalar!(Vector3<i64>, &i64, Vector3<i64>);
impl_op_div_scalar!(&Vector3<i64>, i64, Vector3<i64>);
impl_op_div_scalar!(&Vector3<i64>, &i64, Vector3<i64>);
