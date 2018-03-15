use error::*;

#[derive(Serialize)]
pub struct Trailer {
    key: String,
    val: String,
}

impl Trailer {
    pub fn from_trailer(t: &::libgitdit::trailer::Trailer) -> Result<Trailer> {
        Ok(Trailer {
            key: format!("{}", t.key),
            val: format!("{}", t.value),
        })
    }
}
