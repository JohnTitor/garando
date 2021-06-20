// FIXME garando
macro_rules! __register_diagnostic {
    ($code:tt, $description:tt) => {};
    ($code:tt) => {};
}

// FIXME garando
macro_rules! __diagnostic_used {
    ($code:tt) => {};
}

#[macro_export]
macro_rules! register_diagnostic {
    ($code:tt, $description:tt) => {
        __register_diagnostic! { $code, $description }
    };
    ($code:tt) => {
        __register_diagnostic! { $code }
    };
}

#[macro_export]
macro_rules! span_err {
    ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
        __diagnostic_used!($code);
        $session.span_err_with_code($span, &format!($($message)*), stringify!($code))
    })
}

#[macro_export]
macro_rules! struct_span_err {
    ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
        __diagnostic_used!($code);
        $session.struct_span_err_with_code($span, &format!($($message)*), stringify!($code))
    })
}

#[macro_export]
macro_rules! register_diagnostics {
    ($($code:tt),*) => (
        $(register_diagnostic! { $code })*
    );
    ($($code:tt),*,) => (
        $(register_diagnostic! { $code })*
    )
}

#[macro_export]
macro_rules! register_long_diagnostics {
    ($($code:tt: $description:tt),*) => (
        $(register_diagnostic! { $code, $description })*
    );
    ($($code:tt: $description:tt),*,) => (
        $(register_diagnostic! { $code, $description })*
    )
}
