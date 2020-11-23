pub fn merge<I1, I2>(a: I1, b: I2) -> Merge<I1::Item, I1::IntoIter, I2::IntoIter>
where
    I1: IntoIterator,
    I2: IntoIterator<Item = I1::Item>,
{
    Merge {
        a: a.into_iter(),
        b: b.into_iter(),
        va: None,
        vb: None,
    }
}
pub struct Merge<T, I1, I2> {
    a: I1,
    b: I2,
    va: Option<Option<T>>,
    vb: Option<Option<T>>,
}
impl<T: Ord, I1: Iterator<Item = T>, I2: Iterator<Item = T>> Iterator for Merge<T, I1, I2> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let ta = self.va.take().unwrap_or_else(|| self.a.next());
        let tb = self.vb.take().unwrap_or_else(|| self.b.next());
        match (&ta, &tb) {
            (Some(xa), Some(xb)) => {
                if xa <= xb {
                    self.vb = Some(tb);
                    ta
                } else {
                    self.va = Some(ta);
                    tb
                }
            }
            (Some(_), _) => ta,
            (_, Some(_)) => tb,
            _ => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (la, ua) = self.a.size_hint();
        let (lb, ub) = self.b.size_hint();
        (
            la.saturating_add(lb),
            // u0.zip(u1).map(|(u0, u1)| u0.saturating_add(u1)),
            if let (Some(u0), Some(u1)) = (ua, ub) {
                Some(u0.saturating_add(u1))
            } else {
                None
            },
        )
    }
}
