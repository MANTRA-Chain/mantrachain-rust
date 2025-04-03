use anyhow::Result as AnyResult;
use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_std::{
    coins, to_json_binary, Addr, AnyMsg, Api, BankMsg, Binary, BlockInfo, CustomMsg, CustomQuery,
    GrpcQuery, MsgResponse, Querier, Storage, SubMsgResponse,
};
use cw_multi_test::{AppResponse, BankSudo, CosmosRouter, Stargate};

use mantra_dex_std::tokenfactory::burn::MsgBurn;
use mantra_dex_std::tokenfactory::common::EncodeMessage;
use mantra_dex_std::tokenfactory::create_denom::{MsgCreateDenom, MsgCreateDenomResponse};
use mantra_dex_std::tokenfactory::mint::MsgMint;
use mantra_dex_std::tokenfactory::responses::{Params, QueryParamsResponse};
use mantrachain_std::types::cosmos::base::v1beta1::Coin;

pub struct StargateMock {
    pub fees: Vec<cosmwasm_std::Coin>,
}

impl StargateMock {
    pub fn new(fees: Vec<cosmwasm_std::Coin>) -> Self {
        Self { fees }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_stargate_any<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        type_url: String,
        value: Binary,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        #[allow(deprecated)]
        match type_url.as_str() {
            "/osmosis.tokenfactory.v1beta1.MsgCreateDenom" => {
                let tf_msg: MsgCreateDenom = MsgCreateDenom::decode(value.into())?;
                let submsg_response = SubMsgResponse {
                    events: vec![],
                    data: Some(to_json_binary(&MsgCreateDenomResponse {
                        new_token_denom: format!("factory/{}/{}", tf_msg.sender, tf_msg.subdenom),
                    })?),
                    msg_responses: vec![MsgResponse {
                        type_url,
                        value: to_json_binary(&MsgCreateDenomResponse {
                            new_token_denom: format!(
                                "factory/{}/{}",
                                tf_msg.sender, tf_msg.subdenom
                            ),
                        })?,
                    }],
                };

                // the token factory consumes all the tokens set as fees
                let burn_msg = BankMsg::Burn {
                    amount: self.fees.clone(),
                };

                router.execute(
                    api,
                    storage,
                    block,
                    Addr::unchecked(tf_msg.sender),
                    burn_msg.into(),
                )?;

                Ok(submsg_response.into())
            }
            "/osmosis.tokenfactory.v1beta1.MsgMint" => {
                let tf_msg: MsgMint = MsgMint::decode(value.into())?;
                let mint_coins = tf_msg.amount;
                let bank_sudo = BankSudo::Mint {
                    to_address: tf_msg.mint_to_address,
                    amount: coins(mint_coins.amount.u128(), mint_coins.denom),
                };
                router.sudo(api, storage, block, bank_sudo.into())
            }
            "/osmosis.tokenfactory.v1beta1.MsgBurn" => {
                let tf_msg: MsgBurn = MsgBurn::decode(value.into())?;
                let burn_coins = tf_msg.amount;
                let burn_msg = BankMsg::Burn {
                    amount: coins(burn_coins.amount.u128(), burn_coins.denom),
                };
                router.execute(
                    api,
                    storage,
                    block,
                    Addr::unchecked(tf_msg.sender),
                    burn_msg.into(),
                )
            }
            _ => Err(anyhow::anyhow!(
                "Unexpected exec msg {type_url} from {sender:?}",
            )),
        }
    }
}
impl Stargate for StargateMock {
    fn execute_any<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: AnyMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        self.handle_stargate_any(api, storage, router, block, sender, msg.type_url, msg.value)
    }
    fn execute_stargate<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        type_url: String,
        value: Binary,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        self.handle_stargate_any(api, storage, router, block, sender, type_url, value)
    }

    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        path: String,
        _data: Binary,
    ) -> AnyResult<Binary> {
        // map the coin types
        let mut fees: Vec<Coin> = vec![];
        for fee in self.fees.iter() {
            fees.push(Coin {
                denom: fee.denom.clone(),
                amount: fee.amount.u128().to_string(),
            });
        }

        match path.as_str() {
            "/osmosis.tokenfactory.v1beta1.Query/Params" => {
                Ok(to_json_binary(&QueryParamsResponse {
                    params: Some(Params {
                        denom_creation_fee: fees,
                        denom_creation_gas_consume: 0,
                    }),
                })?)
            }
            _ => Err(anyhow::anyhow!("Unexpected stargate query request {path}",)),
        }
    }

    fn query_grpc(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: GrpcQuery,
    ) -> AnyResult<Binary> {
        let path = request.path.clone();

        let mut fees: Vec<Coin> = vec![];
        for fee in self.fees.iter() {
            fees.push(Coin {
                denom: fee.denom.clone(),
                amount: fee.amount.u128().to_string(),
            });
        }

        match path.as_str() {
            "/osmosis.tokenfactory.v1beta1.Query/Params" => {
                let params_response = QueryParamsResponse {
                    params: Some(Params {
                        denom_creation_fee: fees,
                        denom_creation_gas_consume: 0,
                    }),
                };

                // Use EncodeMessage trait to properly encode the response as protobuf
                let encoded = QueryParamsResponse::encode(params_response);
                Ok(Binary::from(encoded))
            }
            _ => Err(anyhow::anyhow!("Unexpected grpc query request {path}",)),
        }
    }
}
