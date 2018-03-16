use error::GitDitWuiError as GDWE;
use error::*;

use horrorshow::Template;

use handlers::Stats;

pub fn render_index(stats: Stats) -> Result<String> {
    let mut total_number_issues    = stats.total_number_issues;
    let mut total_number_messages  = stats.total_number_messages;
    let mut authors                = stats.authors.into_iter().collect::<Vec<String>>();
    let mut author_count           = authors.len();

    (html! {
        html {
            : ::renderer::render_header(vec![],
                                        vec![String::from(::renderer::header::CHART_JS_SRC)]);

            div(class = "container") {
                div(class = "content") {
                    : ::renderer::render_body_pre();

                    div(class = "tile is-ancestor") {
                        div(class = "tile is-vertical is-3") {
                            div(class = "tile") {
                                div(class = "tile is-parent is-vertical") {
                                    article(class = "tile is-child notification is-primary") {
                                        p(class = "title") {
                                            : total_number_issues;
                                            : " ";
                                            a(href = "/issues"): "Issues";
                                        }
                                    }

                                    article(class = "tile is-child notification is-warning") {
                                        p(class = "title") {
                                            : total_number_messages;
                                            : " Messages"
                                        }
                                    }

                                    article(class = "tile is-child notification is-danger") {
                                        p(class = "title") {
                                            : author_count;
                                            : " Authors"
                                        }
                                    }
                                }
                            }
                        }

                        div(class = "tile is-parent is-7") {
                            article(class = "tile is-child box") {
                                p(class = "title"): "git-dit-wui";
                            }
                        }

                        div(class = "tile is-parent is-2") {
                            article(class = "tile is-child notification is-success") {
                                p(class = "title"): "Authors";
                                p(class = "content") {
                                    @for author in authors {
                                        p: author;
                                    }
                                }
                            }
                        }
                    }
                    : ::renderer::render_body_post();
                }
            }

            : ::renderer::render_footer();
        }
    }).into_string().map_err(GDWE::from)
}
