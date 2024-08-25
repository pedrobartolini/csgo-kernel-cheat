use std::f32::consts::PI;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec {
   pub x: f32,
   pub y: f32,
   pub z: f32
}

impl Vec {
   pub fn new(x: f32, y: f32, z: f32) -> Self {
      Self { x, y, z }
   }

   pub const fn new_const(x: f32, y: f32, z: f32) -> Self {
      Self { x, y, z }
   }

   pub fn is_zeroed(&self) -> bool {
      self.x == 0.0 && self.y == 0.0 && self.z == 0.0
   }

   pub fn length(&self) -> f32 {
      (self.x * self.x + self.y * self.y).sqrt()
   }

   pub fn normalize_angles(&mut self) {
      self.x = self.x % 360.0;
      self.y = self.y % 360.0;
      self.z = self.z % 360.0;

      if self.x < -180.0 {
         self.x += 360.0;
      } else if self.x > 180.0 {
         self.x -= 360.0;
      }

      if self.y < -180.0 {
         self.y += 360.0;
      } else if self.y > 180.0 {
         self.y -= 360.0;
      }

      if self.z < -180.0 {
         self.z += 360.0;
      } else if self.z > 180.0 {
         self.z -= 360.0;
      }
   }

   pub fn filter(&mut self, y_speed: f32, x_speed: f32) {
      self.x = self.x.clamp(-x_speed, x_speed);
      self.y = self.y.clamp(-y_speed, y_speed);
   }

   pub fn to_angle(&mut self) -> Self {
      const DEG_TO_RAD: f32 = 180.0 / PI;
      let xyp = (self.x.powi(2) + self.y.powi(2)).sqrt();
      Self {
         x: (-self.z).atan2(xyp) * DEG_TO_RAD,
         y: self.y.atan2(self.x) * DEG_TO_RAD,
         z: 0.0
      }
   }
}

impl Add for Vec {
   type Output = Self;

   fn add(self, other: Self) -> Self::Output {
      Self {
         x: self.x + other.x,
         y: self.y + other.y,
         z: self.z + other.z
      }
   }
}

impl Sub for Vec {
   type Output = Self;

   fn sub(self, other: Self) -> Self::Output {
      Self {
         x: self.x - other.x,
         y: self.y - other.y,
         z: self.z - other.z
      }
   }
}

impl Mul for Vec {
   type Output = Self;

   fn mul(self, other: Self) -> Self::Output {
      Self {
         x: self.x * other.x,
         y: self.y * other.y,
         z: self.z * other.z
      }
   }
}

impl Div for Vec {
   type Output = Self;

   fn div(self, other: Self) -> Self::Output {
      Self {
         x: self.x / other.x,
         y: self.y / other.y,
         z: self.z / other.z
      }
   }
}

impl Neg for Vec {
   type Output = Self;

   fn neg(self) -> Self::Output {
      Self {
         x: -self.x,
         y: -self.y,
         z: -self.z
      }
   }
}

pub fn calculate_angle(local_pos: &Vec, enemy_pos: &Vec, view_angles: &Vec) -> Vec {
   return (*enemy_pos - *local_pos).to_angle() - *view_angles;
}
