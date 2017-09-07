use reqwest;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        Reqwest(reqwest::Error);
        IO(::std::io::Error);
    }
}
