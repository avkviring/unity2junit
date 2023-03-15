use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TestResults {
    #[serde(rename = "test-suite")]
    pub test_suite: Option<TestSuite>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TestSuite {
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub executed: Option<String>,
    pub result: Option<String>,
    pub success: Option<String>,
    pub asserts: Option<u64>,
    pub results: Results,
    pub reason: Option<Reason>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Results {
    #[serde(rename = "$value")]
    pub items: Vec<SuiteOrCase>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SuiteOrCase {
    #[serde(rename = "test-suite")]
    TestSuite(TestSuite),
    #[serde(rename = "test-case")]
    TestCase(TestCase),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TestCase {
    pub name: Option<String>,
    pub executed: Option<String>,
    pub result: Option<String>,
    pub success: Option<String>,
    pub time: Option<String>,
    pub asserts: Option<u64>,
    pub failure: Option<Failure>,
    pub reason: Option<Reason>,
    pub properties: Option<Properties>,
    pub categories: Option<Categories>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Properties {
    #[serde(rename = "$value")]
    pub items: Vec<Property>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Categories {
    #[serde(rename = "$value")]
    pub items: Vec<Category>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub name: String,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Failure {
    pub message: Option<String>,
    #[serde(rename = "stack-trace")]
    pub stack_trace: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Reason {
    pub message: Option<String>,
}
