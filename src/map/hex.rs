const SQRT_3: f32 = 1.73205080757;

pub struct Hex;

impl Hex {
    pub fn get_size(size: f32) -> (f32, f32) {
        let width = SQRT_3 * size;
        let height = 2.0 * size;

        (width, height)
    }
}

pub struct HexCoords {
    pub q: isize,
    pub r: isize,
}

impl HexCoords {
    pub fn new(q: isize, r: isize) -> Self {
        Self { q, r }
    }

    pub fn get_s_coord(&self) -> isize {
        -self.q - self.r
    }

    pub fn neighbour(&self, dir: Direction) -> Self {
        use Direction::*;
        match dir {
            West => Self::new(self.q - 1, self.r),
            NorthWest => Self::new(self.q, self.r + 1),
            NorthEast => Self::new(self.q + 1, self.r - 1),
            East => Self::new(self.q + 1, self.r),
            SouthEast => Self::new(self.q, self.r + 1),
            SouthWest => Self::new(self.q - 1, self.r + 1),
        }
    }
}

pub fn get_distance(hex1: HexCoords, hex2: HexCoords) -> isize {
    max(
        abs(hex1.q - hex2.q),
        abs(hex1.r - hex2.r),
        abs(hex1.get_s_coord() - hex2.get_s_coord()),
    )
}

pub enum Direction {
    West,
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
}

fn max<T: PartialOrd>(a: T, b: T, c: T) -> T {
    if a > b {
        if a > c {
            a
        } else {
            c
        }
    } else {
        if b > c {
            b
        } else {
            c
        }
    }
}

fn abs(a: isize) -> isize {
    if a < 0 {
        -a
    } else {
        a
    }
}
