/// Add log handlers to the global logger
#[macro_export]
macro_rules! log_add_handlers {
    ($($handler:expr),*) => {
        $(
            $crate::Logger::__private_add_handler(Box::new($handler));
        )*
    };
}

#[macro_export]
macro_rules! log {
    (source: $source:expr, $level:expr, $($fmt_arg:tt)+) => ({
        if let Some(LEVEL) = COMPILE_LOG_LEVEL {
            if $level >= LEVEL {
                $crate::Logger::__private_log($level, $source, format_args!($($fmt_arg)+))
            }
        }
    });

    ($level:expr, $($fmt_arg:tt)+) => ({
         if let Some(LEVEL) = COMPILE_LOG_LEVEL {
            if $level >= LEVEL {
                $crate::Logger::__private_log($level, module_path!(), format_args!($($fmt_arg)+))
            }
        }          
    });
}

#[macro_export]
macro_rules! trace {
    (source: $source:expr, ($fmt_arg:tt)+) => (log!($source, $crate::LogLevel::Trace, $($fmt_arg)+));

    ($($fmt_arg:tt)+) => (log!($crate::LogLevel::Trace, $($fmt_arg)+));
}

#[macro_export]
macro_rules! debug {
    (source: $source:expr, ($fmt_arg:tt)+) => (log!($source, $crate::LogLevel::Debug, $($fmt_arg)+));

    ($($fmt_arg:tt)+) => (log!($crate::LogLevel::Debug, $($fmt_arg)+));
}

#[macro_export]
macro_rules! info {
    (source: $source:expr, ($fmt_arg:tt)+) => (log!($source, $crate::LogLevel::Info, $($fmt_arg)+));

    ($($fmt_arg:tt)+) => (log!($crate::LogLevel::Info, $($fmt_arg)+));
}

#[macro_export]
macro_rules! warn {
    (source: $source:expr, ($fmt_arg:tt)+) => (log!($source, $crate::LogLevel::Warning, $($fmt_arg)+));

    ($($fmt_arg:tt)+) => (log!($crate::LogLevel::Warning, $($fmt_arg)+));
}

#[macro_export]
macro_rules! error {
    (source: $source:expr, ($fmt_arg:tt)+) => (log!($source, $crate::LogLevel::Error, $($fmt_arg)+));

    ($($fmt_arg:tt)+) => (log!($crate::LogLevel::Error, $($fmt_arg)+));
}

#[macro_export]
macro_rules! fatal {
    (source: $source:expr, ($fmt_arg:tt)+) => (log!($source, $crate::LogLevel::Fatal, $($fmt_arg)+));

    ($($fmt_arg:tt)+) => (log!($crate::LogLevel::Fatal, $($fmt_arg)+));
}
