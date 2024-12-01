use std::hash::{Hash, Hasher};

const LSB5_MASK: u16 = 0b11111;

/// 3 u5's condensed into 1 u16.
/// Stored as XYZ where Z is the 5 LSBs.
#[derive(Debug, Clone, Copy)]
pub struct CondensedCoordinate(pub u16);
/// xyz
#[derive(Debug, Clone, Copy)]
pub struct Coordinate(pub u8, pub u8, pub u8);

impl From<Coordinate> for CondensedCoordinate {
    fn from(c: Coordinate) -> Self {
        if c.0 > 31 || c.1 > 31 || c.2 > 31 {
            panic!("Coordinates cannot be more than 31 due to u5 magic");
        }

        let mut v: u16 = c.0 as u16;
        v = (v << 5) | c.1 as u16;
        v = (v << 5) | c.2 as u16;

        CondensedCoordinate(v)
    }
}

impl From<CondensedCoordinate> for Coordinate {
    fn from(c: CondensedCoordinate) -> Self {
        let z = (c.0 & LSB5_MASK) as u8;
        let y = ((c.0 >> 5) & LSB5_MASK) as u8;
        let x = ((c.0 >> 10) & LSB5_MASK) as u8;

        Coordinate(x, y, z)
    }
}

impl From<(u8, u8, u8)> for Coordinate{
    fn from(c: (u8, u8, u8)) -> Self {
        Coordinate(c.0, c.1, c.2)
    }
}

impl PartialEq for CondensedCoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CondensedCoordinate {}

impl Hash for CondensedCoordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Coordinate {}

impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
        self.2.hash(state);
    }
}

impl Coordinate {
    // Manhattan distance
    pub fn shares_side(&self, other: &Coordinate) -> bool {
        let dx = (self.0 as i16 - other.0 as i16).abs();
        let dy = (self.1 as i16 - other.1 as i16).abs();
        let dz = (self.2 as i16 - other.2 as i16).abs();
        (dx + dy + dz) == 1
    }
}

#[test]
fn test_coordinate() {
    let coordinate = Coordinate::from((2, 1, 5));
    assert_eq!(coordinate.0, 2);
    assert_eq!(coordinate.1, 1);
    assert_eq!(coordinate.2, 5);

    let condensed = CondensedCoordinate::from(coordinate);
    assert_eq!(condensed.0, 0b0000_1000_0010_0101);

    let coordinate = Coordinate::from(condensed);
    assert_eq!(coordinate.0, 2);
    assert_eq!(coordinate.1, 1);
    assert_eq!(coordinate.2, 5);
}
