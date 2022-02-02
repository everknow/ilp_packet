use interledger_packet::{Packet, Prepare, Fulfill, Reject};
use rustler::types::binary::{Binary};
use bytes::{BytesMut};

use rustler::types::atom::{ok, error};
use rustler::{Encoder, Env, Term, NifResult, OwnedBinary};
use std::convert::TryFrom;
use crate::custom_atoms::{fulfill, prepare, reject};

use chrono::offset::Utc;
use chrono::DateTime;

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
        Ok(Packet::Prepare(p)) => tuple_response(env, PacketDecoder::extract_data(env, p)),
        Ok(Packet::Fulfill(p)) => tuple_response(env, PacketDecoder::extract_data(env, p)),
        Ok(Packet::Reject(p)) => tuple_response(env, PacketDecoder::extract_data(env, p)),
        Err(err) => Ok((error(), err.to_string()).encode(env))
    }
}

trait PacketDecoder {
    fn extract_data<'a>(env: Env<'a>, packet: Self) -> NifResult<Term<'a>>;
}

impl PacketDecoder for Prepare {
    fn extract_data<'a>(env: Env<'a>, packet: Prepare) -> NifResult<Term<'a>> {
        let keys = [
            "type",
            "amount",
            "expires_at",
            "execution_condition",
            "destination",
            "data"
        ];

        let data = packet.data();
        let mut data_bin: OwnedBinary = OwnedBinary::new(data.len()).unwrap();
        data_bin.as_mut_slice().copy_from_slice(data);

        let expires_at: DateTime<Utc> = packet.expires_at().into();

        let execution_condition = packet.execution_condition();
        let mut execution_condition_bin: OwnedBinary = OwnedBinary::new(execution_condition.len()).unwrap();
        execution_condition_bin.as_mut_slice().copy_from_slice(execution_condition);

        let values = [
            prepare().encode(env),
            packet.amount().encode(env),
            expires_at.to_string().encode(env),
            execution_condition_bin.release(env).encode(env),
            packet.destination().encode(env),
            data_bin.release(env).encode(env)
        ];

        return Term::map_from_arrays(env, &keys.map( |x| x.encode(env)), &values);
    }
}

impl PacketDecoder for Fulfill {
    fn extract_data<'a>(env: Env<'a>, packet: Fulfill) -> NifResult<Term<'a>> {
        let keys = [
            "type",
            "fulfillment",
            "data"
        ];

        let data = packet.data();
        let mut data_bin: OwnedBinary = OwnedBinary::new(data.len()).unwrap();
        data_bin.as_mut_slice().copy_from_slice(data);

        let fulfillment = packet.fulfillment();
        let mut fulfillment_bin: OwnedBinary = OwnedBinary::new(fulfillment.len()).unwrap();
        fulfillment_bin.as_mut_slice().copy_from_slice(fulfillment);

        let values = [
            fulfill().encode(env),
            fulfillment_bin.release(env).encode(env),
            data_bin.release(env).encode(env)
        ];

        return Term::map_from_arrays(env, &keys.map( |x| x.encode(env)), &values);
    }
}

impl PacketDecoder for Reject {
    fn extract_data<'a>(env: Env<'a>, packet: Reject) -> NifResult<Term<'a>> {
        let keys = [
            "type",
            "code",
            "triggered_by",
            "message",
            "data"
        ];

        let data = packet.data();
        let mut data_bin: OwnedBinary = OwnedBinary::new(data.len()).unwrap();
        data_bin.as_mut_slice().copy_from_slice(data);

        let values = [
            reject().encode(env),
            format!("{}", packet.code()).encode(env),
            format!("{}", packet.triggered_by().unwrap()).encode(env),
            packet.message().encode(env),
            data_bin.release(env).encode(env)
        ];

        Term::map_from_arrays(env, &keys.map( |x| x.encode(env)), &values)
    }
}

fn tuple_response<'a>(env: Env<'a>, result: NifResult<Term>) -> NifResult<Term<'a>> {
    return Ok((ok(), result.unwrap()).encode(env))
}

rustler::init!("Elixir.IlpPacket", [decode]);
