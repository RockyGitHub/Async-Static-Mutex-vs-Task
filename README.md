
Benchmark tokio tasks that have a dedicated task to serving data from some structure, or accessing that data via a singleton and mutex's

It might not be super representative, please let me know if flaws you see between the two tests!

The pattern is inspired by the documentation page on tokio's main website,
https://tokio.rs/tokio/tutorial/channels

It goes over a Get, Set example that is perfect for countering the need for shared Mutex references
