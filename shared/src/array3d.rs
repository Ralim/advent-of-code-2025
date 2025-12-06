use zarray::z3d::ZArray3D;

pub fn new_3d_array<T: Copy>(width: usize, height: usize, depth: usize, value: T) -> ZArray3D<T> {
    ZArray3D::new(width, height, depth, value)
}
