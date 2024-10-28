use crate::BezierT;

use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct Curve4<V> {
    a: V,
    h0: V,
    h1: V,
    h2: V,
    b: V
}

impl<V> Curve4<V> {
    pub const fn new(a: V, h0: V, h1: V, h2: V, b: V) -> Self {
        Self {a, h0, h1, h2, b}
    }

    pub fn sample<T: BezierT>(self, t: T) -> V where V: LinearCombination<T>{
        t.bezier4(self.a, self.h0, self.h1, self.h2, self.b)
    }

    pub fn ddt(self) -> Curve3<V> where V: Sub<Output = V> + Add<Output = V> + Copy {
        let a = self.h0 - self.a;
        let h0 = self.h1 - self.h0;
        let h1 = self.h2 - self.h1;
        let b = self.b - self.h2;
        Curve3::new(a+a+a+a, h0+h0+h0+h0, h1+h1+h1+h1, b+b+b+b)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Curve3<V> {
    a: V,
    h0: V,
    h1: V,
    b: V
}

impl<V> Curve3<V> {
    pub const fn new(a: V, h0: V, h1: V, b: V) -> Self {
        Self {a, h0, h1, b}
    }

    pub fn sample<T: BezierT>(self, t: T) -> V where V: LinearCombination<T>{
        t.bezier3(self.a, self.h0, self.h1, self.b)
    }

    pub fn ddt(self) -> Curve2<V> where V: Sub<Output = V> + Add<Output = V> + Copy {
        let a = self.h0 - self.a;
        let h = self.h1 - self.h0;
        let b = self.b - self.h1;
        Curve2::new(a+a+a, h+h+h, b+b+b)
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Curve2<V> {
    a: V,
    h: V,
    b: V
}

impl<V> Curve2<V> {
    pub const fn new(a: V, h: V, b: V) -> Self {
        Self {a, h, b}
    }

    pub fn sample<T: BezierT>(self, t: T) -> V where V: LinearCombination<T>{
        t.bezier2(self.a, self.h, self.b)
    }

    pub fn ddt(self) -> Curve1<V> where V: Sub<Output = V> + Add<Output = V> + Copy {
        let a = self.h - self.a;
        let b = self.b - self.h;
        Curve1::new(a+a, b+b)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Curve1<V> {
    a: V,
    b: V
}

impl<V> Curve1<V> {
    pub const fn new(a: V, b: V) -> Self {
        Self{a, b}
    }

    pub fn sample<T: BezierT>(self, t: T) -> V where V: LinearCombination<T> {
        t.lerp(self.a, self.b)
    }
}