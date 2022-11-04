use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

// 1. Estructura principal

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
// se define una estructura pública "KeyValue"
pub struct KeyValue {
    pairs: UnorderedMap<String, String>,
}

// 2. Implementación

impl Default for KeyValue {
    fn default() -> Self {
        Self {
            pairs: UnorderedMap::new(b"r".to_vec())
        }
    }
}

// 3. Lógica de la estructura "KeyValue"

#[near_bindgen]
impl KeyValue {
    //método para crear o actualizar un par de valores
    pub fn create_update(&mut self, k: String, v: String) {
        // mensaje para el log
        env::log(b"created or updated");
        // método insert para crear el par de valores
        self.pairs.insert(&k, &v);
    }
    //método para leer un valor existente
    pub fn read(&self, k: String) -> Option<String> {
        // mensaje para el log
        env::log(b"read");
        // método get, que retorna un valor
        return self.pairs.get(&k);
    }
    //método para borrar un valor existente
    pub fn delete(&mut self, k: String) {
        // mensaje para el log
        env::log(b"delete");
        // método remove, que elimina un valor existente
        self.pairs.remove(&k);
    }
}

// 4. Pruebas

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    // Probando la creación y lectura de un valor
    #[test]
    fn create_read_pair() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = KeyValue::default();
        contract.create_update("first_key".to_string(), "hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.read("first_key".to_string()).unwrap()
        );
    }

    // Prueba de ambiente simulando que el el valor no existe

    #[test]
    fn read_nonexistent_pair() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = KeyValue::default();
        assert_eq!(None, contract.read("first_key".to_string()));
    }

}