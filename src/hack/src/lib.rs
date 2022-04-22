use ic_cdk_macros::*;



#[query]
fn get_count() -> u32 {
    let state: &State = ic_cdk::storage::get();
    state.count
}

#[update]
fn count() -> u32 {
    let state: &mut State = ic_cdk::storage::get_mut();
    state.count += 1;
    state.count
}

#[update]
fn reset() {
    let state: &mut State = ic_cdk::storage::get_mut();
    std::mem::take(state);
}

#[derive(Default)]
struct State {
    count: u32,
}
