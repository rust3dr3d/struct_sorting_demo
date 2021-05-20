#[allow(dead_code)]
pub mod core{     

    #[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
    pub struct SmartContract{
        pub first_name: &'static str,
        pub last_name: &'static str,
        pub age: u8,
        pub coins: u32,
    }

    impl SmartContract{
        //Initialize a new SmartContract
        pub fn new(first_name: &'static str, last_name: &'static str, age: u8, coins:u32) -> SmartContract{
            SmartContract{
                first_name,
                last_name,
                age,
                coins
            }
        }

        //Set the number of coins owned by a contract
        pub fn set_coins(&mut self, coins:u32){
            self.coins = coins;
        }
    }


    // Contract pool contains a set of SmartContracts in contracts Vec
    pub struct ContractsPool{
        contracts: Vec<SmartContract>, 
    }

    impl ContractsPool{
        //Creates a new Contracts pool, initializes Vec to default
        pub fn new() -> ContractsPool{
            ContractsPool{
                contracts: Vec::new()
            }
        }

        //Prints all the SmartContracts in the pool
        pub fn print_all(&self){
            for contract in self.contracts.iter(){
                println!("{:#?}", contract);
            }
        }

        //Add a new SmartContract
        pub fn add_contract(&mut self, contract: SmartContract){
            self.contracts.push(contract);
        }

        //Auto Total Ordering
        //NOTE: Eq, PartialEq, PartialOrd and Ord Traits derived in the SmartContracts struct
        //We are using the easiest way to sort via deriving above Traits
        //However if you want to, you can implement these traits separately
        //https://doc.rust-lang.org/std/cmp/trait.Ord.html#how-can-i-implement-ord
        pub fn sort(&mut self){
            self.contracts.sort();
        }


        // Sort the SmartContracts in the pool by First Name Ascending
        pub fn sort_by_first_asc(&mut self){
            self.contracts.sort_by(|a, b| a.first_name.cmp(&b.first_name));
        }

        // Sort the SmartContracts in the pool by First Name Defending
        // Note the difference in the cmp method
        pub fn sort_by_first_desc(&mut self){
            self.contracts.sort_by(|a, b| b.first_name.cmp(&a.first_name));
        }

        //Rest should be self-explanatory
        pub fn sort_by_last_asc(&mut self){
            self.contracts.sort_by(|a, b| a.last_name.cmp(&b.last_name));
        }

        pub fn sort_by_last_desc(&mut self){
            self.contracts.sort_by(|a, b| b.last_name.cmp(&a.last_name));
        }

        pub fn sort_by_age_asc(&mut self){
            self.contracts.sort_by(|a, b| a.age.cmp(&b.age));
        }

        pub fn sort_by_age_desc(&mut self){
            self.contracts.sort_by(|a, b| b.age.cmp(&a.age));
        }

        pub fn sort_by_coins_asc(&mut self){
            self.contracts.sort_by(|a, b| a.coins.cmp(&b.coins));
        }

        pub fn sort_by_coins_desc(&mut self){
            self.contracts.sort_by(|a, b| b.coins.cmp(&a.coins));
        }
    }

}