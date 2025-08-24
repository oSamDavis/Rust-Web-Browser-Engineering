// Bringing relevant namespaces into scope.
use std::io;
use std::net::TcpStream;

// Struct: custom data type groups related values. Like
// a C++ struct with only data members.
// 
// #[derive ...] auto implements traits. Gives us extra
// functionality without having to write the code ourselves.
// * Debug: allows us use println!("{:?}", value) for debugging.
// * Clone: allows us make deep copies via .clone(). Like a
// copy ctor in C++.
//
// A url has: a scheme, a host, a path and a port.
#[derive(Debug, Clone)]
struct Url {
    // String is a growable string on the heap.
    // It's owned because the URL struct is responsible for
    // freeing the data.
    scheme: String,
    host: String,
    path: String,
    port: u16, // 16-bit unsigned int.
}

// The impl block is where methods and associated functions are
// defined. Behavior of the URL struct lives here.
// * new(): creates new URL from an input string.
// * request(): Attempts to establish cxn to a url over TCP.
impl Url {
    // An associated function. Will be invoked as `Url::new`,
    // doesn't take self as param. Often used for ctors.
    //
    // * &str: string slice, a reference(pointer + length) to
    // a string data that is owned by someone else. Can't
    // modify it, can't free it. Can only read.
    // * String: owned string buffer on heap. Growable and we
    // own it.
    //
    // The input string is a &str because it's cheap to pass
    // around and we don't need to take ownership of it. However,
    // we store pieces of the input as Strings because the URL
    // class needs to own its data to ensure they stay valid as
    // long as the object exists.
    //
    // Result<T, E>, we'll either return a URL(Self) or an Error String.
    // Ok(value) ==> success: URL instance
    // Err(err) ==> failure: Error String.
    fn new(url_input: &str) -> Result<Self, String> {
        // 1. url_input should be in the form SCHEME://HOSTNAME/PATH.
        // we'll deconstruct the input string and grab relevant pieces
        // to create a URL.

        // "?": unwrap if there's an Ok(value), else immediately
        // return the Err(error).
        // ok_or_else: Converts an Option<T> to a Result(T, E).
        // Takes a closure(|| (){} ) "||" => anonymous func.
        // closure is only run if Option<T> is None. In this case if split_once("://") wasn't successful
        let (scheme, rest_of_url) = url_input
            .split_once("://")
            .ok_or_else(|| "URL missing scheme delimiter :// ".to_string())?;

        // "format!" macro that creates a String by formating a string literal with placeholders.
        if scheme != "http" {
            return Err(format!(
                "only http is supported for now, but got : {scheme}"
            ));
        }

        // 2. HOSTNAME /PATH
        // 'match' super powered switch, ensures we handle every possible case.
        let (host, path) = match rest_of_url.split_once('/') {
            // if `split_once`` returns something, destructure the tuple.
            // we include the / for the path because it's a part of the path.
            Some((host, path)) => (host, format!("/{}", path)),
            // if `split_once` returns None, we use the rest of the url as the host and path as "/"
            None => (rest_of_url, "/".to_string()),
        };

        // Return a String error if the host is empty.
        if host.is_empty() {
            return Err(String::from("host is empty"));
        }

        // Construct an return a URL(self) on success.
        Ok(Self {
            scheme: scheme.to_string(),
            host: host.to_string(),
            path,
            port: 80,
        })
    }

    // A method because first param is &self. &self is an immutable borrow
    // of the Url Struct, giving only read-only access.
    // io::Result<T> == io::Result<T, io::Error>, error type is a standard io::Error.
    fn request(&self) -> io::Result<TcpStream> {
        // TCP: AF_INET, SOCK_STREAM , TCP
        // Takes a tuple of (host, port)
        // passes the host as a borrowed string slice
        TcpStream::connect((self.host.as_str(), self.port))
    }
}
