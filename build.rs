extern crate rusoto_codegen;

use std::env;
use std::fs::copy;
use std::path::Path;

use rusoto_codegen::{Service, generate};

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir);

    let services = vec![
        Service::new("dynamodb", "2012-08-10"),
        Service::new("kms", "2014-11-01"),
        Service::new("ecs", "2014-11-13"),
        Service::new("sqs", "2012-11-05"),
    ];

    for service in services {
        generate(service, out_path);
    }
}
