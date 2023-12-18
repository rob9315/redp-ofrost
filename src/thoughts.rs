
// name_versionid
mod xyz_packet_321 {
    struct Packet {
        id: i32,
        data: Vec<u8>,
    }
    impl Packet {
        pub fn new(id: i32) -> Self {
            Packet { id, data: vec![] }
        }
        pub fn vi32(&mut self, val: i32) -> &mut Self {
            // something something encode
            self
        }
    }
    pub struct XyzPacket {
        // #[var]
        entity_id: i32,
        // #[arr[bi32; vi32]]
        array_stuff: Vec<i32>,
        // #[str[; vi32]]
        name: String,
    }

    #[repr(transparent)]
    pub struct XyzPacketBuilder<S>(super::Packet, std::marker::PhantomData<S>);

    impl<S> From<Packet> for XyzPacketBuilder<S> {
        fn from(value: Packet) -> Self {
            XyzPacketBuilder(value, Default::default())
        }
    }
    impl<S> Into<Packet> for XyzPacketBuilder<S> {
        fn into(self) -> Packet {
            self.0
        }
    }

    /// these won't collide because they are fields whose names cannot collide
    struct EntityId;
    impl XyzPacketBuilder<EntityId> {
        pub fn entity_id(
            XyzPacketBuilder(mut p, _): Self,
            id: i32,
        ) -> XyzPacketBuilder<ArrayStuff> {
            p.vi32(id);
            XyzPacketBuilder(p, Default::default())
        }
    }

    struct ArrayStuff;
    impl XyzPacketBuilder<ArrayStuff> {
        pub fn array_stuff<Arr>(
            XyzPacketBuilder(mut p, _): Self,
            arr: Arr,
        ) -> XyzPacketBuilder<Name>
// where
        //     Arr: conditions_for_arr_on_packet,
        {
            // p.arr(arr);
            XyzPacketBuilder(p, Default::default())
        }
    }

    struct Name;
    impl XyzPacketBuilder<Name> {
        pub fn array_stuff(XyzPacketBuilder(mut p, _): Self, s: &str) -> super::Packet {
            // p.str(s);
            p
        }
    }

    // that works i guess
}

// as for decoding

/// i mean you can do a similar thing
///
/// ```rust
/// // or PacketReader(packet)
/// let pr = PacketReader(id, buf);
/// let id = pr.vi32()?;
/// let arr = pr.arr()?; // i guess this shouldn't exist?
/// // it should probably just be manually reading the length and then looping?
/// let arr_len = pr.vi32()?;
/// let mut arr = vec![];
/// for i in 0..arr_len {
///     arr.push(pr.bi32()?);
/// }
/// // well i mean that works... sure... for now
/// let str_len = pr.vi32()?;
/// let str = pr.str(str_len)?; // mirroring that then?
/// ```
/// // but for a builder-like api we would need apis for arrays etc.
///
/// ```rust
/// // this in much more complicated maybe?
/// impl X {
///     // T = Type
///     // C = Counter
///     pub fn arr<T, C>(&mut self) -> Vec<T> {
///         let n = usize::from(C::decode(self));
///         let v = vec![];
///         for i in 0..n {
///             v.push(T::decode(self)?);
///         }
///         v
///     }
/// }
/// ```
///
/// do we really want structs that hold all the data? (yes)
/// yes because when decoding you don't want the user to have to call once for each thing that can be taken out
/// they call a method and get a struct with all the things
/// also this thing can be constructed by the user to be encoded automatically (for delegating sending)
/// when decoding all non-owned data has the same lifetime
/// when encoding you don't want more than one lifetime (you might want to but i'll not give you)
/// this makes everything much more simple than multiple lifetimes
/// ```
/// struct Xyz<'a> {
///     // it will map to the closest datatype and not a special one which implements the en- and decoding
///     // thus there will have to be a trait for each encoding version to not have duplicate structs
///     entity_id: i32,
///     // there could also be no variables requiring a lifetime here, in that case there wouldn't be a lifetime i guess
///     array_stuff: Vec<i32> /* &[bi32] ????????? */,
///     name: &'a str,
/// }
/// ```
///
/// OOPS i forgot about substructs
///
/// struct B {
///     substructdata: u8,
/// }
///
/// struct A {
///     entity_id: vi32,
///     b: B,
///     idfk: u8,
/// }
///
/// // for manual writing just "flatten" the structures
/// Packet()
///     .vi32() // .entity_id
///     .u8()   // .b.substructdata
///     .u8()   // .idfk
/// // same for manual decoding
///
/// // for helpers:
///
/// // writing
/// APacketBuilder()
///     .entity_id(0x234)
///     .b(|b: BPacketBuilder| -> Packet { // something something just return my writer
///         b.substructdata(0x65)
///         // ˆ returns Packet or smth else containing the current writer state
///     })
///     .idfk(0x03)
///
/// // reading
/// idk it just decodes the other structs like it would decode the main struct in place of another type
///
///
///
/// what is for x-coding:
/// - read and write methods for all datatypes
///   - trait-ed for one-time functions for arrays etcetc.
///   - bool "Boolean"
///   - i8/u8 "Byte/Unsigned Byte" [+"Angle"]
///   - bu16 (big-endian) "Unsigned Short"
///   - [bu32/bu64 (big-endian)]
///   - bu128 "UUID"
///   - bi16/bi32/bi64 (big-endian) "Short/Int/Long"
///   - [bi128 (big-endian)]
///   - [li16/li32/li64/li128 (little-endian)]
///   - [lu16/lu32/lu64/lu128 (little-endian)]
///   - bf32/bf64 (big-endian) "Float/Double"
///   - [lf32/lf64 (little-endian)]
///   - [str<L> (string)]
///   - mstr<L, n?> (mutf8 string) "String" (L=vi32, n=32767) [+"Json Chat", +"Identifier"]
///   - nbt/ubt "NBT/NBT(1.20.2+)" ('nbt named binary tags/µbt 'micro'/'unnamed' binary tags') [+"Chat"]
///   - vi32/vi64 (varint) "VarInt/VarLong"
///   - [vi16 (varint)]
///   - [zi16/zi32/zi64 (zigzag)]
///   - emd_<version> custom type => "Entity Metadata"
///   - slot_<version> custom type => "Slot"
///   - pos_<version> custom type => "Position"
///   - arr<T, L> (Type, Length) "Array of T" (with size of type L prefixed)
///   - "X Enum" => just "X"
///   - buf<L> (byte-buffer) "Byte Array"
///   - manual
///
/// for each existing packet always generate:
/// - builder with "stages" for encoding
/// - default struct? *wait no*, we said we want traits for this
///   - decoding function returning this one
///   - into-method or smth calling all "stages" in order
///
/// alternative to struct:
///     tuple of tuple or not or smth
///     let's try that (below \/)
///
/// at request of user generate?:
/// - struct with user-chosen members representing the fields
///
///
const A: &[u8] = b"";
