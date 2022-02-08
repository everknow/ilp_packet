use interledger_packet::{Address,
    ErrorCode,
    Fulfill,
    FulfillBuilder,
    Packet,
    Prepare,
    PrepareBuilder,
    Reject,
    RejectBuilder
};

use rustler::types::binary::Binary;
use bytes::BytesMut;

use rustler::types::atom::{ok, error};
use rustler::{Env, Encoder, NifResult, OwnedBinary, Term, Error};
use std::convert::TryFrom;
use std::collections::HashMap;
use std::str;
use crate::custom_atoms::{fulfill, prepare, reject};

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

mod custom_atoms {
    rustler::atoms! {
        prepare,
        fulfill,
        reject
    }
}

#[macro_export]
macro_rules! err {
    ( $( $x:expr ),* ) => {
        {
            $(
                Err(Error::Term(Box::new($x)))
            )*
        }
    };
}
macro_rules! error {
    ( $( $x:expr ),* ) => {
        {
            $(
                Error::Term(Box::new($x))
            )*
        }
    };
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
        let mut data_bin: OwnedBinary = OwnedBinary::new(data.len()).ok_or(error!("could not decode data"))?;
        data_bin.as_mut_slice().copy_from_slice(data);

        let expires_at: DateTime<Utc> = packet.expires_at().into();

        let execution_condition = packet.execution_condition();
        let mut execution_condition_bin: OwnedBinary = OwnedBinary::new(execution_condition.len())
            .ok_or(error!("could not decode execution_condition"))?;

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
        let mut data_bin: OwnedBinary = OwnedBinary::new(data.len()).ok_or(error!("could not decode data"))?;
        data_bin.as_mut_slice().copy_from_slice(data);

        let fulfillment = packet.fulfillment();
        let mut fulfillment_bin: OwnedBinary = OwnedBinary::new(fulfillment.len()).ok_or(error!("could not decode fulfillment"))?;
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
        let mut data_bin: OwnedBinary = OwnedBinary::new(data.len()).ok_or(error!("could not decode data"))?;
        data_bin.as_mut_slice().copy_from_slice(data);

        let values = [
            reject().encode(env),
            format!("{}", packet.code()).encode(env),
            format!("{}", packet.triggered_by().ok_or(error!("could not decode triggered_by address"))?).encode(env),
            packet.message().encode(env),
            data_bin.release(env).encode(env)
        ];

        Term::map_from_arrays(env, &keys.map( |x| x.encode(env)), &values)
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn encode_prepare<'a>(env: Env<'a>, args: Term) -> NifResult<Term<'a>> {
    let params = args.decode::<HashMap<String, Term>>().or(err!("args need to be a string keyed map"))?;

    let amount = params.get("amount").ok_or(error!("amount is missing"))?;
    let u64_amount: u64 = amount.decode::<u64>().or(err!("expected amount as u64"))?;

    let destination = params.get("destination").ok_or(error!("destination is missing"))?;
    let destination_address = Address::try_from(destination.into_binary()?.as_slice())
        .or(err!("invalid destination address"))?;

    let execution_condition = params.get("execution_condition")
        .ok_or(error!("execution_condition is missing"))?;
    
    let u8_execution_condition = execution_condition.into_binary()
        .or(err!("could not decode execution_condition"))?.as_slice();
    
    let bin_execution_condition = &<[u8; 32]>::try_from(u8_execution_condition)
        .or(err!("expected execution_condition to be [u8; 32]"))?;

    let data = params.get("data").ok_or(error!("data is missing"))?;
    let bin_data = data.into_binary().or(err!("expected data to be binary"))?.as_slice();

    let expires_at = params.get("expires_at").ok_or(error!("expires_at is missing"))?;
    let bin_expires_at = expires_at.into_binary().or(err!("expected expires_at to be binary"))?.as_slice();
    let utf8_expires_at = str::from_utf8(bin_expires_at).or(err!("expires_at should be utf8"))?;

    let date_time_expires_at = DateTime::parse_from_rfc3339(utf8_expires_at)
        .or(err!("could not parse expires_at as DateTime"))?;
        
    let system_time_expires_at: SystemTime = date_time_expires_at.with_timezone(&Utc).into();

    let prepare = PrepareBuilder {
        amount: u64_amount,
        destination: destination_address,
        expires_at: system_time_expires_at,
        execution_condition: bin_execution_condition,
        data: bin_data
    }.build();

    return Ok((ok(), BytesMut::from(prepare).encode(env)).encode(env));
}

#[rustler::nif(schedule = "DirtyCpu")]
fn encode_fulfill<'a>(env: Env<'a>, args: Term) -> NifResult<Term<'a>> {
    let params = args.decode::<HashMap<String, Term>>().or(err!("args need to be a string keyed map"))?;

    let fulfillment = params.get("fulfillment").ok_or(error!("fulfillment is missing"))?;
    let u8_fulfillment = fulfillment.into_binary().or(err!("expected fulfillment to be binary"))?.as_slice();
    let bin_fulfillment = &<[u8; 32]>::try_from(u8_fulfillment).or(err!("fulfillment should be [u8; 32]"))?;

    let data = params.get("data").ok_or(error!("data is missing"))?;
    let bin_data = data.into_binary().or(err!("expected data to be binary"))?.as_slice();

    let fulfill = FulfillBuilder {
        fulfillment: bin_fulfillment,
        data: bin_data
    }.build();

    return Ok((ok(), BytesMut::from(fulfill).encode(env)).encode(env));
}

#[rustler::nif(schedule = "DirtyCpu")]
fn encode_reject<'a>(env: Env<'a>, args: Term) -> NifResult<Term<'a>> {
    let params = args.decode::<HashMap<String, Term>>().or(err!("args need to be a string keyed map"))?;

    let code = params.get("code").ok_or(error!("code is missing"))?;
    let bin_code = <[u8; 3]>::try_from(code.into_binary().or(err!("expected code to be binary"))?
        .as_slice()).or(err!("expected code to be [u8; 3]"))?;

    let error_code = ErrorCode::new(bin_code);

    let message = params.get("message").ok_or(error!("message is missing"))?;
    let bin_message = message.into_binary().or(err!("expected message to be binary"))?.as_slice();

    let triggered_by = params.get("triggered_by").ok_or(error!("triggered_by is missing"))?;
    let triggered_by_address = Address::try_from(triggered_by.into_binary()
        .or(err!("expected triggered_by to be binary"))?
        .as_slice())
        .or(err!("invalid triggered_by address"))?;

    let data = params.get("data").ok_or(error!("data is missing"))?;
    let bin_data = data.into_binary().or(err!("expected data to be binary"))?.as_slice();

    let reject = RejectBuilder {
        code: error_code,
        message: bin_message,
        triggered_by: Some(&triggered_by_address),
        data: bin_data
    }.build();

    return Ok((ok(), BytesMut::from(reject).encode(env)).encode(env));
}

fn tuple_response<'a>(env: Env<'a>, result: NifResult<Term>) -> NifResult<Term<'a>> {
    return Ok((ok(), result.or(err!("could not encode final result"))?).encode(env))
}

rustler::init!("Elixir.IlpPacket", [decode, encode_prepare, encode_fulfill, encode_reject]);
