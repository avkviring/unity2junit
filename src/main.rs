use std::env;
use std::fs::{read_to_string, File};
use std::io::Sink;
use std::path::PathBuf;

use junit_report::{
    datetime, Duration, ReportBuilder, TestCase, TestCaseBuilder, TestSuiteBuilder,
};
use serde_xml_rs::from_str;

use crate::unity::{TestResults, TestSuite};

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

    let result = match item.test_suite {
        None => {
            panic!("Root test suite not defined")
        }
        Some(suite) => {
            let report = ReportBuilder::new()
                .add_testsuite(From::from(suite))
                .build();
            let mut out = File::create(out_file).unwrap();
            report.write_xml(out).unwrap();
        }
    };
}

#[cfg(test)]
pub mod test {
    use std::fs::read_to_string;

    use junit_report::{
        datetime, Duration, ReportBuilder, TestCase, TestCaseBuilder, TestSuiteBuilder,
    };
    use serde_xml_rs::from_str;

    use crate::unity::{TestResults, TestSuite};

    // #[test]
    // fn test() {
    //     let src = read_to_string("test.xml").unwrap();
    //     let item: TestResults = serde_xml_rs::from_str(&src).unwrap();
    //
    //     let result = match item.test_suite {
    //         None => None,
    //         Some(suite) => Some(
    //             ReportBuilder::new()
    //                 .add_testsuite(From::from(suite))
    //                 .build(),
    //         ),
    //     };
    //
    //     println!("{:#?}", result);
    // }
}
