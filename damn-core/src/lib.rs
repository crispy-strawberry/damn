use std::{
    ffi::FromBytesWithNulError,
    io::{self, Error},
    net::{IpAddr, SocketAddr},
};

use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    error::ResolveError,
    proto::rr::rdata::SRV,
    TokioAsyncResolver,
};
use tokio::net::TcpStream;

pub struct SrvLookError;

pub async fn resolve_connection(domain: &str) -> io::Result<TcpStream> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::google(), ResolverOpts::default());

    let query = format!("_xmpp-client._tcp.{domain}.");
    let srv_res = resolver.srv_lookup(query).await;
    if let Ok(srv_records) = srv_res {
        let mut records: Vec<SRV> = srv_records.into_iter().collect();

        // Sorting it according to priority as specified in RFC
        records.sort_unstable_by(|a, b| {
            if a.priority() == b.priority() {
                return a.weight().cmp(&b.weight()).reverse();
            }
            a.priority().cmp(&b.priority())
        });
        // for service in records {
        //     println!("{service}");
        // }
        'services: for service in records {
            let ip_list = resolver.lookup_ip(service.target().clone()).await;

            if ip_list.is_err() {
                continue 'services;
            }

            'ips: for ip in ip_list.unwrap().iter() {
                // let addr = SocketAddr::new(ip, service.port());
                // let connection = TcpStream::connect(addr).await;
                // if connection.is_err() {
                //     continue 'ips;
                // }
                println!("{service} {domain} {ip}");
                // return connection;
            }
        }
    }
    Err(Error::new(io::ErrorKind::NotFound, "Domain not found"))
}

pub async fn connect_ip(ip: IpAddr) -> TcpStream {
    let stream = TcpStream::connect(SocketAddr::new(ip, 5222)).await.unwrap();

    stream
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
