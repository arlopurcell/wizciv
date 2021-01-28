use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::core::math::{Vector2, Point3, Matrix2};

pub const HEX_SIZE: f32 = 60.0;

#[derive(PartialEq, Eq)]
pub struct HexCoord {
    pub q: i16,
    pub r: i16,
}

impl HexCoord {
    pub fn new(q: i16, r: i16) -> Self {
        HexCoord{q, r}
    }
}

impl From<HexCoord> for Vector2<f32> {
    fn from(hex: HexCoord) -> Vector2<f32> {
        let hex_vec = Vector2::new(hex.q as f32, hex.r as f32);
        let matrix = Matrix2::new(3./2., 0.,
                                  (3_f32).sqrt()/2., (3_f32).sqrt());
        HEX_SIZE * matrix * hex_vec
    }
}

impl From<Vector2<f32>> for HexCoord {
    fn from(v: Vector2<f32>) -> HexCoord {
        let matrix = Matrix2::new(2./3., 0.,
                                  -1./3., (3_f32).sqrt()/3.);
        let hex_vec = matrix * v / HEX_SIZE;
        HexCoord::new(hex_vec.x.round() as i16, hex_vec.y.round() as i16)
    }
}

impl From<Point3<f32>> for HexCoord {
    fn from(v: Point3<f32>) -> HexCoord {
        let v = Vector2::new(v.x, v.y);
        v.into()
    }
}

impl Component for HexCoord {
    type Storage = DenseVecStorage<Self>;
}