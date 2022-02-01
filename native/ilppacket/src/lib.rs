use interledger_packet::Packet;
use rustler::types::binary::{Binary};
use bytes::{BytesMut};

use rustler::types::atom::{ok, error};
use rustler::{Encoder, Env, Term, NifResult};
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
        Ok(Packet::Prepare(p)) => {
            Term::map_from_arrays(env, &[
                "type",
                "data"
            ].map( |x| x.encode(env)),&[
                prepare().encode(env),
                p.data().encode(env)
            ])
        }
        Ok(Packet::Fulfill(f)) => {
            Term::map_from_arrays(env, &[
                "type",
                "data"
            ].map( |x| x.encode(env)),&[
                fulfill().encode(env),
                f.data().encode(env)
            ])
        }
        Ok(Packet::Reject(r)) => {
            Term::map_from_arrays(env, &[
                "type",
                "data"
            ].map( |x| x.encode(env)),&[
                reject().encode(env),
                r.data().encode(env)
            ])
        }
        Err(_) => {
            Ok((error()).encode(env))
        }
    }
}

rustler::init!("Elixir.IlpPacket", [decode]);
