# mine-rs protocol

transport layer?
maybe we are just transporting signals
the same signal can have a different meaning in a different version

update events trigger on changed signals but how do you group them

why signals and not packets? multiple packets transmit the same signals

entity ids are like multiplexors i guess

predecessors
- protodef
    -> json generating javascript composable to parse packets
    + easy to use
    - not very typed -> error-prone
    - does not translate well to typed stuff
- protobuf
    -> arbitrary buffer en- and decoding
    - not minecraft
    + elegant
    + widely used

other recently looked at things that might be of influence
- can bus dbc
    -> defining messages for Controller Area Networks

# goals

*shouts like they do at CppCon:* WE WANT EVERYTHING FOR THE COST OF NOTHING (except compile time oh god the compile time)

okay so:
- every version
- opt-in
- cheap
-> all message structs defined in a crate
-> macro to generate handlers or smth

Net crate to shove network traffic through protocol (de)serialization

next layer?
tooling with goodish interoperability

Client tools:
- Statekeeper
  - World state (similar to server)
  - Entity states
- Delegating Plugin Tree ?

Server tools:
- World(s)

## now to the most important part

name the test project folder

old names that i might want to jumble up
protodef-rs

redp-ofrost

## problems in the past

owning and non-owning strings, arrays, etc.
conclusion: just don't have owning types (have at most one owning type), references shouldn't exist for long, no single reference for each string/array, whatever
small numbers should be copied and used in little-endian format until encoded

as this is only a transport protocol we do not want to have enums unless they change the structure of the message

## having said that

how should it look now

## oh boy let me tell you about something i thought about

so what if we did it similarly to protodef where you specify the fields in order for writing

example:

Packet(id)
    .vi32(0x2)
    .array::<Bu32, Vi32>(|a|{
        a.extend(vec![0x23, 0x10, 0x53])
    })
    // specialisation of array
    .str::<Vi32>("idfk")

// where this would actually write the buffer which would be sent into a packet structure
Packet {
    id: u32,
    data: Vec<u8>
}
// returning a nonowning thing as well? - no, there are datatypes that don't just want a reference into a buffer (Vec<Bi32>, Vi32, etcetc) effectively lists of variable length stuff

// ==

v1_12_2::XyzPacket()
    .entity_id(0x02)
    .array_stuff(|a|{
        a.extend(vec![0x23, 0x10, 0x53])
    })
    .name("idfk")
    .manual(|w: &mut Writer<_>| {
        // do whatever
    })

// how do you do that with rust, let's have a thought

people like state types for this

struct EntityId;
struct ArrayStuff;
struct Name;

of course they will need to be namespaced to avoid collisions

mod xyz_packet {

    /// these won't collide because they are fields whose names cannot collide
    struct EntityId;
    struct ArrayStuff;
    struct Name;

    // now do we just want to add alias/forwarding methods?
    // hmm, maybe
    
}
