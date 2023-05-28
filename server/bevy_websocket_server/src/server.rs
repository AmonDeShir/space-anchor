//Based on https://github.com/AmonDeShir/bevy-ws-server/blob/main/src/lib.rs

use std::sync::atomic::{Ordering, AtomicU64};

use async_net::{TcpStream, AsyncToSocketAddrs, TcpListener};
use async_tungstenite::{WebSocketStream, tungstenite::Message};
use bevy::{prelude::{Resource, debug, error}, tasks::{Task, IoTaskPool}, utils::HashMap};
use crossbeam_channel::{Receiver, Sender};
use futures::{FutureExt, StreamExt, SinkExt, select};

pub struct Connection {
    pub _io: Task<()>,
    pub sender: async_channel::Sender<Message>,
    pub receiver: async_channel::Receiver<Message>,
}

#[derive(Resource)]
pub struct Server {
    listener: Sender<WebSocketStream<TcpStream>>,
    accept_queue: Receiver<WebSocketStream<TcpStream>>,
    pub connections: HashMap<u64, Connection>,
}


impl Server {
    /// Creates a new server instance.
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Self { listener: tx, accept_queue: rx, connections: HashMap::new() }
    }

    /// Starts listening for new connections. When a connection occurs, this method accepts it and sends it to the accept_queue.
    pub fn listen(&self, bind_to: impl AsyncToSocketAddrs) {
        let listener = futures::executor::block_on(TcpListener::bind(bind_to))
            .expect("cannot bind to the address");

        let task_pool = IoTaskPool::get();
        let tx = self.listener.clone();
        let task = task_pool.spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        debug!("new connection from {}", addr);

                        let tx = tx.clone();
                        let accept = async move {
                            match async_tungstenite::accept_async(stream).await {
                                Ok(websocket) => {
                                    // Ignoring error is ok because then WsResource is not present,
                                    // thus there is no need for accepting a new websocket.
                                    let _ = tx.send(websocket);
                                }
                                Err(e) => {
                                    error!("error handshaking a new websocket: {}", e);
                                }
                            }
                        };
                        task_pool.spawn(accept).detach();
                    }
                    Err(e) => {
                        error!("error accepting a new connection: {}", e);
                    }
                }
            }
        });

        task.detach();
    }

    /// Generates an unique connection ID.
    fn next_id() -> u64 {
        static COUNTER:AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    /// Handles a new client by generating an ID for it and moving it from the accept_queue to the connections hash map.
    /// It returns a vector containing the IDs of all newly connected clients.
    pub fn handle_accept_queue(&mut self) -> Vec<u64> {
        let mut new_clients = vec![];

        for mut websocket in self.accept_queue.try_iter() {
            let (message_tx, io_message_rx) = async_channel::unbounded::<Message>();
            let (io_message_tx, message_rx) = async_channel::unbounded::<Message>();
    
            let io = IoTaskPool::get().spawn(async move {
                loop {
                    let mut from_channel = io_message_rx.recv().fuse();
                    let mut from_ws = websocket.next().fuse();
                    select! {
                        message = from_channel => if let Ok(message) = message {
                            let _ =  websocket.send(message).await;
                        } else {
                            break;
                        },
                        message = from_ws => if let Some(Ok(message)) = message {
                            let _ = io_message_tx.send(message).await;
                        } else {
                            break;
                        },
                        complete => break,
                    }
                }
            });
            
            let id: u64 = Self::next_id();

            self.connections.insert(id.clone(), Connection{
                _io: io,
                sender: message_tx,
                receiver: message_rx,
            });

            new_clients.push(id);
        }

        return new_clients;
    }

    /// Sends a message to a client. Returns false if the client with the provided id does not exist in the connections hash map or if there is an error while the message is being sent.
    pub fn send(&self, id: &u64, message: String) -> bool {
        if let Some(connection) = self.connections.get(id) {
            return connection.sender.try_send(Message::Text(message)).is_ok()
        }

        return false;
    }

    /// Sends a message to all clients.
    pub fn broadcast(&self, message: String) {
        for (_, connection) in self.connections.iter() { 
            let _ = connection.sender.try_send(Message::Text(message.clone()));
        }
    }

    /// Send a message to all clients except one.
    pub fn broadcast_except(&self, id: &u64, message: String) {
        for (connection_id, connection) in self.connections.iter() { 
            if *id == *connection_id {
                continue;
            }

            let _ = connection.sender.try_send(Message::Text(message.clone()));
        }
    }

}

