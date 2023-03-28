// SPDX-License-Identifier: MIT

use ink_lang::contract;
use ink_prelude::vec::Vec;
use ink_storage::{
    collections::{HashMap as StorageHashMap, Vec as StorageVec},
    lazy::Lazy,
    traits::{PackedLayout, SpreadLayout},
};

type TokenId = u32;
type Balance = u128;
type ArtistId = u32;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Token {
    Owned {
        price: Balance,
        owner: AccountId,
    },
    ForSale {
        price: Balance,
        artist: ArtistId,
    },
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Artist {
    name: Vec<u8>,
    account_id: AccountId,
}

#[contract]
mod psp34 {
    use super::*;

    #[ink(storage)]
    pub struct PSP34 {
        tokens: StorageHashMap<TokenId, Token>,
        artists: StorageHashMap<ArtistId, Artist>,
        next_artist_id: Lazy<ArtistId>,
    }

    impl PSP34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tokens: StorageHashMap::new(),
                artists: StorageHashMap::new(),
                next_artist_id: Lazy::new(|| 0),
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, id: TokenId, price: Balance, artist_id: ArtistId) {
            let caller = self.env().caller();
            let token = Token::Owned {
                price,
                owner: caller,
            };
            self.tokens.insert(id, token);
            self.set_token_artist(id, artist_id);
        }

        #[ink(message)]
        pub fn transfer(&mut self, id: TokenId, to: AccountId) {
            let caller = self.env().caller();
            let token = self.tokens.get(&id).unwrap();

            match token {
                Token::Owned { owner, .. } if *owner == caller => {
                    self.tokens.insert(id, Token::Owned {
                        price: 0,
                        owner: to,
                    });
                }
                _ => ink_env::debug_println!("Transfer not allowed"),
            }
        }

        #[ink(message)]
        pub fn set_price(&mut self, id: TokenId, price: Balance) {
            let caller = self.env().caller();
            let token = self.tokens.get(&id).unwrap();

            match token {
                Token::Owned { owner, .. } if *owner == caller => {
                    self.tokens.insert(id, Token::Owned {
                        price,
                        owner: *owner,
                    });
                }
                _ => ink_env::debug_println!("Set price not allowed"),
            }
        }

        #[ink(message)]
        pub fn buy(&mut self, id: TokenId) {
            let caller = self.env().caller();
            let token = self.tokens.get(&id).unwrap();

            match token {
                Token::ForSale { price, artist } => {
                    let artist_account = self.artist_account(*artist);

                    let balance = self.env().balance();
                    let value = self.env().transferred_balance();
                    assert_eq!(value, *price, "Incorrect price");

                    let artist_share = value / 10;
                    let buyer_share = value - artist_share;

                    // Transfer the token to
                                    // the buyer
                self.tokens.insert(id, Token::Owned {
                    price: 0,
                    owner: caller,
                });

                // Transfer the payment to the artist and the buyer
                let _ = artist_account
                    .transfer(artist_share)
                    .expect("Transfer to artist failed");
                let _ = caller
                    .transfer(buyer_share)
                    .expect("Transfer to buyer failed");
                }
            _ => ink_env::debug_println!("Buy not allowed"),
            }
        }

        #[ink(message)]
        pub fn set_artist(&mut self, id: ArtistId, name: Vec<u8>, account_id: AccountId) {
            let caller = self.env().caller();
            assert!(caller == account_id, "Only the artist can set their details");

            let artist = Artist { name, account_id };
            self.artists.insert(id, artist);
        }

        #[ink(message)]
        pub fn artist_account(&self, id: ArtistId) -> AccountId {
            let artist = self.artists.get(&id).unwrap();
            artist.account_id
        }

        #[ink(message)]
        pub fn get_token(&self, id: TokenId) -> Option<Token> {
            self.tokens.get(&id).cloned()
        }

        #[ink(message)]
        pub fn set_token_artist(&mut self, id: TokenId, artist_id: ArtistId) {
            let token = self.tokens.get(&id).unwrap();
            match token {
                Token::Owned { .. } => {
                    ink_env::debug_println!("Only for sale tokens can be associated with an artist");
                }
                Token::ForSale { price, .. } => {
                    self.tokens.insert(id, Token::ForSale {
                        price: *price,
                        artist: artist_id,
                    });
                }
            }
        }

        #[ink(message)]
        pub fn next_artist_id(&self) -> ArtistId {
            *self.next_artist_id
        }

        #[ink(message)]
        pub fn increment_artist_id(&mut self) {
            *self.next_artist_id += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint() {
        let mut psp34 = PSP34::new();
        let caller = account_id::<ink_env::DefaultEnvironment>()
            .unwrap_or(Default::default());

        psp34.mint(0, 100, 0);
        let token = psp34.get_token(0).unwrap();
        assert_eq!(token, Token::Owned {
            price: 100,
            owner: caller,
        });
    }

    #[test]
    fn test_transfer() {
        let mut psp34 = PSP34::new();
        let caller1 = account_id::<ink_env::DefaultEnvironment>()
            .unwrap_or(Default::default());
        let caller2 = AccountId::from([0x2; 32]);

        psp34.mint(0, 100, 0);

        // Try to transfer token to another account
        psp34.transfer(0, caller2);
        let token = psp34.get_token(0).unwrap();
        assert_eq!(token, Token::Owned {
            price: 0,
            owner: caller2,
        });

        // Try to transfer token from another account (should fail)
        psp34.env().test_set_caller(caller2);
        psp34.transfer(0, caller1);
        let token = psp34.get_token(0).unwrap();
        assert_eq!(token, Token::Owned {
            price: 0,
            owner: caller2,
        });
    }

    #[test]
    fn test_set_artist() {
        let mut psp34 = PSP34::new();
        let caller = account_id::<ink_env::DefaultEnvironment>()
            .unwrap_or(Default::default());

        // Set artist details
        psp34.set_artist(0, b"Artist 1".to_vec(), caller);

        // Check artist details
        let artist_id = psp34.next_artist_id();
        assert_eq!(psp34.artist_account(artist_id - 1), caller);
    }
    

    #[test]
    fn test_buy() {
        let mut psp34 = PSP34::new();
        let caller1 = account_id::<ink_env::DefaultEnvironment>()
            .unwrap_or(Default::default());
        let caller2 = AccountId::from([0x2; 32]);

        // Mint a token and set it for sale
        psp34.mint(0, 100, 0);
        psp34.set_token_artist(0, 0);
        psp34.set_for_sale(0, 50);

        // Try to buy token as owner (should fail)
        psp34.buy(0);
        let token = psp34.get_token(0).unwrap();
        assert_eq!(token, Token::ForSale {
            price: 50,
            artist: 0,
        });

        // Try to buy token as non-owner
        psp34.env().test_set_caller(caller2);
        psp34.buy(0);
        let token = psp34.get_token(0).unwrap();
        assert_eq!(token, Token::Owned {
            price: 0,
            owner: caller2,
        });
    }
}

