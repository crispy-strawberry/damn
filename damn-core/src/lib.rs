use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    error::ResolveError,
    TokioAsyncResolver,
};

pub struct SrvLookError {}

pub async fn resolve_connection() -> Result<(), ResolveError> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::google(), ResolverOpts::default());

    let srv = resolver.srv_lookup("google.com").await?;

    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
