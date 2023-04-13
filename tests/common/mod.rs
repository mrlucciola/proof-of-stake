// imports
use std::{fs::File, io::BufReader};
// local
use posbc::ledger::{
    general::{PbKey, KP},
    txn::{Txn, TxnType},
    wallet::Wallet,
};
// test
use constants::*;
// mods
pub mod constants;
pub mod fxns;

/// secp-2-ed: from get_user_info
fn create_keypair_from_file(filepath: &String) -> KP {
    if !filepath.contains("_ed25519") {
        panic!("Filename must have _ed25519 in it: {}", filepath);
    };

    let f = File::open(filepath).unwrap();
    let reader = BufReader::new(f);
    let key_json: Vec<u8> = serde_json::from_reader(reader).unwrap();

    // open with ed 25519 lib
    let kp = KP::from_bytes(&key_json).unwrap();

    kp
}

pub struct UserInfo {
    pub kp: KP,
    pub wallet: Wallet,
    pub filepath: String,
}
impl UserInfo {
    pub fn pbkey(&self) -> PbKey {
        self.kp.public.into()
    }
}
/// in progress secp-2-ed
fn get_user_info(key_str: &String) -> UserInfo {
    let kp = create_keypair_from_file(key_str);
    let wallet = Wallet::new_from_kp(kp);
    let kp = create_keypair_from_file(key_str);
    UserInfo {
        kp,
        wallet,
        filepath: key_str.to_owned(),
    }
}
pub struct UsersInfo {
    pub main: UserInfo,
    pub send: UserInfo,
    pub recv: UserInfo,
    pub test1: UserInfo,
    pub test2: UserInfo,
    pub test3: UserInfo,
}
pub fn init_users() -> UsersInfo {
    UsersInfo {
        main: get_user_info(&KEYPAIR_MAIN.to_string()),
        send: get_user_info(&KEYPAIR_SEND.to_string()),
        recv: get_user_info(&KEYPAIR_RECV.to_string()),
        test1: get_user_info(&KEYPAIR_TEST1.to_string()),
        test2: get_user_info(&KEYPAIR_TEST2.to_string()),
        test3: get_user_info(&KEYPAIR_TEST3.to_string()),
    }
}

pub fn init_send_recv() -> (UserInfo, UserInfo) {
    let users = init_users();

    (users.send, users.recv)
}

pub fn create_transfer_txn_default() -> Txn {
    let (send, recv) = init_send_recv();

    // turn the raw txn into message
    let mut txn = Txn::new(send.pbkey(), recv.pbkey(), 100, TxnType::Transfer);
    txn.system_time = 1669699785826;

    txn.set_id();

    txn
}

/// Automatically uses the default send and recv accounts
pub fn create_transfer_txn(amt_to_transfer: u128) -> Txn {
    let (send, recv) = init_send_recv();

    // turn the raw txn into message
    let mut txn = Txn::new(
        send.pbkey(),
        recv.pbkey(),
        amt_to_transfer,
        TxnType::Transfer,
    );
    txn.system_time = 1669699785826;

    txn.set_id();

    txn
}

pub fn create_transfer_txn_manual(send: UserInfo, recv: UserInfo, amt_to_transfer: u128) -> Txn {
    // turn the raw txn into message
    let mut txn = Txn::new(
        send.pbkey(),
        recv.pbkey(),
        amt_to_transfer,
        TxnType::Transfer,
    );
    txn.system_time = 1669699785826;

    txn.set_id();

    txn
}
