/// Initialize the logger whit the given handlers
///
/// Initialization can only happen once !
#[macro_export]
macro_rules! init_logger {
    ($($handler:expr),*) => ({
        let mut handlers: Vec<Box<dyn $crate::handlers::LogHandler>> = vec![];

        // Fill the handlers vector
        $(
            handlers.push(Box::new($handler));
        )*

        $crate::Logger::__private_init(handlers)
    })
}

/// Chain multiple filter function together
#[macro_export]
macro_rules! join_filters {
    ($($filter:expr),*) => {
        // Create the outer closure
        |metadata: &log::Metadata| {
            let mut status = true;

            $(
                status &= ($filter)(metadata);     
            )*

            status
        }
    }
}
