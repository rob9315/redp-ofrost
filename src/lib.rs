#![allow(unused)]
#![allow(non_camel_case_types)]

macro_rules! create_ty {
    (struct $name:ident {
        $($field:ident: $field_type:ty),+ $(,)?
    }) => {
        struct $name<'a> {
            $($field: <$field_type as Type<'a>>::V),+
        }
        impl<'a> Type<'a> for $name<'a> {
            type V = $name<'a>;
            fn enc(w: &mut impl std::io::Write, v: Self::V) -> std::io::Result<()> {
                $(<$field_type as Type<'a>>::enc(w, v.$field)?;)+
                Ok(())
            }
            fn dec(c: &mut std::io::Cursor<&'a [u8]>) -> std::io::Result<Self::V> {
                $(let $field = <$field_type as Type<'a>>::dec(c)?;)+
                Ok($name{$($field),+})
            }
        }
    };
}

create_ty! {
    struct Aurgh2 {
        entity_id: vi32,
        array_stuff: arr<bi32, vi32>,
        name: str<vi32>,
    }
}

use std::io::Read;

pub trait Type<'a> {
    type V: 'a;
    fn enc(w: &mut impl std::io::Write, v: Self::V) -> std::io::Result<()>;
    fn dec(c: &mut std::io::Cursor<&'a [u8]>) -> std::io::Result<Self::V>;
}
pub trait LengthType<'a>: Type<'a> {
    fn from_usize(u: usize) -> Self::V;
    fn into_usize(v: Self::V) -> usize;
}

macro_rules! len_typ {
    ($id:ident; $v:ty) => {
        impl LengthType<'_> for $id {
            fn from_usize(u: usize) -> Self::V {
                u as $v
            }
            fn into_usize(v: Self::V) -> usize {
                v as usize
            }
        }
    };
}

// bool "Boolean"
pub use bool;
impl Type<'_> for bool {
    type V = bool;
    fn enc(w: &mut impl std::io::Write, b: Self::V) -> std::io::Result<()> {
        w.write_all(&[b as u8])
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let b: u8 = 0;
        r.read_exact(&mut [b])?;
        Ok(b != 0)
    }
}

// i8/u8 "Byte/Unsigned Byte" [+"Angle"]
pub use i8;
impl Type<'_> for i8 {
    type V = i8;
    fn enc(w: &mut impl std::io::Write, i: Self::V) -> std::io::Result<()> {
        w.write_all(&[i as u8])
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let b: u8 = 0;
        r.read_exact(&mut [b])?;
        Ok(b as i8)
    }
}
len_typ!(i8; i8);

pub use u8;
impl Type<'_> for u8 {
    type V = u8;
    fn enc(w: &mut impl std::io::Write, u: Self::V) -> std::io::Result<()> {
        w.write_all(&[u])
    }
    fn dec(r: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self::V> {
        let b: u8 = 0;
        r.read_exact(&mut [b])?;
        Ok(b)
    }
}
len_typ!(u8; u8);

// the default in mc
// bu16 (big-endian) "Unsigned Short"
// [bu32/bu64 (big-endian)]
// bu128 "UUID"
// bi16/bi32/bi64 (big-endian) "Short/Int/Long"
// [bi128 (big-endian)]
// bf32/bf64 (big-endian) "Float/Double"
mod big_endian;
pub use big_endian::*;

// otherwise the default
// [li16/li32/li64/li128 (little-endian)]
// [lu16/lu32/lu64/lu128 (little-endian)]
// [lf32/lf64 (little-endian)]
mod little_endian;
pub use little_endian::*;

// [str<L> (string)]
pub struct str<L: for<'l> LengthType<'l>>(std::marker::PhantomData<L>);
impl<'a, L: for<'l> LengthType<'l>> Type<'a> for str<L> {
    type V = &'a std::primitive::str;
    fn enc(w: &mut impl std::io::Write, s: Self::V) -> std::io::Result<()> {
        buf::<L>::enc(w, s.as_bytes())
    }
    fn dec(c: &mut std::io::Cursor<&'a [u8]>) -> std::io::Result<Self::V> {
        let b = buf::<L>::dec(c)?;
        std::str::from_utf8(b).map_err(|e| unimplemented!())
    }
}

// mstr<L, n?> (mutf8 string) "String" (L=vi32, n=32767) [+"Json Chat", +"Identifier"]
pub struct mstr<L: for<'l> LengthType<'l>>(std::marker::PhantomData<L>);
impl<'a, L: for<'l> LengthType<'l>> Type<'a> for mstr<L> {
    type V = std::borrow::Cow<'a, std::primitive::str>;
    fn enc(w: &mut impl std::io::Write, v: Self::V) -> std::io::Result<()> {
        todo!()
    }
    fn dec(c: &mut std::io::Cursor<&'a [u8]>) -> std::io::Result<Self::V> {
        todo!()
    }
}

// buf<L> (byte-buffer) "Byte Array"
pub struct buf<L: for<'l> LengthType<'l>>(std::marker::PhantomData<L>);
impl<'a, L: for<'l> LengthType<'l>> Type<'a> for buf<L> {
    type V = &'a [u8];
    fn enc(w: &mut impl std::io::Write, v: Self::V) -> std::io::Result<()> {
        L::enc(w, L::from_usize(v.len()))?;
        w.write_all(v)
    }
    fn dec(c: &mut std::io::Cursor<&'a [u8]>) -> std::io::Result<&'a [u8]> {
        let l = L::into_usize(L::dec(c)?);
        let slice = c.get_ref();
        let pos = slice.len().min(c.position() as usize);
        let Some(b) = slice.get(pos..pos + l) else {
            return Err(unimplemented!());
        };
        c.set_position(pos as u64 + l as u64);
        Ok(b)
    }
}

// nbt/ubt "NBT/NBT(1.20.2+)" ('nbt named binary tags/µbt 'micro'/'unnamed' binary tags') [+"Chat"]
// extern
pub struct Compound<'a>(std::marker::PhantomData<&'a ()>);
pub struct nbt;
impl<'a> Type<'a> for nbt {
    type V = (std::borrow::Cow<'a, std::primitive::str>, Compound<'a>);
    fn enc(w: &mut impl std::io::Write, v: Self::V) -> std::io::Result<()> {
        todo!()
    }
    fn dec(c: &mut std::io::Cursor<&'_ [u8]>) -> std::io::Result<Self::V> {
        todo!()
    }
}
pub struct ubt;
impl<'a> Type<'a> for ubt {
    type V = Compound<'a>;
    fn enc(w: &mut impl std::io::Write, v: Self::V) -> std::io::Result<()> {
        todo!()
    }
    fn dec(c: &mut std::io::Cursor<&'_ [u8]>) -> std::io::Result<Self::V> {
        todo!()
    }
}

// [vi16 (varint)]
// vi32/vi64 (varint) "VarInt/VarLong"
// [vi128 (varint)]
// [vu16/vu32/vu64/vu128 (varint)]
mod varint;
pub use varint::*;
// [zi16/zi32/zi64 (zigzag)]
mod zigzag {
    // todo!
}
pub use zigzag::*;

// arr<T, L> (Type, Length) "Array of T" (with size of type L prefixed)
pub struct arr<T: for<'t> Type<'t>, L: for<'l> LengthType<'l>>(std::marker::PhantomData<(T, L)>);
impl<'a, T: for<'t> Type<'t>, L: for<'l> LengthType<'l>> Type<'a> for arr<T, L>
where
    <T as Type<'a>>::V: Clone,
{
    type V = std::borrow::Cow<'a, [<T as Type<'a>>::V]>;
    fn enc(w: &mut impl std::io::Write, v: Self::V) -> std::io::Result<()> {
        L::enc(w, L::from_usize(v.len()))?;
        for elem in v.into_iter() {
            T::enc(w, <T::V as Clone>::clone(elem));
        }
        Ok(())
    }
    fn dec(c: &mut std::io::Cursor<&'a [u8]>) -> std::io::Result<Self::V> {
        let l = L::into_usize(L::dec(c)?);
        let mut v = vec![];
        for _ in 0..l {
            v.push(T::dec(c)?);
        }
        Ok(std::borrow::Cow::Owned(v))
    }
}

mod enc {

    ///   - emd_<version> custom type => "Entity Metadata"
    ///   - slot_<version> custom type => "Slot"
    ///   - pos_<version> custom type => "Position"
    ///   - "X Enum" => just "X"
    ///   - manual
    const ADSF: u8 = 0;
}
pub fn aaaa(w: &mut impl std::io::Write) {}
