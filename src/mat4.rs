use crate::vec4::Vec4;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Mat4 {
    elements: [f32; 16],
}

#[allow(dead_code)]
impl Mat4 {
    pub fn zeroes() -> Self {
        Self {
            elements: [0.0; 16]
        }
    }

    pub fn identity() -> Self {
        let mut elements = [0.0; 16];
        elements[0] = 1.0;
        elements[5] = 1.0;
        elements[9] = 1.0;
        elements[13] = 1.0;
        Self {
            elements
        }
    }
    pub fn x_vector(&self) -> Vec4 {
        Vec4 {
            x: self.elements[0],
            y: self.elements[1],
            z: self.elements[2],
            w: self.elements[3]
        }
    }

    pub fn y_vector(&self) -> Vec4 {
        Vec4 {
            x: self.elements[4],
            y: self.elements[5],
            z: self.elements[6],
            w: self.elements[7]
        }
    }

    pub fn z_vector(&self) -> Vec4 {
        Vec4 {
            x: self.elements[8],
            y: self.elements[9],
            z: self.elements[10],
            w: self.elements[11]
        }
    }

    pub fn position(&self) -> Vec4 {
        Vec4 {
            x: self.elements[12],
            y: self.elements[13],
            z: self.elements[14],
            w: self.elements[15]
        }
    }

    pub fn transpose(&self) -> Self {
        let mut elements = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                elements[i * 4 + j] = self.elements[j * 4 + i];
            }
        }
        Self {
            elements
        }
    }

    pub fn translate(&self, translation: &Vec4) -> Self {
        let mut elements = self.elements;
        elements[12] += translation.x;
        elements[13] += translation.y;
        elements[14] += translation.z;
        Self {
            elements
        }
    }

    pub fn translate_local(&self, translation: &Vec4) -> Self {
        let mut elements = self.elements;
        elements[12] += self.elements[0] * translation.x + self.elements[4] * translation.y + self.elements[8] * translation.z;
        elements[13] += self.elements[1] * translation.x + self.elements[5] * translation.y + self.elements[9] * translation.z;
        elements[14] += self.elements[2] * translation.x + self.elements[6] * translation.y + self.elements[10] * translation.z;
        Self {
            elements
        }
    }

    pub fn scale(&self, scale: Vec4) -> Self {
        let mut elements = self.elements;
        elements[0] *= scale.x;
        elements[5] *= scale.y;
        elements[10] *= scale.z;
        Self {
            elements
        }
    }

    fn rodrigues(&self, axis: Vec4, angle: f32) -> Self {
        let mut elements = [0.0; 16];
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        elements[0] = t * x * x + c;
        elements[1] = t * x * y - s * z;
        elements[2] = t * x * z + s * y;
        elements[3] = 0.0;
        elements[4] = t * x * y + s * z;
        elements[5] = t * y * y + c;
        elements[6] = t * y * z - s * x;
        elements[7] = 0.0;
        elements[8] = t * x * z - s * y;
        elements[9] = t * y * z + s * x;
        elements[10] = t * z * z + c;
        elements[11] = 0.0;
        elements[12] = 0.0;
        elements[13] = 0.0;
        elements[14] = 0.0;
        elements[15] = 1.0;
        Self {
            elements
        }
    }

    pub fn rotate(&mut self, axis: Vec4, angle: f32) {
        *self *= self.rodrigues(axis, angle);
    }

    pub fn rotate_local(&mut self, axis: Vec4, angle: f32) {
        *self = self.rodrigues(axis, angle) * *self;
    }


    pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let mut elements = [0.0; 16];
        elements[0] = f / aspect_ratio;
        elements[5] = f;
        elements[10] = (far + near) / (near - far);
        elements[11] = -1.0;
        elements[14] = (2.0 * far * near) / (near - far);
        Self {
            elements
        }
    }
    pub fn view(position: Vec4, forward: Vec4, up: Vec4) -> Self {
        let right = forward.cross(&up).normalize();
        let up = right.cross(&forward).normalize();
        let forward = forward.normalize();
        let mut elements = [0.0; 16];
        elements[0] = right.x;
        elements[4] = right.y;
        elements[8] = right.z;
        elements[1] = up.x;
        elements[5] = up.y;
        elements[9] = up.z;
        elements[2] = -forward.x;
        elements[6] = -forward.y;
        elements[10] = -forward.z;
        elements[12] = -right.dot(&position);
        elements[13] = -up.dot(&position);
        elements[14] = forward.dot(&position);
        elements[15] = 1.0;
        Self {
            elements
        }
    }
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut elements = [0.0; 16];
        elements[0] = 2.0 / (right - left);
        elements[5] = 2.0 / (top - bottom);
        elements[10] = -2.0 / (far - near);
        elements[12] = -(right + left) / (right - left);
        elements[13] = -(top + bottom) / (top - bottom);
        elements[14] = -(far + near) / (far - near);
        elements[15] = 1.0;
        Self {
            elements
        }
    }

}

impl std::ops::MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        let mut result = [0.0; 16];
        for col in 0..4 { // Iterate over columns of the result
            for row in 0..4 { // Iterate over rows of the result
                for k in 0..4 { // Accumulate the dot product
                    result[col * 4 + row] += self.elements[k * 4 + row] * rhs.elements[col * 4 + k];
                }
            }
        }
        self.elements = result;
    }
}


impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;
    
    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut out = self.clone();
        out *= rhs;
        out 
    }
}

impl std::ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;
    
    fn mul(self, rhs: Vec4) -> Self::Output {
        let x = self.elements[0] * rhs.x + self.elements[4] * rhs.y + self.elements[8] * rhs.z + self.elements[12] * rhs.w;
        let y = self.elements[1] * rhs.x + self.elements[5] * rhs.y + self.elements[9] * rhs.z + self.elements[13] * rhs.w;
        let z = self.elements[2] * rhs.x + self.elements[6] * rhs.y + self.elements[10] * rhs.z + self.elements[14] * rhs.w;
        let w = self.elements[3] * rhs.x + self.elements[7] * rhs.y + self.elements[11] * rhs.z + self.elements[15] * rhs.w;
        Vec4 { x, y, z, w }
    }
}

impl std::ops::MulAssign<f32> for Mat4 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..16 {
            self.elements[i] *= rhs;
        }
    }
}


impl std::ops::Mul<f32> for Mat4 {
    type Output = Mat4;
    
    fn mul(self, rhs: f32) -> Self::Output {
        let mut out = self.clone();
        out *= rhs;
        out 
    }
    
}

impl std::ops::AddAssign<Mat4> for Mat4 {
    fn add_assign(&mut self, rhs: Mat4) {
        for i in 0..16 {
            self.elements[i] += rhs.elements[i];
        }
    }
}

impl std::ops::Add<Mat4> for Mat4 {    
    type Output = Self;
    
    fn add(self, rhs: Mat4) -> Self::Output {
        let mut out = self.clone();
        out += rhs;
        out 
    }
}

impl std::ops::SubAssign<Mat4> for Mat4 {

    fn sub_assign(&mut self, rhs: Mat4) {
        for i in 0..16 {
            self.elements[i] -= rhs.elements[i];
        }
    }
}

impl std::ops::Sub<Mat4> for Mat4 {
    type Output = Self;

    fn sub(self, rhs: Mat4) -> Self::Output {
        let mut out = self.clone();
        out -= rhs;
        out 
    }
}

impl std::fmt::Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]", 
            self.elements[0], self.elements[4], self.elements[8], self.elements[12],
            self.elements[1], self.elements[5], self.elements[9], self.elements[13],
            self.elements[2], self.elements[6], self.elements[10], self.elements[14],
            self.elements[3], self.elements[7], self.elements[11], self.elements[15]
        )
    }
}
