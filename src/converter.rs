use std::fmt::format;
use std::time::Duration;

use crate::unity::{TestCase, TestSuite};

impl From<TestSuite> for junit_report::TestSuite {
    fn from(source: TestSuite) -> Self {
        let mut builder = junit_report::TestSuiteBuilder::new(source.fullname.as_str());
        for item in source.test_suite.unwrap_or_default() {
            let converted: junit_report::TestSuite = item.into();
            for test_case in converted.testcases {
                builder.add_testcase(test_case);
            }
        }
        for item in source.test_case.unwrap_or_default() {
            let converted = item.into();
            builder.add_testcase(converted);
        }
        builder.build()
    }
}

impl From<TestCase> for junit_report::TestCase {
    fn from(source: TestCase) -> Self {
        junit_report::TestCase {
            name: source.name.into(),
            time: junit_report::Duration::default(),
            result: if source.result == "Passed" {
                junit_report::TestResult::Success
            } else {
                junit_report::TestResult::Error {
                    type_: source.result,
                    message: source.output.unwrap_or_default(),
                }
            },
            classname: Some(source.classname),
            filepath: None,
            system_out: None,
            system_err: None,
        }
    }
}
