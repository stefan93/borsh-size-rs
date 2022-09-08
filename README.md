# borsh-size-rs
Calculate size of struct serialized with borsh

## Example

```rust

#[derive(BorshSerialize, BorshDeserialize, BorshSize)]
pub struct VotingAccount {
    pub uid: String,
    pub voting_name: String,
    pub voting_options: Vec<VotingOption>,
}

voting_account = VotingAccount {
    uid: voting_uid,
    voting_name: voting_name,
    voting_options: Vec::new(),
};

// get size of borsh serilized data
let size = voting_account.calculate_borsh_size();

// now use size for something usefull like creating account on solace chain


```
