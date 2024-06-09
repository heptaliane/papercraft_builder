use std::convert::{From, Into};

pub type CoordUnit = f32;

pub type Coord = [CoordUnit; 3];

pub type Face = Vec<Coord>;

#[derive(Clone, PartialOrd, PartialEq, Debug)]
pub struct PolarCoord {
    r: CoordUnit,
    theta: CoordUnit,
    phi: CoordUnit,
}

impl From<Coord> for PolarCoord {
    fn from(coord: Coord) -> Self {
        let r = coord.iter().fold(0., |acc, v| acc + v * v).sqrt();
        let inv_r = 1. / r;
        let theta = match inv_r.is_finite() {
            true => ((coord[2]) * inv_r).acos(),
            _ => 0.,
        };
        let norm_xy = 1. / (r * theta.sin());
        let phi = match norm_xy.is_finite() {
            true => (coord[1] * norm_xy).atan2(coord[0] * norm_xy),
            _ => 0.,
        };

        PolarCoord { r, theta, phi }
    }
}

impl Into<Coord> for PolarCoord {
    fn into(self) -> Coord {
        [
            self.r * self.theta.sin() * self.phi.cos(),
            self.r * self.theta.sin() * self.phi.sin(),
            self.r * self.theta.cos(),
        ]
    }
}

#[test]
fn test_polar_coord_from() {
    use approx::assert_relative_eq;
    let coords: Vec<Coord> = vec![
        [0., 0., 0.],
        [1., 0., 0.],
        [1., 1., 1.],
        [-1., 1., 1.],
        [-1., 1., -1.],
        [-1., -1., -1.],
    ];
    coords.iter().for_each(|&coord| {
        let polar = PolarCoord::from(coord);
        let actual_coord: Coord = polar.into();
        coord.iter().zip(actual_coord).for_each(|(&expected, actual)| {
            assert_relative_eq!(expected, actual);
        });
    });
}
