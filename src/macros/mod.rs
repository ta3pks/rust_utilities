#[macro_export]
macro_rules! if_else {
    ($cond:expr, $if:expr, $else:expr) => {
        if $cond {
            $if
        } else {
            $else
        }
    };
}

#[macro_export]
macro_rules! match_if {
    ($cond1:expr => $block:expr; $($conds:expr => $blocks:expr;)* $(_ => $block3:expr$(;)?)?) => {
        if $cond1 {
            $block
        } $(
            else if $conds{
                $blocks
            }
        )*
        $(
            else {
                $block3
            }
        )?
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_if_else() {
        let a = 1;
        let b = 2;
        let c = if_else!(a > b, a, b);
        assert_eq!(c, 2);
    }

    #[test]
    fn test_match_if() {
        let a = 1;
        let b = 2;
        let c = match_if!(
            a > b => a;
            a < b => b;
            _ => 0
        );
        assert_eq!(c, b);
    }
}
