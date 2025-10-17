// Minimal native library exposed to the IntelliJ plugin via FFI.
// This is a placeholder until native APIs are defined.

#[no_mangle]
pub extern "C" fn aurion_native_version() -> *const u8 {
    b"aurion-native v0.1\0".as_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_ptr_not_null() {
        let p = aurion_native_version();
        assert!(!p.is_null());
    }
}

