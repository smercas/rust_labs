fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}
impl Complex {
    fn new<X: Into<f64>, Y: Into<f64>>(real: X, imag: Y) -> Complex {
        Complex {
            real: real.into(),
            imag: imag.into(),
        }
    }
    fn conjugate(self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}
impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.real != 0.0 && self.imag != 0.0 {
            write!(f, "{}{:+}i", self.real, self.imag)
        } else if self.real == 0.0 && self.imag != 0.0 {
            write!(f, "{}i", self.imag)
        } else if self.real != 0.0 && self.imag == 0.0 {
            write!(f, "{}", self.real)
        } else {
            write!(f, "0")
        }
    }
}
impl std::ops::Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl<X: Into<Complex>> std::ops::Add<X> for Complex {
    type Output = Complex;
    fn add(self, rhs: X) -> Self::Output {
        let rrhs = rhs.into();
        Complex {
            real: self.real + rrhs.real,
            imag: self.imag + rrhs.imag,
        }
    }
}
impl<X: Into<Complex>> std::ops::Sub<X> for Complex {
    type Output = Complex;
    fn sub(self, rhs: X) -> Self::Output {
        let rrhs = rhs.into();
        Complex {
            real: self.real - rrhs.real,
            imag: self.imag - rrhs.imag,
        }
    }
}
impl<X: Into<Complex>> std::ops::Mul<X> for Complex {
    type Output = Complex;
    fn mul(self, rhs: X) -> Self::Output {
        let rrhs = rhs.into();
        Complex {
            real: self.real * rrhs.real - self.imag * rrhs.imag,
            imag: self.real * rrhs.imag + self.imag * rrhs.real,
        }
    }
}
impl<X: Into<Complex>> std::ops::Div<X> for Complex {
    type Output = Complex;
    fn div(self, rhs: X) -> Self::Output {
        let rrhs = rhs.into();
        if rrhs.real == 0.0 && rrhs.imag == 0.0 {
            panic!("Division by zero");
        }
        Complex {
            real: (self.real * rrhs.real + self.imag * rrhs.imag) / (rrhs.real * rrhs.real + rrhs.imag * rrhs.imag),
            imag: (self.real * rrhs.imag + self.imag * rrhs.real) / (rrhs.real * rrhs.real + rrhs.imag * rrhs.imag),
        }
    }
}

impl<X: Into<Complex>> std::ops::AddAssign<X> for Complex {
    fn add_assign(&mut self, rhs: X) {
        let rrhs = rhs.into();
        self.real += rrhs.real;
        self.imag += rrhs.imag;
        // *self = *self + rhs.into();
    }
}
impl<X: Into<Complex>> std::ops::SubAssign<X> for Complex {
    fn sub_assign(&mut self, rhs: X) {
        let rrhs = rhs.into();
        self.real -= rrhs.real;
        self.imag -= rrhs.imag;
        // *self = *self - rhs.into();
    }
}
impl<X: Into<Complex>> std::ops::MulAssign<X> for Complex {
    fn mul_assign(&mut self, rhs: X) {
        *self = *self * rhs.into();
    }
}
impl<X: Into<Complex>> std::ops::DivAssign<X> for Complex {
    fn div_assign(&mut self, rhs: X) {
        *self = *self / rhs.into();
    }
}

// impl<X: Into<(f64, f64)>> From<X> for Complex {
//     fn from(value: X) -> Self {
//         Complex {
//             real: value.into().0,
//             imag: value.into().1,
//         }
//     }
// }

/*
    how can I implement this?
*/

// impl<X: Into<f64>> From<X> for Complex {
//     fn from(value: X) -> Self {
//         Complex {
//             real: value.into(),
//             imag: 0.0,
//         }
//     }
// }
impl<T> From<T> for Complex where f64: From<T> {
    fn from(value: T) -> Self {
        Complex {
            real: value.into(),
            imag: 0,
        }
    }
}
impl From<(f64,f64)> for Complex {
    fn from(value: (f64,f64)) -> Self {
        Complex {
            real: value.0,
            imag: value.1
        }
    }
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b/* + (5, -4)*/;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
