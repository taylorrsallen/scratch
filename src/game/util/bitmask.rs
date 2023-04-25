//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// OnMaskIter
#[derive(Clone)]
pub struct OnMaskIter<'a> {
    index: usize,
    parent: &'a Bitmask,
}

impl<'a> OnMaskIter<'a> {
    pub fn new(index: usize, parent: &'a Bitmask) -> Self {
        Self { index, parent }
    }
}

impl<'a> Iterator for OnMaskIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.parent.get_next_bit_on(self.index);
        if self.index != self.parent.size {
            self.index += 1;
            Some(self.index - 1)
        } else {
            None
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// Bitmask
#[derive(Default, Clone, Debug)]
pub struct Bitmask {
    pub size: usize,
    pub word_num: usize,
    pub words: Vec<usize>,
}

impl Bitmask {
    pub fn from_dim2d(dim: &IVec2, active: bool) -> Self {
        let mut word: usize = 0;
        if active { word = usize::MAX; }

        let size = (dim.x * dim.y) as usize;
        let word_num = (size >> 6) + 1;
        Self { size, word_num, words: vec![word; word_num] }
    }

    // pub fn from_log2(
    //     dim: &IVec2,
    //     active: bool,
    // ) -> Self {
    //     let mut word: usize = 0;
    //     if active { word = usize::MAX; }

    //     if log2 < 6 {
    //         return Self { size: 64, word_num: 1, words: vec![word; 1] };
    //     } else {
    //         let size = 1 << log2;
    //         let word_num = size >> 6;
    //         Self { size, word_num, words: vec![word; word_num] }
    //     }
    // }

    pub fn from_cube_log2dim(log2dim: usize, active: bool) -> Self {
        let mut word: usize = 0;
        if active { word = usize::MAX; }

        if log2dim == 1 {
            return Self { size: 64, word_num: 1, words: vec![word; 1] };
        } else {
            let size = 1 << (log2dim*3);
            let word_num = size >> 6;
            Self { size, word_num, words: vec![word; word_num] }
        }
    }

    pub fn from_cube_size(size: usize, active: bool) -> Self {
        let mut word: usize = 0;
        if active { word = usize::MAX; }

        let word_num = size >> 6;
        Self { size, word_num, words: vec![word; word_num] }
    }

    pub fn get_next_bit_on(&self, start: usize) -> usize {
        let mut n = start >> 6;
        if n >= self.word_num { return self.size; }
        
        let m = start & 63;
        let mut word = self.words[n as usize];
        if word & ((1 as usize) << m) > 0 { return start; } // simple case: start is active
    
        word &= ((usize::MAX >> m) << m); // mask out lower bits
        while word == 0 {
            n += 1;
            if n == self.word_num { break; }
            word = self.words[n as usize]; // find next non-zero word
        }
        
        if word == 0 {
            self.size
        } else {
            (n << 6) + self.get_lowest_bit_on(word)
        }
    }

    pub fn get_next_bit_off(&self, start: usize) -> usize {
        let mut n = start >> 6;
        if n >= self.word_num { return self.size; }
        
        let m = start & 63;
        let mut word = !self.words[n];
        if word & ((1 as usize) << m) > 0 { return start; } // simple case: start is active
    
        word &= ((usize::MAX >> m) << m); // mask out lower bits
        while word == 0 {
            n += 1;
            if n == self.word_num { break; }
            word = !self.words[n]; // find next non-zero word
        }
        
        if word == 0 {
            self.size
        } else {
            (n << 6) + self.get_lowest_bit_on(word)
        }
    }
    
    pub fn get_lowest_bit_on(&self, word: usize) -> usize {
        word.trailing_zeros() as usize
    }

    pub fn is_bit_on(&self, index: usize) -> bool {
        0 != (self.words[index >> 6] & ((1 as usize) << (index & 63)))
    }

    pub fn is_bit_off(&self, index: usize) -> bool {
        0 == (self.words[index >> 6] & ((1 as usize) << (index & 63)))
    }

    pub fn set_on(&mut self) {
        for word in self.words.iter_mut() { *word = usize::MAX; }
    }

    pub fn set_off(&mut self) {
        for word in self.words.iter_mut() { *word = 0; }
    }

    pub fn set_bit(&mut self, index: usize, active: bool) {
        if active { self.set_bit_on(index); } else { self.set_bit_off(index); }
    }

    pub fn set_bit_on(&mut self, index: usize) {
        self.words[index >> 6] |= (1 as usize) << (index & 63);
    }

    pub fn set_bit_off(&mut self, index: usize) {
        self.words[index >> 6] &= !((1 as usize) << (index & 63));
    }
}

pub fn is_flag_on_u8<T: Flag>(mask: u8, flag: T) -> bool {
    0 != mask & flag.u8()
}
pub fn is_flag_off_u8<T: Flag>(mask: u8, flag: T) -> bool {
    0 == mask & flag.u8()
}
pub fn is_flag_on_u16<T: Flag>(mask: u16, flag: T) -> bool {
    0 != mask & flag.u16()
}
pub fn is_flag_off_u16<T: Flag>(mask: u16, flag: T) -> bool {
    0 == mask & flag.u16()
}
pub fn is_flag_on_u32<T: Flag>(mask: u32, flag: T) -> bool {
    0 != mask & flag.u32()
}
pub fn is_flag_off_u32<T: Flag>(mask: u32, flag: T) -> bool {
    0 == mask & flag.u32()
}

/// Type is a bit flag
pub trait Flag {
    #[inline(always)]
    fn u8(self) -> u8;
    #[inline(always)]
    fn u16(self) -> u16;
    #[inline(always)]
    fn u32(self) -> u32;
}