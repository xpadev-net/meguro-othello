use zoon::{eprintln, *};
use shared::{DownMsg, UpMsg,Message};
use uuid::Uuid;
use crate::othello::{Board, create_board};

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|DownMsg::MessageReceived(message), _| {
        if message.key == "create_room" && is_searching_room().get() {
            send_join_room_req(message.data[0].to_string());
        }else if message.key == "join_room" && self_id().get_cloned() == message.data[0] {
            if is_searching_room().get() {
                is_searching_room().set(false);
                stopwatch().take();
                let _target_id = &message.data[1];
                send_join_room_approve(_target_id.clone());
                target_id().set(Option::from(_target_id).cloned());
            }else{
                send_join_room_reject(message.data[1].to_string());
            }
        }else if message.key == "approve" && self_id().get_cloned() == message.data[0]{
            target_id().set(Option::from(message.data[1].to_string()));
        }else if message.key == "send_board" && self_id().get_cloned() == message.data[0] && target_id().get_cloned().unwrap() == message.data[1]{
            //todo: load_board(message.data[2]) でmain.rsのデータを更新する
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


pub fn search_room(){
    stopwatch().set(Some(Timer::new(1_000, send_create_room_req)));
    is_searching_room().set(true)
}

pub fn send_board(board: Board) {
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                key: "send_board".parse().unwrap(),
                data: vec![target_id().get_cloned().unwrap(),self_id().get_cloned(),board.dump()],
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn send_create_room_req(){
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
