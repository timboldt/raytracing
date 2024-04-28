pub type Point3 = Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
}


//     vec3 operator-() const { return vec3(-e[0], -e[1], -e[2]); }
//     double operator[](int i) const { return e[i]; }
//     double& operator[](int i) { return e[i]; }

//     vec3& operator+=(const vec3& v) {
//         e[0] += v.e[0];
//         e[1] += v.e[1];
//         e[2] += v.e[2];
//         return *this;
//     }

//     vec3& operator*=(double t) {
//         e[0] *= t;
//         e[1] *= t;
//         e[2] *= t;
//         return *this;
//     }

//     vec3& operator/=(double t) {
//         return *this *= 1/t;
//     }

//     double length() const {
//         return sqrt(length_squared());
//     }

//     double length_squared() const {
//         return e[0]*e[0] + e[1]*e[1] + e[2]*e[2];
//     }
// };

// // point3 is just an alias for vec3, but useful for geometric clarity in the code.
// using point3 = vec3;

// // Vector Utility Functions

// inline std::ostream& operator<<(std::ostream& out, const vec3& v) {
//     return out << v.e[0] << ' ' << v.e[1] << ' ' << v.e[2];
// }

// inline vec3 operator+(const vec3& u, const vec3& v) {
//     return vec3(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2]);
// }

// inline vec3 operator-(const vec3& u, const vec3& v) {
//     return vec3(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2]);
// }

// inline vec3 operator*(const vec3& u, const vec3& v) {
//     return vec3(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2]);
// }

// inline vec3 operator*(double t, const vec3& v) {
//     return vec3(t*v.e[0], t*v.e[1], t*v.e[2]);
// }

// inline vec3 operator*(const vec3& v, double t) {
//     return t * v;
// }

// inline vec3 operator/(const vec3& v, double t) {
//     return (1/t) * v;
// }

// inline double dot(const vec3& u, const vec3& v) {
//     return u.e[0] * v.e[0]
//          + u.e[1] * v.e[1]
//          + u.e[2] * v.e[2];
// }

// inline vec3 cross(const vec3& u, const vec3& v) {
//     return vec3(u.e[1] * v.e[2] - u.e[2] * v.e[1],
//                 u.e[2] * v.e[0] - u.e[0] * v.e[2],
//                 u.e[0] * v.e[1] - u.e[1] * v.e[0]);
// }

// inline vec3 unit_vector(const vec3& v) {
//     return v / v.length();
// }
