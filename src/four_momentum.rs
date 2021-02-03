use std::ops::{Add, Sub, Mul, Div};
use crate::three_vector::ThreeVector;




/**
 * Describes a relativistic four-momentum vector
 */
#[derive(Clone)]
pub struct FourMomentum(pub f64, pub f64, pub f64, pub f64);




// ============================================================================
impl FourMomentum {

    pub fn contract(&self, b: &FourMomentum) -> f64 {
        let p = self;
        -p.0 * b.0 + p.1 * b.1 + p.2 * b.2 + p.3 * b.3
    }

    pub fn rest_mass(&self) -> f64 {
        (-self.contract(self)).sqrt()
    }

    pub fn lorentz_factor(&self) -> f64 {
        let u = self.gamma_beta_vector().norm();
        u / (1.0 + u * u).sqrt()
    }

    pub fn gamma_beta_vector(&self) -> ThreeVector {
        ThreeVector(self.1, self.2, self.3) / self.rest_mass()
    }

    pub fn velocity_vector(&self) -> ThreeVector {
        self.gamma_beta_vector() / self.lorentz_factor()
    }
}




// ============================================================================
impl Add<FourMomentum> for FourMomentum {
    type Output = FourMomentum;
    fn add(self, b: FourMomentum) -> FourMomentum {
        FourMomentum(self.0 + b.0, self.1 + b.1, self.2 + b.2, self.3 + b.3)
    }
}

impl Sub<FourMomentum> for FourMomentum {
    type Output = FourMomentum;
    fn sub(self, b: FourMomentum) -> FourMomentum {
        FourMomentum(self.0 - b.0, self.1 - b.1, self.2 - b.2, self.3 - b.3)
    }
}

impl Mul<f64> for FourMomentum {
    type Output = FourMomentum;
    fn mul(self, b: f64) -> FourMomentum {
        FourMomentum(
            self.0 * b,
            self.1 * b,
            self.2 * b,
            self.3 * b,
        )
    }
}

impl Div<f64> for FourMomentum {
    type Output = FourMomentum;
    fn div(self, b: f64) -> FourMomentum {
        FourMomentum(
            self.0 / b,
            self.1 / b,
            self.2 / b,
            self.3 / b,
        )
    }
}
