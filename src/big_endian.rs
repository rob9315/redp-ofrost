use super::{Type, LengthType};
use std::io::Read;

pub struct bu16;
impl Type<'_> for bu16 {
    type V = u16;
    fn enc(w: &mut impl std::io::Write, u: Self::V) -> std::io::Result<()> {
        w.write_all(&u16::to_be_bytes(u))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 2];
        r.read_exact(&mut be[..])?;
        Ok(u16::from_be_bytes(be))
    }
}
len_typ!(bu16; u16);

pub struct bu32;
impl Type<'_> for bu32 {
    type V = u32;
    fn enc(w: &mut impl std::io::Write, u: Self::V) -> std::io::Result<()> {
        w.write_all(&u32::to_be_bytes(u))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 4];
        r.read_exact(&mut be[..])?;
        Ok(u32::from_be_bytes(be))
    }
}
len_typ!(bu32; u32);

pub struct bu64;
impl Type<'_> for bu64 {
    type V = u64;
    fn enc(w: &mut impl std::io::Write, u: Self::V) -> std::io::Result<()> {
        w.write_all(&u64::to_be_bytes(u))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 8];
        r.read_exact(&mut be[..])?;
        Ok(u64::from_be_bytes(be))
    }
}
len_typ!(bu64; u64);

pub struct bu128;
impl Type<'_> for bu128 {
    type V = u128;
    fn enc(w: &mut impl std::io::Write, u: Self::V) -> std::io::Result<()> {
        w.write_all(&u128::to_be_bytes(u))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 16];
        r.read_exact(&mut be[..])?;
        Ok(u128::from_be_bytes(be))
    }
}
len_typ!(bu128; u128);

pub struct bi16;
impl Type<'_> for bi16 {
    type V = i16;
    fn enc(w: &mut impl std::io::Write, i: Self::V) -> std::io::Result<()> {
        w.write_all(&i16::to_be_bytes(i))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 2];
        r.read_exact(&mut be[..])?;
        Ok(i16::from_be_bytes(be))
    }
}
len_typ!(bi16; i16);

pub struct bi32;
impl Type<'_> for bi32 {
    type V = i32;
    fn enc(w: &mut impl std::io::Write, i: Self::V) -> std::io::Result<()> {
        w.write_all(&i32::to_be_bytes(i))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 4];
        r.read_exact(&mut be[..])?;
        Ok(i32::from_be_bytes(be))
    }
}
len_typ!(bi32; i32);

pub struct bi64;
impl Type<'_> for bi64 {
    type V = i64;
    fn enc(w: &mut impl std::io::Write, i: Self::V) -> std::io::Result<()> {
        w.write_all(&i64::to_be_bytes(i))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 8];
        r.read_exact(&mut be[..])?;
        Ok(i64::from_be_bytes(be))
    }
}
len_typ!(bi64; i64);

pub struct bi128;
impl Type<'_> for bi128 {
    type V = i128;
    fn enc(w: &mut impl std::io::Write, i: Self::V) -> std::io::Result<()> {
        w.write_all(&i128::to_be_bytes(i))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 16];
        r.read_exact(&mut be[..])?;
        Ok(i128::from_be_bytes(be))
    }
}
len_typ!(bi128; i128);

pub struct bf32;
impl Type<'_> for bf32 {
    type V = f32;
    fn enc(w: &mut impl std::io::Write, f: Self::V) -> std::io::Result<()> {
        w.write_all(&f32::to_be_bytes(f))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 4];
        r.read_exact(&mut be[..])?;
        Ok(f32::from_be_bytes(be))
    }
}

pub struct bf64;
impl Type<'_> for bf64 {
    type V = f64;
    fn enc(w: &mut impl std::io::Write, f: Self::V) -> std::io::Result<()> {
        w.write_all(&f64::to_be_bytes(f))
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let mut be = [0u8; 8];
        r.read_exact(&mut be[..])?;
        Ok(f64::from_be_bytes(be))
    }
}