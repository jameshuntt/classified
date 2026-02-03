pub trait ClassifiedEq<Rhs = Self> {
    fn classified_eq(&self, rhs: &Rhs) -> bool;
}

#[allow(unused)]trait KeyType:Eq+std::hash::Hash+Clone+std::fmt::Debug{}
#[allow(unused)]trait ValueType:zeroize::Zeroize+Clone+std::fmt::Debug{}