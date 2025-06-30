use num_bigint::{BigUint, ToBigUint};
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(fib_handler).await
}



/// Computes the nth Fibonacci number using BigInt.
pub fn fib(n: u32) -> BigUint {
    let mut f0 = 0.to_biguint().unwrap();
    let mut f1 = 1.to_biguint().unwrap();

    if n == 0 {
        f0
    } else if n == 1 {
        f1
    } else {
        for _ in 2..=n {
            let f2 = &f0 + &f1;
            f0 = f1;
            f1 = f2;
        }
        f1
    }
}


async fn fib_handler(req: Request) -> Result<Response<Body>, Error> {
    // Extract the path parameter from the URL: /api/fib/{n}
    let path = req.uri().path();
    let n_str = path.trim_start_matches("/api/fib/");
    let n = n_str.parse::<u32>().unwrap_or(10);

    let result = fib(n);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::Text(format!(r#"{{"fib":"{}"}}"#, result)))
        .unwrap())
}