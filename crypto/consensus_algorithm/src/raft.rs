use rand::Rng;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub enum NodeState {
    Leader,
    Follower,
    Candidate,
}

pub enum RaftMessageBody {
    RequestVoteMsg(u64),
    // candidate_id vote_status
    RequestVoteResponseMsg(u64, bool),
    AppendEntriesRequest(u64, u64),
    AppendEntriesResponse(bool),
}

pub struct Message {
    sender_node_id: u64,
    body: Box<RaftMessageBody>,
    timestamp: u64,
}

pub struct RaftNode<'a> {
    id: u64,
    state: NodeState,
    current_term: u64,
    vote_for: Option<u64>,
    log: Vec<u64>,
    commit_index: u64,
    sender: Arc<Sender<Arc<Message>>>,
    connect_peer_channel: &'a mut HashMap<u64, Arc<Receiver<Arc<Message>>>>,
    leader_latest_msg: Option<Arc<Message>>,
    system: &'a mut System<'a>,
}

pub struct VoteInfo {
    // 获得票数
    count: u64,
    // 投票者的id，投票者投票的阶段
    votes: HashMap<u64, u64>,
}

pub struct System<'a> {
    peers: HashMap<u64, Arc<Mutex<RaftNode<'a>>>>,
    leader: Option<u64>,
    term: u64,
    // <被投的id， 被投的数量>
    votes: Mutex<HashMap<u64, Box<VoteInfo>>>,
}

impl<'a> System<'a> {
    pub fn new() -> Self {
        System {
            peers: HashMap::new(),
            leader: None,
            term: 0,
            votes: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_peer(&mut self, raft_node: Arc<Mutex<RaftNode<'a>>>) {
        self.peers
            .insert(raft_node.lock().unwrap().id, raft_node.clone());
    }
}

pub fn get_current_timestamp() -> u64 {
    let current_time = SystemTime::now();
    let since_epoch = current_time
        .duration_since(UNIX_EPOCH)
        .expect("time went backward.");
    since_epoch.as_secs()
}

impl<'a> RaftNode<'a> {
    pub fn new(
        id: u64,
        mut connect_peer_channel: &'a mut HashMap<u64, Arc<Receiver<Arc<Message>>>>,
        system: &'a mut System<'a>,
    ) -> RaftNode<'a> {
        let (tx, rx) = channel();
        connect_peer_channel.insert(id, Arc::new(rx));
        RaftNode {
            id,
            state: NodeState::Follower,
            current_term: 0,
            vote_for: None,
            log: Vec::new(),
            commit_index: 0,
            sender: Arc::new(tx),
            connect_peer_channel,
            leader_latest_msg: None,
            system,
        }
    }

    pub fn get_peers(&self) -> HashMap<u64, Arc<Mutex<RaftNode<'a>>>> {
        return self.system.peers.clone();
    }

    pub fn run(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            match self.state {
                NodeState::Follower => {
                    // 一段时间没有收到leader的消息，则自动转化为Candidate
                    let time_update = get_current_timestamp()
                        - self.leader_latest_msg.as_ref().unwrap().timestamp;
                    if time_update > 5000000 {
                        self.state = NodeState::Candidate;
                        println!("Node {} becomes a candidate", self.id);
                    }
                }
                NodeState::Candidate => {
                    for (_, peer) in self.get_peers().iter() {
                        let mut peer = peer.lock().unwrap();
                        peer.send_msg(RaftMessageBody::RequestVoteMsg(self.current_term));
                    }
                    thread::sleep(Duration::from_secs(1));
                }
                NodeState::Leader => {
                    for (_, peer) in self.get_peers().iter() {
                        let mut peer = peer.lock().unwrap();
                        let log = rng.gen_range(0..1024);
                        peer.send_msg(RaftMessageBody::AppendEntriesRequest(
                            self.current_term,
                            log,
                        ))
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }

    pub fn send_msg(&mut self, msg_body: RaftMessageBody) {
        let msg = Message {
            sender_node_id: self.id,
            timestamp: get_current_timestamp(),
            body: Box::new(msg_body),
        };
        self.sender
            .send(Arc::new(msg))
            .expect("send msg failed; msg: {:?}");
    }

    fn handle_leader_msg(&mut self, msg: Arc<Message>) -> Option<RaftMessageBody> {
        self.leader_latest_msg = Some(msg.clone());
        match self.leader_latest_msg.as_ref().unwrap().body.deref() {
            RaftMessageBody::RequestVoteMsg(term) => {
                println!("RequestVoteMsg for term: {term}");
                Some(RaftMessageBody::RequestVoteResponseMsg(
                    msg.sender_node_id,
                    true,
                ))
            }
            RaftMessageBody::RequestVoteResponseMsg(candidate_id, vote_success) => {
                let count = if *vote_success { 1 } else { 0 };
                if let Some(&mut ref mut vote_info) =
                    self.system.votes.lock().unwrap().get_mut(&candidate_id)
                {
                    if let Some(&term_id) = vote_info.votes.get(&msg.sender_node_id) {
                        if term_id > self.system.term {
                            println!(
                                "have already count {:?} vote for {:?}",
                                msg.sender_node_id, candidate_id
                            );
                        } else {
                            println!(
                                "term id: ${term_id} less than current system term id: ${:?}",
                                self.system.term
                            )
                        }
                    } else {
                        vote_info.count += count;
                        vote_info
                            .votes
                            .insert(msg.sender_node_id, self.system.term + 1);
                    }
                } else {
                    let mut vote_info = HashMap::new();
                    vote_info.insert(msg.sender_node_id, self.current_term);
                    self.system.votes.lock().unwrap().insert(
                        *candidate_id,
                        Box::new(VoteInfo {
                            count: 1,
                            votes: vote_info,
                        }),
                    );
                }
                if self
                    .system
                    .votes
                    .lock()
                    .unwrap()
                    .get(&candidate_id)
                    .unwrap()
                    .count
                    >= self.system.peers.len() as u64 * 2 / 3
                {
                    self.system.borrow_mut().leader = Some(*candidate_id);
                }
                None
            }
            RaftMessageBody::AppendEntriesRequest(term, log) => {
                self.log.push(*log);
                Some(RaftMessageBody::AppendEntriesResponse(true))
            }
            RaftMessageBody::AppendEntriesResponse(_success) => {
                panic!("status error: only leader append entries");
            }
        }
    }

    fn handle_msg(&mut self, msg: Arc<Message>, channel_node_id: u64) -> Option<RaftMessageBody> {
        match self.system.leader {
            Some(leader_id) => {
                if leader_id == channel_node_id {
                    self.handle_leader_msg(msg)
                } else {
                    None
                }
            }
            None => match *msg.body.deref() {
                RaftMessageBody::RequestVoteMsg(term) => {
                    println!("RequestVoteMsg for term: {term}");
                    Some(RaftMessageBody::RequestVoteResponseMsg(
                        msg.sender_node_id,
                        true,
                    ))
                }
                RaftMessageBody::RequestVoteResponseMsg(candidate_id, vote_success) => {
                    let count = if vote_success { 1 } else { 0 };
                    if let Some(&mut ref mut vote_info) =
                        self.system.votes.lock().unwrap().get_mut(&candidate_id)
                    {
                        if let Some(&term_id) = vote_info.votes.get(&msg.sender_node_id) {
                            if term_id > self.system.term {
                                println!(
                                    "have already count {:?} vote for {:?}",
                                    msg.sender_node_id, candidate_id
                                );
                            } else {
                                println!(
                                    "term id: ${term_id} less than current system term id: ${:?}",
                                    self.system.term
                                )
                            }
                        } else {
                            vote_info.count += count;
                            vote_info
                                .votes
                                .insert(msg.sender_node_id, self.system.term + 1);
                        }
                    } else {
                        let mut vote_info = HashMap::new();
                        vote_info.insert(msg.sender_node_id, self.current_term);
                        self.system.votes.lock().unwrap().insert(
                            candidate_id,
                            Box::new(VoteInfo {
                                count: 1,
                                votes: vote_info,
                            }),
                        );
                    }
                    if self
                        .system
                        .votes
                        .lock()
                        .unwrap()
                        .get(&candidate_id)
                        .unwrap()
                        .count
                        >= self.system.peers.len() as u64 * 2 / 3
                    {
                        self.system.borrow_mut().leader = Some(candidate_id);
                    }
                    None
                }
                RaftMessageBody::AppendEntriesRequest(term, log) => {
                    panic!("append entries not send from leader node, node id {:?} term: {:?} log: {:?}", msg.sender_node_id, term, log);
                }
                RaftMessageBody::AppendEntriesResponse(success) => {
                    println!(
                        "node {:?} append entries status {:?}",
                        msg.sender_node_id, success
                    );
                    None
                }
            },
        }
    }

    pub fn receive_msg(
        &mut self,
        connect_peer_channel: &'a HashMap<u64, Arc<Receiver<Arc<Message>>>>,
    ) {
        for (&id, &ref channel) in connect_peer_channel.iter() {
            if id == self.id {
                println!("self channel ignore.")
            } else {
                for msg in channel.iter() {
                    if let Some(res) = self.handle_msg(msg, id) {
                        self.send_msg(res);
                    }
                }
            }
        }
    }
}
