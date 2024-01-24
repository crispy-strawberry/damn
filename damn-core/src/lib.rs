use std::{
    io::{self, Error},
    net::SocketAddr,
    time::Duration,
};

use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub struct SrvLookError;

pub async fn resolve_connection(domain: &str) -> io::Result<TcpStream> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default());

    let query = format!("_xmpp-client._tcp.{domain}.");
    let srv_res = resolver.srv_lookup(query).await;
    if let Ok(srv_records) = srv_res {
        let mut records = srv_records.into_iter().collect::<Vec<_>>();

        // Sorting it according to priority
        // I am not doing the random probability bullshit.
        records.sort_unstable_by(|a, b| {
            if a.priority() == b.priority() {
                return a.weight().cmp(&b.weight()).reverse();
            }
            a.priority().cmp(&b.priority())
        });

        for service in records {
            let ip_list = resolver.lookup_ip(service.target().clone()).await;

            if ip_list.is_err() {
                continue;
            }

            for ip in ip_list.unwrap().iter() {
                let addr = SocketAddr::new(ip, service.port());
                let conn = TcpStream::connect(addr).await;
                if let Ok(connection) = conn {
                    return Ok(connection);
                }
                println!("{service} {domain} {ip}");
            }
        }

        return Err(Error::new(io::ErrorKind::NotFound, "Could not connect"));
    } else {
        let ip_list = resolver.lookup_ip(domain).await?;

        for ip in ip_list.iter() {
            println!("{ip}");
            let addr = SocketAddr::new(ip, 5222);
            let conn = timeout(Duration::from_secs(4), TcpStream::connect(addr)).await;

            if let Ok(Ok(connection)) = conn {
                return Ok(connection);
            }
        }
    }
    Err(Error::new(
        io::ErrorKind::NotFound,
        "Failed to connect to server! Please check if the server exists and is accepting connections",
    ))
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
