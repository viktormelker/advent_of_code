use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseDiagnosticDataError;
impl fmt::Display for ParseDiagnosticDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse diagnostic data")
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticData {
    pub gamma_rate: u32,
    pub epsilon_rate: u32,
}

pub trait GetPower {
    fn get_power(&self) -> u32;
}

impl GetPower for DiagnosticData {
    fn get_power(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}

impl FromStr for DiagnosticData {
    type Err = ParseDiagnosticDataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Implement

        let result = DiagnosticData{gamma_rate: 0, epsilon_rate: 0};
        Ok(result)
    }
}
