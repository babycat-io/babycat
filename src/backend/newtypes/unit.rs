use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit<T>(pub T);

impl<T: fmt::Display> fmt::Display for Unit<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Unit<T> {
    pub fn into_bare(self) -> T {
        self.0
    }
}

impl<T> From<T> for Unit<T> {
    fn from(val: T) -> Unit<T> {
        Unit(val)
    }
}

impl<T: PartialEq> PartialEq<T> for Unit<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: ops::Add<Output = T>> ops::Add<Unit<T>> for Unit<T> {
    type Output = Unit<T>;

    fn add(self, rhs: Unit<T>) -> Self::Output {
        Unit(self.into_bare() + rhs.into_bare())
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub<Unit<T>> for Unit<T> {
    type Output = Unit<T>;

    fn sub(self, rhs: Unit<T>) -> Self::Output {
        Unit(self.into_bare() - rhs.into_bare())
    }
}

impl<T: Into<f64>> ops::Div<Unit<T>> for Unit<T> {
    type Output = f64;

    fn div(self, rhs: Unit<T>) -> Self::Output {
        let lhs: f64 = self.into_bare().into();
        let rhs: f64 = rhs.into_bare().into();
        lhs / rhs
    }
}

impl<T: Into<f64>> ops::Mul<f64> for Unit<T> {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs: f64 = self.into_bare().into();
        lhs * rhs
    }
}
