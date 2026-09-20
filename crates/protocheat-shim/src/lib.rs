//! Observability only shim.
//! It logs that it was loaded. It never hooks, patches, or alters
//! anti-cheat behavior. See SECURITY.md.

#[no_mangle]
pub extern "C" fn protocheat_shim_loaded() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shim_reports_loaded() {
        assert_eq!(protocheat_shim_loaded(), 1);
    }
}
