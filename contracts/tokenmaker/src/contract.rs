#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,Addr};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};
use crate::error::ContractError;
use crate::msg::BalanceResponse;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State,BALANCES, self};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:please_new_token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

//we have data like deps , env, info, msg
//deps includes api,queries, storag
//env includes blockinfo,transaction, contract
//info includes address of sender and amount
//and all stuff you need for getting data use all variable above
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        amount: msg.amount,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.amount.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GetToken { quantity }=>try_getToken(deps,_env, info, quantity),
    }
}

pub fn try_getToken(deps:DepsMut, env:Env,info: MessageInfo,quantity:Uint128)-> Result<Response,ContractError>{
    let ownerToken = info.sender.clone().to_string();

    let getTokenMinted = cw20::Cw20ExecuteMsg::Mint { recipient: ownerToken.to_string(), amount: quantity };
    println!("this is result in the cw20 =>{:?}", &getTokenMinted);

    let state = State{
        owner: info.sender.clone(),amount:quantity
    };
    println!("this is state =>{:?}", state);
    
    BALANCES.save(deps.storage, &info.sender.clone(), &state)?;

    Ok(
        Response::new()
    .add_attribute("method", "try_mint")
    .add_attribute("recipient", quantity)
    )
    }

    #[cfg_attr(not(feature = "library"), entry_point)]   
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance {user}=>to_binary(&try_getBalance(deps,user)?), 
    }
}

fn try_getBalance(deps: Deps, user:Addr) -> StdResult<BalanceResponse>{
    let state = BALANCES.load(deps.storage, &user)?;
    Ok(BalanceResponse{balance: state.amount,owner:state.owner})
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr};

    #[test]
    fn gettingBalance(){
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {
            amount:Uint128::from(10u128),
            owner:Addr::unchecked("Dorium").to_string(),
        };
        
        let info = mock_info("Dorium", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        let msg1 =ExecuteMsg::GetToken { quantity: Uint128::new(10) };
        let res = execute(deps.as_mut(),mock_env(), info.clone(), msg1).unwrap();

        let msg2 = InstantiateMsg {
            amount:Uint128::from(10u128),
            owner:Addr::unchecked("safeparadise").to_string(),
        };
    
        let info2 =  mock_info("safeparadise", &coins(2, "token"));
        let _res2 = instantiate(deps.as_mut(), mock_env(), info.clone(), msg2).unwrap();
        
        
        let msg3 =ExecuteMsg::GetToken { quantity: Uint128::new(40) };
        let res2 = execute(deps.as_mut(),mock_env(), info2.clone(), msg3).unwrap();

        let user1 = Addr::unchecked("Dorium");
        let user2 = Addr::unchecked("safeparadise");

        let res_getBalance1 = query(deps.as_ref(),mock_env() ,QueryMsg::GetBalance {user:user1}).unwrap();
        let value1: BalanceResponse = from_binary(&res_getBalance1).unwrap();
        println!("you can see the result of dorrium=>{:?}", value1);

        let res_getBalance2 = query(deps.as_ref(),mock_env() ,QueryMsg::GetBalance {user:user2}).unwrap();
        let value2: BalanceResponse = from_binary(&res_getBalance2).unwrap();
        println!("you can see the result of safepradise=>{:?}", value2);

    }

    #[test]
    fn tst_token(){
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {
            amount:Uint128::from(10u128),
            owner:Addr::unchecked("Dorium").to_string(),
        };
        print!("this is first");
        let info = mock_info("Dorium", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        ///////
        print!("this is secound");
        let msg =ExecuteMsg::GetToken { quantity: Uint128::new(10) };
        println!("{:?}", &msg);
        let res = execute(deps.as_mut(),mock_env(), info, msg).unwrap();
        println!("{:?}", &res.messages);
        assert_eq!(0, res.messages.len());
    }
}
