use amethyst::core::math::{Matrix2, Point3, Vector2};

pub const HEX_SIZE: f32 = 60.0;

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HexCoord {
    pub q: i16,
    pub r: i16,
}

impl HexCoord {
    pub fn new(q: i16, r: i16) -> Self {
        HexCoord { q, r }
    }

    pub fn is_adjacent(&self, other: &Self) -> bool {
        let q_diff = self.q - other.q;
        let r_diff = self.r - other.r;

        match (q_diff, r_diff) {
            (1, 0) | (-1, 0) | (0, 1) | (0, -1) | (1, -1) | (-1, 1) => true,
            _ => false,
        }
    }

    pub fn world_coords(&self) -> (f32, f32) {
        let hex_vec = Vector2::new(self.q as f32, self.r as f32);
        let matrix = Matrix2::new(3. / 2., 0., (3_f32).sqrt() / 2., (3_f32).sqrt());
        let world_vec = HEX_SIZE * matrix * hex_vec;
        (world_vec.x, world_vec.y)
    }
}

impl From<HexCoord> for Vector2<f32> {
    fn from(hex: HexCoord) -> Vector2<f32> {
        let hex_vec = Vector2::new(hex.q as f32, hex.r as f32);
        let matrix = Matrix2::new(3. / 2., 0., (3_f32).sqrt() / 2., (3_f32).sqrt());
        HEX_SIZE * matrix * hex_vec
    }
}

impl From<(f32, f32)> for HexCoord {
    fn from((x, y): (f32, f32)) -> HexCoord {
        let v = Vector2::new(x, y);
        v.into()
    }
}

impl From<Vector2<f32>> for HexCoord {
    fn from(v: Vector2<f32>) -> HexCoord {
        let matrix = Matrix2::new(2. / 3., 0., -1. / 3., (3_f32).sqrt() / 3.);
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
