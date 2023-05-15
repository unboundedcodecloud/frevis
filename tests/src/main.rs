use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup(){
        assert_eq!("frevis", env!("CARGO_PKG_NAME"));
    }
}

// Test plan: std lib, io, add dependency (cyclone), template types (haskell)