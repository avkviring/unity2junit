use std::env;
use std::fs::{read_to_string, File};
use std::path::PathBuf;

use junit_report::ReportBuilder;
use serde_xml_rs::from_str;

use crate::unity::TestResults;

pub mod converter;
pub mod unity;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("use convert input_file output_file")
    }
    let in_file = args[1].clone().into();
    let out_file = args[2].clone().into();
    convert(in_file, out_file);
}

pub fn convert(in_file: PathBuf, out_file: PathBuf) {
    let src = read_to_string(in_file).unwrap();
    let item: TestResults = from_str(&src).unwrap();

    match item.test_suite {
        None => {
            panic!("Root test suite not defined")
        }
        Some(suites) => {
            let mut report_builder = ReportBuilder::new();
            report_builder.add_testsuites(suites.into_iter().map(Into::into));
            report_builder.build();
            let out = File::create(out_file).unwrap();
            report_builder.build().write_xml(out).unwrap();
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::fs::read_to_string;
    use junit_report::ReportBuilder;
    use serde_xml_rs::from_str;

    use crate::unity::TestResults;

    #[test]
    fn test() {
        let src = read_to_string("test.xml").unwrap();
        let item: TestResults = from_str(&src).unwrap();

        let mut report_builder = ReportBuilder::new();
        report_builder.add_testsuites(item.test_suite.unwrap().into_iter().map(Into::into));
        println!("{:#?}", report_builder.build());
    }
}
