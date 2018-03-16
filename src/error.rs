error_chain! {
    types {
        GitDitWuiError, GitDitWuiErrorKind, ResultExt, Result;
    }

    links {
        LibGitDit(::libgitdit::error::Error, ::libgitdit::error::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Git(::git2::Error);
        Horrorshow(::horrorshow::Error);
    }

    errors {
        NoInitialTimeForIssue(id: String) {
            description("Cannot find initial time for issue")
                display("Cannot find initial time for issue {}", id)
        }
    }
}

