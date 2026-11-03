use askama::Template;
#[derive(Template)]
#[template(path = "main.ts.jinja", escape = "none")]   // ← "this struct uses main.ts.jinja"
pub struct MainTemplate {
    validation: bool,   // ← the {% if validation %} in the file
    helmet: bool,       // ← the {% if helmet %} in the file
}

impl  MainTemplate {
    pub fn new(validation:bool, helmet:bool) -> Self {
        Self { validation, helmet }
    }
}
