use crate::{config::Config, proxy::manager::ProxyManager,logging,error::ProssimoError};
use anyhow::Result;
use env_filter::Builder;
use hyper::service::service_fn;
use hyper::service::make_service_fn;
use hyper::server::Server;
use std::net::{IpAddr,Ipv4Addr,SocketAddr};
use colored::Colorize;

pub async fn start_proxy_server(
    config: Config,
    manager: ProxyManager,
) -> Result<()> {
    let ipv4addr: Ipv4Addr = config.server.listen_addr.parse().expect("failed to parse ip address");
    let addr= SocketAddr::new(IpAddr::V4(ipv4addr),config.server.listen_port);

    let msg=format!("opening listening port on {}:{}",config.server.listen_addr,config.server.listen_port);
    logging::trace(&msg);

    let make_svc=make_service_fn(move |_| {
        let manager = manager.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |_req| {
                let _proxy = manager.next();
                async move {
                    Ok::<_, hyper::Error>(hyper::Response::<hyper::Body>::new("OK".into()))
                }
            }))
        }
    });


    let msg=format!("binding to ip address {:?} and port {:?}",addr.ip(),addr.port());
    logging::trace(&msg);
    //Server::bind(&addr).serve(make_svc).await?;
    match Server::try_bind(&addr) {
        Ok(b) => {
            let em = format!("Server listening on IP {} port {} for incoming connections...",addr.ip(),addr.port()).as_str().green();
            logging::info("Waiting for incoming connections.");
            println!("{}",em);
            println!("{}","To stop the server use Ctrl-C".blue());
            b.serve(make_svc).await?;
            return Ok(());
        },
        Err(e) => {
            logging::error(format!("{}",e).as_str());
            let em=format!("ERROR: UNABLE TO START SERVER: {}",e.message().to_string()).as_str().red();
            println!("{}",em);
            return Err(ProssimoError::Network(format!("{:#?}",e).to_string()).into());
        },
    }
}
