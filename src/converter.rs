use std::fmt::format;
use std::time::Duration;

use crate::unity::{Failure, SuiteOrCase, TestCase, TestSuite};

impl From<TestSuite> for junit_report::TestSuite {
    fn from(source: TestSuite) -> Self {
        let mut builder = junit_report::TestSuiteBuilder::new(&source.name.unwrap_or_default());
        for item in source.results.unwrap_or_default().items {
            match item {
                SuiteOrCase::TestSuite(suite) => {
                    let converted: junit_report::TestSuite = suite.into();
                    for test_case in converted.testcases {
                        builder.add_testcase(test_case);
                    }
                }
                SuiteOrCase::TestCase(case) => {
                    let converted = case.into();
                    builder.add_testcase(converted);
                }
            };
        }
        builder.build()
    }
}

impl From<TestCase> for junit_report::TestCase {
    fn from(source: TestCase) -> Self {
        let time = source.time.unwrap_or("0.0".to_owned()).parse().unwrap();
        let duration =
            junit_report::Duration::new(Duration::from_secs_f64(time).as_secs() as i64, 0);
        let name = source.name.unwrap_or_default();
        match source.result.unwrap_or_default().as_str() {
            "Success" => junit_report::TestCase::success(name.as_str(), duration),
            "Error" => to_error("Error", source.failure, duration, &name),
            "Inconclusive" => to_error("Inconclusive", source.failure, duration, &name),
            "NotRunnable" => to_error("NotRunnable", source.failure, duration, &name),
            "Ignored" => junit_report::TestCase::skipped(name.as_str()),
            _ => junit_report::TestCase::skipped(name.as_str()),
        }
    }
}

fn to_error(
    error_type: &str,
    failure: Option<Failure>,
    duration: junit_report::Duration,
    name: &String,
) -> junit_report::TestCase {
    let failure = failure.unwrap_or_default();
    junit_report::TestCase::error(
        name.as_str(),
        duration,
        error_type,
        format!(
            "{}\n{}",
            failure.message.unwrap_or_default(),
            failure.stack_trace.unwrap_or_default()
        )
        .as_str(),
    )
}
