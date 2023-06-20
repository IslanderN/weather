use crate::contracts::{GetFullAddress};

pub struct LocationManager<TAddress: GetFullAddress> {
    addresses: Vec<TAddress>,
    chosen_address_index: Option<usize>
}

impl<TAddress: GetFullAddress> LocationManager<TAddress> {

    pub fn new() -> Self {
        LocationManager{addresses: Vec::new(), chosen_address_index:None}
    }

    pub fn store_addresses(&mut self, addresses: Vec<TAddress>) {
        self.addresses = addresses
    }

    pub fn choose_address(&mut self, index: usize) -> Result<(), String> {
        if self.addresses.is_empty() {
            Err("There's no addresses to choose".to_string())
        } else if self.addresses.len() < index {
            Err("There's less address that you want to choose".to_string())
        } else {
            self.chosen_address_index = Some(index);
            Ok(())
        }
    }

    pub fn get_chosen_address(&self) -> Result<&TAddress, String> {
        match &self.chosen_address_index {
            Some(i) => Ok(&self.addresses[*i]),
            None => Err("There's no chosen address".to_string())
        }
    }

    pub fn get_all_addresses(&self) -> &Vec<TAddress> {
        &self.addresses
    }
}