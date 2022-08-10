use std::collections::HashMap;
use std::net::TcpStream;
use super::tcp_message::Message;
use super::super::Quey;
use std::sync::Arc;
use std::sync::Mutex;
use async_trait::async_trait;
pub type Callback = fn(TcpStream,Vec<u8>);

pub struct Route<D=()> {
    pub method: u32,
    pub handler: Box<dyn Middleware<D> + Send + Sync + 'static>,
    
}

#[async_trait]
impl<D: Send + Sync + 'static> Middleware<D> for Router<D> {
    async fn invoke(&self, req: &mut Request<D>, mut res: Response<D>)
                          -> MiddlewareResult<D> {
        debug!("Router::invoke for '{:?}'", req.origin.uri());

        // Strip off the querystring when matching a route
        let route_result = self.match_route(&req.origin.method(), req.path_without_query());

        debug!("route_result.route.path: {:?}", route_result.as_ref().map(|(_, r)| r.matcher.path()));

        match route_result {
            Some((route_result, route)) => {
                res.set(StatusCode::OK);
                req.route_result = Some(route_result);
                route.handler.invoke(req, res).await
            },
            None => res.next_middleware()
        }
    }
}

pub struct RouterHandler {
    route_function: HashMap<u32, Callback>,    
}

impl RouterHandler {
    pub fn new()->RouterHandler{        
        RouterHandler { route_function: HashMap::new() }
    }
    pub fn add_route(&mut self, id: u32, func: Callback) {
        self.route_function.insert(id, func);
    }
    pub fn call_route(&mut self, mut stream:TcpStream, id:u32, body: Vec<u8>){ 
                
        let func = self.route_function[&id];        
        //println!("CALLED MSG ID: {}",id)
        func(stream,body);
    }
}