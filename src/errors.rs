// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        UnableToConvert() {
            display("unable to convert");
        }
    }
}
