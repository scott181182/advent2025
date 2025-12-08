use std::{
    hash::Hash,
    ops::{Add, Div},
};

macro_rules! impl_op_add {
    ($left:ty, $right:ty, $output:ty) => {
        impl Add<$right> for $left {
            type Output = $output;

            fn add(self, other: $right) -> Self::Output {
                Self::Output {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }
    };
}

macro_rules! impl_op_div_scalar {
    ($left:ty, $right:ty, $output:ty) => {
        impl Div<$right> for $left {
            type Output = $output;

            fn div(self, other: $right) -> Self::Output {
                Self::Output {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                }
            }
        }
    };
}

macro_rules! impl_vec {
    ($name:ident, $component:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name {
            pub x: $component,
            pub y: $component,
            pub z: $component,
        }

        impl $name {
            pub fn new(x: $component, y: $component, z: $component) -> Self {
                Self { x, y, z }
            }
            pub fn dist2(&self, other: &$name) -> $component {
                let dx = self.x - other.x;
                let dy = self.y - other.y;
                let dz = self.z - other.z;
                dx * dx + dy * dy + dz * dz
            }
        }

        impl_op_add!($name, $name, $name);
        impl_op_add!($name, &$name, $name);
        impl_op_add!(&$name, $name, $name);
        impl_op_add!(&$name, &$name, $name);

        impl_op_div_scalar!($name, i64, $name);
        impl_op_div_scalar!($name, &i64, $name);
        impl_op_div_scalar!(&$name, i64, $name);
        impl_op_div_scalar!(&$name, &i64, $name);
    };
}

impl_vec!(Vector3i, i64);
impl Hash for Vector3i {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}
