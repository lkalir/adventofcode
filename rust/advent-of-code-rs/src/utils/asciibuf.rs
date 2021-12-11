pub const ASCII_D: u8 = 0x64;
pub const ASCII_F: u8 = 0x66;
pub const ASCII_U: u8 = 0x75;
pub const ASCII_0: u8 = 0x30;

#[allow(non_upper_case_globals)]
pub const ASCII_a: u8 = 0x61;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct AsciiBuf<const K: usize> {
    vals: [u8; K],
}

impl<const K: usize> AsciiBuf<K> {
    pub fn len(&self) -> usize {
        K
    }

    pub fn as_int(&self, idx: usize) -> u8 {
        self[idx] - ASCII_0
    }

    pub fn as_dec_ascii<T>(&self) -> T
    where
        T: From<u8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
    {
        self.vals
            .iter()
            .fold(0.into(), |ret, i| ret * 10.into() + (i - ASCII_0).into())
    }

    pub fn as_bin_ascii<T>(&self) -> T
    where
        T: From<u8>
            + std::ops::Mul<T, Output = T>
            + std::ops::Shl<usize, Output = T>
            + std::ops::BitOr<T, Output = T>,
    {
        self.vals
            .iter()
            .enumerate()
            .fold(0.into(), |ret, (idx, i)| {
                ret | (T::from(i - ASCII_0) << (K - 1 - idx))
            })
    }
}

impl<const K: usize> std::ops::Index<usize> for AsciiBuf<K> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vals[index]
    }
}

impl<const K: usize> std::fmt::Debug for AsciiBuf<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.vals).unwrap();
        write!(f, "{}", s)
    }
}

pub trait AsciiBuffable<const K: usize> {
    fn to_ascii_buf(&self) -> &AsciiBuf<K>;
    fn to_ascii_buf_off(&self, size: usize) -> &AsciiBuf<K>;
}

impl<T: AsRef<str>, const K: usize> AsciiBuffable<K> for T {
    fn to_ascii_buf_off(&self, size: usize) -> &AsciiBuf<K> {
        unsafe { &*(self.as_ref().as_ptr().add(size - K) as *const _ as *const AsciiBuf<K>) }
    }

    fn to_ascii_buf(&self) -> &AsciiBuf<K> {
        unsafe { &*(self.as_ref().as_ptr() as *const _ as *const AsciiBuf<K>) }
    }
}
