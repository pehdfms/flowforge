use enum_dispatch::enum_dispatch;

pub struct ActionStep {
    name: Option<String>,
    uses: String,
    with: Option<String>,
}

impl ActionStep {
    pub fn new(uses: String) -> Self {
        Self {
            name: None,
            uses,
            with: None,
        }
    }
}

pub struct RunStep {
    name: Option<String>,
    command: String,
}

impl RunStep {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            name: None,
            command: command.into(),
        }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }
}

#[enum_dispatch]
trait JobStep {}

#[enum_dispatch(JobStep)]
pub enum Step {
    ActionStep,
    RunStep,
}
