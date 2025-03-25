use crate::tokenfactory::common::{create_msg, MsgTypes};
use anybuf::{Anybuf, Bufany};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdResult;

use cosmwasm_std::{Addr, CosmosMsg};

use crate::tokenfactory::common::EncodeMessage;

/// Returns the MsgSetBeforeSendHook Stargate message
pub fn set_before_send_hook(sender: Addr, denom: String, contract_addr: String) -> CosmosMsg {
    let message_data = MsgSetBeforeSendHook {
        sender: sender.to_string(),
        denom,
        contract_addr,
    };
    create_msg(message_data, MsgTypes::SetBeforeSendHook.as_str())
}

#[cw_serde]
pub struct MsgSetBeforeSendHook {
    pub sender: String,
    pub denom: String,
    pub contract_addr: String,
}

impl EncodeMessage for MsgSetBeforeSendHook {
    fn encode(data: Self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, data.sender)
            .append_string(2, data.denom)
            .append_string(3, data.contract_addr)
            .into_vec()
    }

    fn decode(data: Vec<u8>) -> StdResult<Self>
    where
        Self: Sized,
    {
        let deserialized = Bufany::deserialize(&data).unwrap();
        Ok(Self {
            sender: deserialized.string(1).unwrap(),
            denom: deserialized.string(2).unwrap(),
            contract_addr: deserialized.string(3).unwrap(),
        })
    }
}

/// MsgSetBeforeSendHookResponse defines the response structure for an executed
/// MsgSetBeforeSendHook message.
#[cw_serde]
pub struct MsgSetBeforeSendHookResponse {
    pub new_token_denom: String,
}
