pub mod types;

use handlebars::Handlebars;

use error::GitDitWuiErrorKind as GDWEK;
use error::*;

pub fn get_renderer() -> Result<Handlebars> {
    let mut hb = Handlebars::new();

    let _ = hb.register_template_string("issue_list", include_str!("../templates/issue_list.hbs"))
        .chain_err(|| GDWEK::HandlebarsTemplateRegister("issue_list"))?;

    Ok(hb)
}
