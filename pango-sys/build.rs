extern crate pkg_config;

fn main() {
    match pkg_config::find_library("pango") {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    };
}