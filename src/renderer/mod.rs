pub mod body;
pub mod footer;
pub mod header;
pub mod index;
pub mod issue;
pub mod issue_list;
pub mod message;
pub mod trailer;

pub use self::footer::render_footer;
pub use self::header::render_header;
pub use self::body::render_body_pre;
pub use self::body::render_body_post;

