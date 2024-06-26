contract;

abi MyContract {
    fn test_function(a: str) -> str;
}

impl MyContract for Contract {
    fn test_function(a: str) -> str {
        a
    }
}

#[test]
fn test_success() {
    let contract_id = 0xa2c21e6713a5bd5b44a11b1008bebf35c5498619c545cb37697cd44ddbdb4c73;
    let caller = abi(MyContract, contract_id);
    let result = caller.test_function("a");
    assert(result == "a")
}
