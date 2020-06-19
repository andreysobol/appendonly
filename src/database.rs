use blake2::{Blake2s, Digest};
use hex_literal::hex;
use digest::generic_array::GenericArray;
use digest::generic_array::typenum::{U32};

pub struct State<'a> {
    data: Vec<&'a Vec<u8>>,
    hash: digest::generic_array::GenericArray<u8, U32>,
}

pub struct GenesisSeed<'a> {
    seed: &'a Vec<u8>,
}

pub struct StateTransition<'a> {
    data: &'a Vec<u8>,
    hash: digest::generic_array::GenericArray<u8, U32>,
}

fn blake2s_hash(data: &Vec<u8>) -> digest::generic_array::GenericArray<u8, U32>{
    let mut hasher = Blake2s::new();
    hasher.update(data);
    let res = hasher.finalize();
    res
}

pub fn create_transition(data: &Vec<u8>, prevhash: digest::generic_array::GenericArray<u8, U32>) -> StateTransition {
    let mut hasher = Blake2s::new();
    hasher.update(data);
    let res = hasher.finalize();
    
    hasher = Blake2s::new();
    hasher.update([res.to_vec(), prevhash.to_vec()].concat());
    let hash = hasher.finalize();

    StateTransition {
        data: data,
        hash: hash,
    }
}

pub fn apply_transition<'a>(prevstate: State<'a>, state_transition: StateTransition<'a>) -> State<'a> {
    let mut data_list = prevstate.data;
    data_list.push(state_transition.data);

    State {
        data: data_list,
        hash: state_transition.hash,
    }
}

pub fn verify_hash_transition<'a>(state: State<'a>, state_transition: StateTransition<'a>) -> bool {
    let datahash = blake2s_hash(state_transition.data);
    let prevhash = state.hash;
    let data = [datahash.to_vec(), prevhash.to_vec()].concat();
    let hash = blake2s_hash(&data);
    hash == state_transition.hash
}

pub fn create_inital_state<'a>(genesis_seed: GenesisSeed<'a>) -> State {
    let data = Vec::new();
    State {
        data: data,
        hash: blake2s_hash(genesis_seed.seed),
    }
}

#[test]
fn create_inital_state_test() {
    let genesis_seed:GenesisSeed = GenesisSeed {
        seed: &b"hello".to_vec(),
    };
    let inital_state = create_inital_state(genesis_seed);
    assert_eq!(inital_state.data.len(), 0);
    assert_eq!(inital_state.hash[..], hex!("19213bacc58dee6dbde3ceb9a47cbb330b3d86f8cca8997eb00be456f140ca25")[..]);
}

#[test]
fn create_transition_test() {
    let data = &b"justdata".to_vec();
    let prevhash = GenericArray::from_slice(&hex!("19213bacc58dee6dbde3ceb9a47cbb330b3d86f8cca8997eb00be456f140ca25")[..]);
    let state_transiation = create_transition(data, *prevhash);
    assert_eq!(
        state_transiation.hash[..],
        hex!("53b67e5bec3482a70dbe3970fc5ba5bcd5622c269d5490f073ec7662072c4579")[..],
    );
}

#[test]
fn apply_transition_test() {
    let genesis_seed:GenesisSeed = GenesisSeed {
        seed: &b"hello".to_vec(),
    };
    let inital_state = create_inital_state(genesis_seed);

    let data = &b"justdata".to_vec();
    let prevhash = GenericArray::from_slice(&hex!("19213bacc58dee6dbde3ceb9a47cbb330b3d86f8cca8997eb00be456f140ca25")[..]);
    let state_transiation = create_transition(data, *prevhash);

    let newstate = apply_transition(inital_state, state_transiation);

    assert_eq!(
        newstate.hash[..],
        hex!("53b67e5bec3482a70dbe3970fc5ba5bcd5622c269d5490f073ec7662072c4579")[..],
    );

    let mut datavec: Vec<&Vec<u8>> = Vec::new();
    let el = &b"justdata".to_vec();
    datavec.push(el);
    assert_eq!(
        newstate.data,
        datavec,
    );
}