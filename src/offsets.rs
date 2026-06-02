pub fn get_offsets() -> Offsets {
    Offsets{
        fPlayTimeCleared: [0x525CAF8 ,0x8],
            }
}

pub(crate) struct Offsets {
    pub fPlayTimeCleared: [u64; 2],
}