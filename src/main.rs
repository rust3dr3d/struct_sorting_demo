//Import the contracts module
mod contract;
use contract::core::{ContractsPool, SmartContract};

//Create some contracts to test. I'm using Jurassic Park character names :)
fn create_contracts(pool: &mut ContractsPool){
    
    pool.add_contract(
        SmartContract{
            first_name: "John",
            last_name: "Hammond",
            age: 84,
            coins: 1000
        }
    );

    pool.add_contract(
        SmartContract{
            first_name: "Alan",
            last_name: "Grant",
            age: 35,
            coins: 200
        }
    );

    pool.add_contract(
        SmartContract{
            first_name: "Ellie",
            last_name: "Sattler",
            age: 30,
            coins: 300
        }
    );

    pool.add_contract(
        SmartContract{
            first_name: "Ian",
            last_name: "Malcom",
            age: 36,
            coins: 450
        }
    );
}


fn main() {
    let mut pool = ContractsPool::new();

    create_contracts(&mut pool);

    //Test your sort functions here...
    //You'll have to call the pool.print_all() after each sort function
    pool.sort_by_coins_asc();

    pool.print_all();
}
