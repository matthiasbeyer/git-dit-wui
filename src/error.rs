error_chain! {
    types {
        GitDitWuiError, GitDitWuiErrorKind, ResultExt, Result;
    }

    links {
        LibGitDit(::libgitdit::error::Error, ::libgitdit::error::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        HandlebarsRenderError(::handlebars::RenderError);
        HandlebarsFileError(::handlebars::TemplateError);
    }

    errors {
        HandlebarsTemplateRegister(name: &'static str) {
            description("Failed to register template")
                display("Failed to register template '{}'", name)
        }
    }
}

