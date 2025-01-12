use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoadTests {
    pub load_zenario: Vec<LoadZenario>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)] // Allows handling `parallel_group` and regular tests
pub enum LoadZenario {
    Test(Test),
    ParallelGroup { parallel_group: Vec<Test> },
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub name: String,
    #[serde(rename = "type")]
    pub test_type: String,
    pub duration: Option<u64>,
    pub operation: Option<String>,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub data_size: Option<String>,
}

pub fn load_zenario(file_path: &str) -> Result<LoadTests, Box<dyn std::error::Error>> {
    let yaml_content = std::fs::read_to_string(file_path)?;
    Ok(serde_yaml::from_str(&yaml_content)?)
}
