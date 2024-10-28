use crate::*;


#[derive(Copy, Clone, Debug)]
pub struct Surface3<V> {
    a: Curve3<V>,
    h0: Curve3<V>,
    h1: Curve3<V>,
    b: Curve3<V>
}

impl<V> Surface3<V> {
    pub const fn new(a: Curve3<V>, h0: Curve3<V>, h1: Curve3<V>, b: Curve3<V>) -> Self {
        Self {a, h0, h1, b}
    }

    pub fn sample<T: BezierT>(self, t: T, u: T) -> V where V: LinearCombination<T>{
        u.bezier3(self.a.sample(t), self.h0.sample(t), self.h1.sample(t), self.b.sample(t))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Surface2<V> {
    a: Curve2<V>,
    h: Curve2<V>,
    b: Curve2<V>
}

impl<V> Surface2<V> {
    pub const fn new(a: Curve2<V>, h: Curve2<V>, b: Curve2<V>) -> Self {
        Self {a, h, b}
    }

    pub fn sample<T: BezierT>(self, t: T, u: T) -> V where V: LinearCombination<T>{
        u.bezier2(self.a.sample(t), self.h.sample(t), self.b.sample(t))
    }
}