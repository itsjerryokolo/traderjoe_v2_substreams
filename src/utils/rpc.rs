use crate::abi;

use substreams_ethereum::rpc::RpcBatch;

use super::constants::DEXCANDLES_FACTORY;

pub fn get_token_data(token_address: &Vec<u8>) -> (String, String, String, String) {
    let contract_address = token_address;
    let batch_calls = RpcBatch::new();
    let responses = batch_calls
        .add(abi::erc20::functions::Name {}, contract_address.clone())
        .add(abi::erc20::functions::Symbol {}, contract_address.clone())
        .add(abi::erc20::functions::Decimals {}, contract_address.clone())
        .add(
            abi::erc20::functions::TotalSupply {},
            contract_address.clone(),
        )
        .execute()
        .unwrap()
        .responses;

    let name = match RpcBatch::decode::<_, abi::erc20::functions::Name>(&responses[0]) {
        Some(contract_name) => contract_name.to_string(),
        None => "Name Call Reverted".to_string(),
    };
    let symbol = match RpcBatch::decode::<_, abi::erc20::functions::Symbol>(&responses[1]) {
        Some(contract_symbol) => contract_symbol.to_string(),
        None => "Name Call Reverted".to_string(),
    };
    let decimals = match RpcBatch::decode::<_, abi::erc20::functions::Decimals>(&responses[2]) {
        Some(contract_decimals) => contract_decimals.to_string(),
        None => "0".to_string(),
    };
    let total_supply =
        match RpcBatch::decode::<_, abi::erc20::functions::TotalSupply>(&responses[3]) {
            Some(contract_total_supply) => contract_total_supply.to_string(),
            None => "0".to_string(),
        };

    let tup: (String, String, String, String) = (name, symbol, decimals, total_supply);
    tup
}

pub fn get_bin_step() -> String {
    let fee_parameters = abi::lb_pair::functions::FeeParameters {};
    let fee_parameters = fee_parameters.call(DEXCANDLES_FACTORY.to_vec()).unwrap();

    fee_parameters.0.to_string()
}
