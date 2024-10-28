use std::ops::*;


mod bezier;


pub use bezier::curve::Curve1;
pub use bezier::curve::Curve2;
pub use bezier::curve::Curve3;
pub use bezier::curve::Curve4;
pub use bezier::surface::Surface2;
pub use bezier::surface::Surface3;


type Dif<T, U = T> = <T as Sub<U>>::Output;
type Quot<T, U = T> = <T as Div<U>>::Output;
type Prod<T, U = T> = <T as Mul<U>>::Output;
type Sum<T, U = T> = <T as Add<U>>::Output;




pub fn lerp<T: Copy+Sub+Mul<Dif<T>>>(t: T, a: T, b: T) -> Sum<Prod<T, Dif<T>>, T>
where
    Prod<T, Dif<T>>: Add<T>
{
    t * (b-a) + a
}

pub fn unlerp<T: Copy+Sub>(t: T, a: T, b: T) -> Quot<Dif<T>>
where
    Dif<T>: Div,
{
    (t-a) / (b-a)
}


pub trait Unlerp: Copy {
    
    fn unlerp(self, a: Self, b: Self) -> Self;
}

impl Unlerp for f32 { 
                
    fn unlerp(self, a: Self, b: Self) -> Self { unlerp(self, a, b) } 
}

impl Unlerp for f64 { 
                
    fn unlerp(self, a: Self, b: Self) -> Self { unlerp(self, a, b) } 
}

pub trait BezierT: Sized+Copy {
    fn pow(self, pow: i32) -> Self;
    fn comp(self) -> Self;
    fn prod(self, coeff: i32) -> Self;
    fn mul(self, other: Self) -> Self;
    fn coeff(self, comp: i32, pow: i32, coeff: i32) -> Self {
        self.comp().pow(comp).mul(self.pow(pow)).prod(coeff)
    }
}

impl BezierT for f32 {
    fn pow(self, pow: i32) -> Self { self.powi(pow) }
    fn comp(self) -> Self { 1. - self }
    fn prod(self, coeff: i32) -> Self { self * coeff as Self }
    fn mul(self, other: Self) -> Self { self * other }
}

impl BezierT for f64 {
    fn pow(self, pow: i32) -> Self { self.powi(pow) }
    fn comp(self) -> Self { 1. - self }
    fn prod(self, coeff: i32) -> Self { self * coeff as Self }
    fn mul(self, other: Self) -> Self { self * other }
}

pub trait LinearCombination<S>: Sized {
    fn linear_combination<const N: usize>(terms: [(Self, S); N]) -> Self;
}

impl LinearCombination<Self> for f32 {
    fn linear_combination<const N: usize>(terms: [(Self, Self); N]) -> Self {
        terms.into_iter().fold(0., |c, n| c + n.1 * n.0)
    }
}

impl LinearCombination<Self> for f64 {
    fn linear_combination<const N: usize>(terms: [(Self, Self); N]) -> Self {
        terms.into_iter().fold(0., |c, n| c + n.1 * n.0)
    }
}

pub trait Bezier<V>: Copy+Sized {
    fn lerp(self, a: V, b: V) -> V;
    fn bezier2(self, a: V, h: V, b: V) -> V;
    fn bezier3(self, a: V, h0: V, h1: V, b: V) -> V;
    fn bezier4(self, a: V, h0: V, h1: V, h2: V, b: V) -> V;
}

impl<T: Copy+BezierT, V: LinearCombination<T>> Bezier<V> for T {
    fn lerp(self, a: V, b: V) -> V {
        V::linear_combination([(a, self.comp()), (b, self)])
    }

    fn bezier2(self, a: V, h: V, b: V) -> V {
        V::linear_combination([(a, self.comp().pow(2)), (h, self.coeff(1, 1, 2)), (b, self.pow(2))])
    }

    fn bezier3(self, a: V, h0: V, h1: V, b: V) -> V {
        V::linear_combination([(a, self.comp().pow(3)), (h0, self.coeff(2, 1, 3)), (h1, self.coeff(1, 2, 3)), (b, self.pow(3))])
    }

    fn bezier4(self, a: V, h0: V, h1: V, h2: V, b: V) -> V {
        V::linear_combination([(a, self.comp().pow(4)), (h0, self.coeff(3, 1, 4)), (h1, self.coeff(2, 2, 6)), (h2, self.coeff(1, 3, 4)), (b, self.pow(4))])
    }
}

