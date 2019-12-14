#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vector3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x,y,z}
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0,0,0)
    }

    pub fn add(&self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}


#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 {x,y}
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0,0)
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }

    pub fn scale(&self, scalar: i32) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}