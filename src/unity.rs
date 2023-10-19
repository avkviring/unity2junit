use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TestResults {
    #[serde(rename = "$value")]
    pub test_suite: Option<Vec<TestSuite>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TestItem {
    TestSuite(TestSuite),
    TestCase(TestCase),
    Properties(Properties),
    Failure(Failure)
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Properties {

}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TestSuite {
    pub r#type: Option<String>,
    pub fullname: String,
    pub result: String,
    pub total: u64,
    pub passed: u64,
    pub skipped: u64,
    #[serde(rename = "$value")]
    pub items: Vec<TestItem>,
}



//   <test-case id="1127" name="AttackRequestTest" fullname="DMP.Battle.Client.Test.CombatActions.CombatActionSystemTest.AttackRequestTest" methodname="AttackRequestTest" classname="DMP.Battle.Client.Test.CombatActions.CombatActionSystemTest" runstate="Runnable" seed="1588844276" result="Passed" start-time="2023-03-15 11:11:59Z" end-time="2023-03-15 11:11:59Z" duration="0.132441" asserts="0">
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TestCase {
    pub name: String,
    pub classname: String,
    pub methodname: String,
    pub fullname: String,
    pub result: String,
    pub output: Option<String>,
    pub failure: Option<Failure>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Failure {
    pub message: Option<String>,
    #[serde(rename = "stack-trace")]
    pub stacktrace: Option<String>,
}
