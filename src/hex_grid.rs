use amethyst::core::math::{Matrix2, Vector2};

pub const HEX_SIZE: f32 = 60.0;

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HexCoord {
    pub q: i16,
    pub r: i16,
}

impl HexCoord {
    pub fn new(q: i16, r: i16, world_radius: i16) -> Option<Self> {
        if q <= world_radius && r <= (world_radius - q.max(0))  && q >= -world_radius && r >= (-world_radius - q.min(0)) {
            Some(HexCoord { q, r })
        } else {
            None
        }
    }

    pub fn from_xy(x: f32, y: f32, world_radius: i16) -> Option<Self> {
        HexCoord::from_vector(&Vector2::new(x, y), world_radius)
    }

    pub fn from_vector(v: &Vector2<f32>, world_radius: i16) -> Option<Self> {
        let matrix = Matrix2::new(2. / 3., 0., -1. / 3., (3_f32).sqrt() / 3.);
        let hex_vec = matrix * v / HEX_SIZE;
        HexCoord::new(hex_vec.x.round() as i16, hex_vec.y.round() as i16, world_radius)
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
