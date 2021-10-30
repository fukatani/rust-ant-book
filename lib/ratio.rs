#[derive(Copy, Clone)]
pub struct Ratio {
    /// Numerator.
    numer: i64,
    /// Denominator.
    denom: i64,
}

// These method are `const` for Rust 1.31 and later.
impl Ratio {
    pub const fn new_raw(numer: i64, denom: i64) -> Ratio {
        Ratio { numer, denom }
    }
}

impl Ord for Ratio {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.denom == 0 && other.denom == 0 {
            return std::cmp::Ordering::Equal;
        }
        if self.denom == 0 {
            return std::cmp::Ordering::Greater;
        }
        if other.denom == 0 {
            return std::cmp::Ordering::Less;
        }
        return (self.numer * other.denom).cmp(&(self.denom * other.numer));
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Ratio {}
