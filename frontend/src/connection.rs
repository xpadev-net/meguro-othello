use std::fmt::format;
use zoon::{eprintln, *};
use shared::{DownMsg, UpMsg,Message};
use uuid::Uuid;
use zoon::console::log;
use crate::othello::{Board, create_board};

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|DownMsg::MessageReceived(message), _| {
        log(&*format!("message_received {:?}, {:?}, {}", message.key, message.data, self_id().get_cloned()));
        if message.key == "create_room" && is_searching_room().get() && message.data[0] != self_id().get_cloned() {
            send_join_room_req(message.data[0].to_string());
            return
        }
        if message.key == "join_room" {
            log(&*format!("join_room {},{},{}", message.key == "join_room" ,self_id().get_cloned() == message.data[0] , message.data[1] != self_id().get_cloned()));
        }
        if message.key == "join_room" && self_id().get_cloned() == message.data[0] && message.data[1] != self_id().get_cloned()  {
            if is_searching_room().get() {
                is_searching_room().set(false);
                stopwatch().take();
                let _target_id = &message.data[1];
                target_id().set(Option::from(_target_id).cloned());
                send_join_room_approve(_target_id.clone());
            }else{
                send_join_room_reject(message.data[1].to_string());
            }
            return;
        }
        if message.key == "approve" {
            log(&*format!("approve {},{},{}", message.key == "approve" ,self_id().get_cloned() == message.data[0] , message.data[1] != self_id().get_cloned()));
        }
        if message.key == "approve" && self_id().get_cloned() == message.data[0] && message.data[1] != self_id().get_cloned(){
            target_id().set(Option::from(message.data[1].to_string()));
            stopwatch().take();
            send_syn_request();
            return;
        }
        if message.key == "syn" && self_id().get_cloned() == message.data[0] && target_id().get_cloned().unwrap() == message.data[1]{
            send_syn_ack_request();
            return;
        }
        if message.key == "syn_ack" && self_id().get_cloned() == message.data[0] && target_id().get_cloned().unwrap() == message.data[1]{
            send_ack_request();
            return;
        }
        if message.key == "ack" && self_id().get_cloned() == message.data[0] && target_id().get_cloned().unwrap() == message.data[1]{
            send_game_start_request();
            is_master().set(true);
            return;
        }
        if message.key == "game_start" && ((self_id().get_cloned() == message.data[0] && target_id().get_cloned().unwrap() == message.data[1]) || (self_id().get_cloned() == message.data[1] && target_id().get_cloned().unwrap() == message.data[0])){
            log("game_start");
            if is_master().get() {
                stopwatch().set(Some(Timer::new(10_000, send_ping_req)));
            }
            //todo: ゲームを開始する
            return;
        }
        if message.key == "send_board" && self_id().get_cloned() == message.data[0] && target_id().get_cloned().unwrap() == message.data[1]{
            //todo: load_board(message.data[2]) でmain.rsのデータを更新する
            return;
        }
        if message.key == "ping" && self_id().get_cloned() == message.data[0] && target_id().get_cloned().unwrap() == message.data[1]{
            send_pong_req();
            return;
        }
    })
}

#[static_ref]
fn self_id() -> &'static Mutable<String> {
    Mutable::new(Uuid::new_v4().to_string())
}

#[static_ref]
fn target_id() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn stopwatch() -> &'static Mutable<Option<Timer>> {
    Mutable::new(None)
}

#[static_ref]
fn is_searching_room() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn is_master() -> &'static Mutable<bool> {
    Mutable::new(false)
}

pub fn search_room(){
    log("search_room");
    stopwatch().set(Some(Timer::new(1_000, send_create_room_req)));
    is_searching_room().set(true);
    is_master().set(false);
}

pub fn send_board(board: Board) {
    log("send_board");
    let board_json = board.dump();
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "send_board".parse().unwrap(),
                data: vec![target_id().get_cloned().unwrap(),self_id().get_cloned(),board_json],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_create_room_req(){
    log("send_create_room_req");
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "create_room".parse().unwrap(),
                data: vec![self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_join_room_req(target_id: String) {
    log(&*format!("send_join_room_req {}", target_id));
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "join_room".parse().unwrap(),
                data: vec![target_id,self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_join_room_approve(target_id: String){
    log(&*format!("send_join_room_approve {}", target_id));
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "approve".parse().unwrap(),
                data: vec![target_id,self_id().get_cloned()],
            }))
            .await;
        //todo: boardを作成し、send_board(board)を呼び出す
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_join_room_reject(target_id: String) {
    log(&*format!("send_join_room_reject {}", target_id));
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "reject".parse().unwrap(),
                data: vec![target_id,self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_syn_request(){
    log("send_syn_request");
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "syn".parse().unwrap(),
                data: vec![target_id().get_cloned().unwrap(),self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_syn_ack_request(){
    log("send_syn_ack_request");
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "syn_ack".parse().unwrap(),
                data: vec![target_id().get_cloned().unwrap(),self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_ack_request(){
    log("send_ack_request");
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "ack".parse().unwrap(),
                data: vec![target_id().get_cloned().unwrap(),self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_game_start_request(){
    log("send_game_start_request");
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "game_start".parse().unwrap(),
                data: vec![target_id().get_cloned().unwrap(),self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_ping_req(){
    log("send_ping_req");
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "ping".parse().unwrap(),
                data: vec![target_id().get_cloned().unwrap(),self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_pong_req(){
    log("send_pong_req");
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "pong".parse().unwrap(),
                data: vec![target_id().get_cloned().unwrap(),self_id().get_cloned()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}
