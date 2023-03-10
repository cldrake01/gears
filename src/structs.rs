use std::sync::mpsc;
use warp::ws::Message;

pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<Result<Message, warp::Error>>>,
}

pub struct RegisterRequest {
    user_id: usize,
}

pub struct RegisterResponse {
    url: String,
}

pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

pub struct TopicsRequest {
    topics: Vec<String>,
}