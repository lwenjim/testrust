#[macro_use]
extern crate error_chain;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        // You can link to other crates' error types here
    }

    foreign_links {
        // You can link to other crates' errors here
        Io(std::io::Error);
    }

    errors {
        CustomError(msg: String) {
            description("A custom error occurred")
            display("Custom error: {}", msg)
        }
    }
}

fn might_fail() -> Result<()> {
    Err(ErrorKind::CustomError("something went wrong".into()).into())
}

fn main() -> Result<()> {
    might_fail().chain_err(|| "failed in might_fail")?;
    Ok(())
}
