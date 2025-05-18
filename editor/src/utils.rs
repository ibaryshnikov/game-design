// Combine two &'static str within a const context.
// A replacement for concat! macro to use with
// const variables
macro_rules! combine {
    ($a:ident, $b:ident) => {{
        const TMP: [u8; $a.len() + $b.len()] = {
            let mut res = [0u8; $a.len() + $b.len()];
            let mut i = 0;
            while i < $a.len() + $b.len() {
                if i < $a.len() {
                    res[i] = $a.as_bytes()[i];
                } else {
                    res[i] = $b.as_bytes()[i - $a.len()];
                }
                i += 1;
            }
            res
        };
        unsafe { std::str::from_utf8_unchecked(&TMP) }
    }};
}

pub(crate) use combine;
