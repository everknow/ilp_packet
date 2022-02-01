use interledger_packet::Packet;
use rustler::types::binary::{Binary};
use bytes::{BytesMut};

use rustler::types::atom::{ok, error};
use rustler::{Atom, Encoder, Env, Term, NifResult};
use std::convert::TryFrom;
use crate::custom_atoms::{fulfill, prepare, reject};

mod custom_atoms {
    rustler::atoms! {
        prepare,
        fulfill,
        reject
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn decode<'a>(env: Env<'a>, bin: Binary) -> NifResult<Term<'a>> {
    match Packet::try_from(BytesMut::from(bin.as_slice())) {
        Ok(Packet::Prepare(p)) => response(env, prepare(), p.data()),
        Ok(Packet::Fulfill(p)) => response(env, fulfill(), p.data()),
        Ok(Packet::Reject(p)) => response(env, reject(), p.data()),
        Err(err) => Ok((error(), err.to_string()).encode(env))
    }
}

fn response<'a>(env: Env<'a>, packet_type: Atom, data: &[u8]) -> NifResult<Term<'a>> {
    let keys = ["type", "data"];
    let values = [packet_type.encode(env), data.encode(env)];
    let result = Term::map_from_arrays(env, &keys.map( |x| x.encode(env)), &values);
    Ok((ok(), result.unwrap()).encode(env))
}

rustler::init!("Elixir.IlpPacket", [decode]);
