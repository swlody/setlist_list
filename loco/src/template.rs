use crate::Result;

pub fn render_string(
    env: &minijinja::Environment<'_>,
    jinja_template: &str,
    locals: &serde_json::Value,
) -> Result<String> {
    let tmpl = env.template_from_named_str("jinja_template", jinja_template)?;
    let text = tmpl.render(locals)?;
    Ok(text)
}
