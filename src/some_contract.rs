use odra::{
    execution_error, types::Address, Mapping, OdraType, Sequence, UnwrapOrRevert, Variable,
};

#[odra::module]
pub struct SomeContract {
    name: Variable<String>,
    store: Mapping<ProposalId, Something>,
    ids_gen: Sequence<u64>,
}

pub type ProposalId = u64;

#[derive(OdraType, Debug, PartialEq, Eq)]
pub struct Something {
    id: u64,

    some_text: String,
}

impl Something {
    fn new(id: u64, some_text: String) -> Self {
        Something { id, some_text }
    }
}

#[odra::module]
impl SomeContract {
    #[odra(init)]
    pub fn init(&mut self, name: String) {
        self.name.set(name);
    }

    pub fn add_something(&mut self, some_text: String) {
        let next_id = self.ids_gen.next_value();
        self.store.set(&next_id, Something::new(next_id, some_text))
    }

    pub fn get_something(&mut self, id: ProposalId) -> Something {
        self.store.get(&id).unwrap_or_revert()
    }
}

#[cfg(test)]
mod tests {
    use hex::decode;
    use odra::{test_env, types::Address};

    use super::{SomeContractDeployer, Something};

    #[test]
    fn add_something() {
        
        let admin = test_env::get_account(0);
        test_env::set_caller(admin);

        let contract_name = String::from("test name");

        let mut contract = SomeContractDeployer::init(contract_name);

        let some_text = "test text".to_string();
        contract.add_something(some_text.clone());
        let result = contract.get_something(0);
        let expected = Something::new(0, some_text.clone());
        assert_eq!(result, expected);
    }
}
