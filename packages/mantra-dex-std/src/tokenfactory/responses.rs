use anybuf::{Anybuf, Bufany};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdResult;
use mantrachain_std::types::cosmos::base::v1beta1::Coin;

use super::common::EncodeMessage;

///QueryParamsResponse is the response type for the Query/Params RPC method.
#[cw_serde]
pub struct QueryParamsResponse {
    pub params: Option<Params>,
}

/// Params defines the parameters for the tokenfactory module.
#[cw_serde]
pub struct Params {
    pub denom_creation_fee: Vec<Coin>,
    pub denom_creation_gas_consume: u64,
}

impl EncodeMessage for QueryParamsResponse {
    fn encode(data: Self) -> Vec<u8> {
        match data.params {
            Some(params) => {
                let mut params_buf = Anybuf::new();

                for coin in params.denom_creation_fee {
                    let coin_buf = Anybuf::new()
                        .append_string(1, coin.denom)
                        .append_string(2, &coin.amount);
                    params_buf = params_buf.append_message(1, &coin_buf);
                }

                params_buf = params_buf.append_uint64(2, params.denom_creation_gas_consume);

                Anybuf::new().append_message(1, &params_buf).into_vec()
            }
            None => Anybuf::new().into_vec(),
        }
    }

    fn decode(data: Vec<u8>) -> StdResult<Self>
    where
        Self: Sized,
    {
        if data.is_empty() {
            return Ok(QueryParamsResponse { params: None });
        }

        let deserialized = Bufany::deserialize(&data).unwrap();

        match deserialized.message(1) {
            Some(params_msg) => {
                let fee_coins_msgs = params_msg.repeated_message(1).unwrap();
                let mut denom_creation_fee = Vec::with_capacity(fee_coins_msgs.len());

                for coin_msg in fee_coins_msgs {
                    denom_creation_fee.push(Coin {
                        denom: coin_msg.string(1).unwrap(),
                        amount: coin_msg.string(2).unwrap(),
                    });
                }

                let denom_creation_gas_consume = params_msg.uint64(2).unwrap();

                Ok(QueryParamsResponse {
                    params: Some(Params {
                        denom_creation_fee,
                        denom_creation_gas_consume,
                    }),
                })
            }
            None => Ok(QueryParamsResponse { params: None }),
        }
    }
}
