#[derive(Fail, Debug)]
#[fail(display = "An error occurred: {}", message)]
pub struct GistError {
    pub message: String,
}
