#[derive(Copy, Clone)]
pub struct BitIter {
    pub bb: u16,
}

impl Iterator for BitIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.bb == 0 {
            return None;
        }
        let lsb = self.bb & self.bb.wrapping_neg();
        let sq = lsb.trailing_zeros() as u8;
        self.bb ^= lsb;
        Some(sq)
    }
}
