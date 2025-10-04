use once_cell::sync::Lazy;
use state::KEY_STATE;

use crate::state;

pub fn init() {
    Lazy::force(&KEY_STATE);
}
