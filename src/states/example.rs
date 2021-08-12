use std::sync::Mutex;

//Static state
#[derive(Clone)] //cloneはコストが掛かるらしい
pub struct AppState {
    pub app_name: String,
    pub author: String
}
//Dynamic state
pub struct MutableMessageState {
    pub text: Mutex<String>
}
//Dynamic state
#[derive(Default, Debug)]
pub struct ImmutableIntState {
    pub random_uint: usize
}

