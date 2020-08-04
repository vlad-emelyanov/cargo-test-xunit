extern crate getopts;
extern crate sxd_document;

mod element;
mod parser;

use getopts::Options;
use std::env;
use std::fs::DirBuilder;
use std::fs::File;
use std::process::Command;
use sxd_document::writer::format_document;
use sxd_document::Package;

fn main() {
    // Read args and prepare vars
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "",
        "outd",
        "set output directory (defaults to current dir)",
        "PATH",
    );
    opts.optopt(
        "",
        "outf",
        "set output file name (defaults to test-results.xml)",
        "NAME",
    );
    opts.optopt(
        "",
        "args",
        "pass in custom parameters to append to 'cargo test'",
        "ARGS",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let outd_arg = matches.opt_str("outd");
    let outd_path = outd_arg.unwrap_or(".".to_owned());

    let outf_arg = matches.opt_str("outf");
    let outf_name = outf_arg.unwrap_or("test-results.xml".to_owned());

    let mut filepath: String = "".to_owned();
    filepath.push_str(&outd_path);
    filepath.push_str("/");
    filepath.push_str(&outf_name);

    let mut test_cmd = vec!["test"];
    let test_args = matches.opt_str("args").unwrap_or("".to_owned());
    if !test_args.is_empty() {
        let test_args_vec = test_args.split(" ").collect::<Vec<&str>>();
        test_cmd.extend(test_args_vec);
    }

    println!("Running tests");
    println!("cargo command: {:?}", test_cmd);

    let output = Command::new("cargo")
        .args(test_cmd)
        .output()
        .expect("failed to execute command");

    println!("Running tests -> done");

    println!("Parsing of results");

    let report = parser::parse_test_report(output);

    println!("Parsing of results -> done");

    println!("Building xunit test report");

    let package = Package::new();
    let document = element::build_xunit_report(&package, report);

    println!("Building xunit test report -> done");

    println!("Writing report");

    DirBuilder::new().recursive(true).create(outd_path).unwrap();
    let mut f = File::create(&filepath).unwrap();

    format_document(&document, &mut f)
        .ok()
        .expect("unable to output XML");

    println!("Writing report -> done");
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn it_does_not_works() {
        assert!(false);
    }
}
