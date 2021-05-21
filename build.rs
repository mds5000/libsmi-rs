extern crate pkg_config;

fn main() {
    if std::env::var("DOCS_RS").is_err() {
        // do not probe for libsoxr when compiling at docs.rs
        if let Err(e) = pkg_config::probe_library("libsmi") {
            match e {
            pkg_config::Error::Failure { .. } => panic! (
                "Pkg-config failed - usually this is because libsmi development headers are not installed.\n\n\
                For Mac users using brew: brew install libsmi\n\n\
                For Debian/Ubuntu users:\n# apt-get install libsmi-dev\n\n\
                pkg_config details:\n{}",
                e
            ),
            _ => panic!("{}", e)
        }
        }
    }
}
