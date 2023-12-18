use super::{Type, LengthType};
use std::io::Read;

macro_rules! var {
    ($iid:ident, $uid:ident: $i:ty, $u:ty) => {
        var!(@impl $iid $i {as $u});
        var!(@impl $uid $u {});
    };
    (@impl $id:ident $v:ty {$($asunsigned:tt)*}) => {
        pub struct $id;
        impl Type<'_> for $id {
            type V = $v;
            fn enc(w: &mut impl std::io::Write, mut v: Self::V) -> std::io::Result<()> {
                loop {
                    let next_byte = (v $($asunsigned)* >> 7) as $v;
                    if next_byte == 0 {
                        w.write_all(&[v as u8])?;
                        break;
                    }
                    w.write_all(&[v as u8 | 0x80])?;
                    v = next_byte;
                }
                Ok(())
            }
            fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
                let mut v = 0;
                let mut current_byte = [0];
                for i in 0..(<$v>::BITS as usize + 6) / 7 {
                    r.read_exact(&mut current_byte)?;
                    v += ((current_byte[0] & 0x7f) $($asunsigned)*) << (i * 7);
                    if (current_byte[0] & 0x80) == 0x00 {
                        break;
                    }
                }
                Ok(v as $v)
            }
        }
        len_typ!($id; $v);
    }
}

var!(vi16, vu16: i16, u16);
var!(vi32, vu32: i32, u32);
var!(vi64, vu64: i64, u64);
var!(vi128, vu128: i128, u128);
