use std::ops::ControlFlow;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Error,
};

use futures_util::{sink::SinkExt, stream::StreamExt};
// use anyhow::Error;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::adapter::{LocalShell, ResizablePty};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| {
        let pty = LocalShell::new("sh").unwrap();
        handle_socket(pty, socket)
    })
}

async fn handle_socket<T>(pty: T, socket: WebSocket) 
where
    T: ResizablePty,
    T::OwnedR: Unpin + Send + 'static,
    T::OwnedW: Unpin + Send + 'static,
{
    let (mut reader, mut writer) = pty.into_split();

    let (mut sender, mut receiver) = socket.split();

    let mut send_task: tokio::task::JoinHandle<Result<(), Error>> = tokio::spawn(async move {
        let mut buf = [0; 1024];
        while let Ok(size) = reader.read(&mut buf).await {
            if size > 0 {
                // TODO: proper coding converion
                // sender.send(Message::Binary(buf.to_vec())).await?
                sender
                    .send(Message::Text(
                        String::from_utf8(buf[..size].to_vec()).unwrap(),
                    ))
                    .await?
            } else {
                break;
            }
        }
        Ok(())
    });

    let mut recv_task: tokio::task::JoinHandle<Result<(), std::io::Error>> =
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                match process_message(msg, &mut writer).await {
                    ControlFlow::Continue(ret) => ret?,
                    ControlFlow::Break(()) => break,
                }
            }
            Ok(())
        });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(Ok(())) => (),
                _ => println!("Error sending messages {rv_a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(Ok(())) => (),
                _ => println!("Error receiving messages {rv_b:?}")
            }
            send_task.abort();
        }
    }

    println!("Websocket context destroyed");
}

async fn process_message(
    msg: Message,
    writer: &mut (impl AsyncWrite + Unpin),
) -> ControlFlow<(), Result<(), std::io::Error>> {
    match msg {
        Message::Text(t) => {
            // println!(">>> sent str: {t:?}");
            match writer.write(t.as_bytes()).await {
                Ok(n) => {
                    if n == 0 {
                        return ControlFlow::Break(());
                    };
                    assert!(n == t.as_bytes().len()); // TODO: Handle n <= len
                }
                Err(e) => return ControlFlow::Continue(Err(e)),
            }
        }
        Message::Binary(d) => {
            // println!(">>> sent {} bytes: {:?}", d.len(), d);
            match writer.write(&d).await {
                Ok(n) => {
                    assert!(n == d.len()); // TODO: Handle n <= len
                }
                Err(e) => return ControlFlow::Continue(Err(e)),
            }
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    "client close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                println!("client force closed");
            }
            return ControlFlow::Break(());
        }
        _ => (),
        // Message::Pong(v) => {
        //     // println!(">>> sent pong with {v:?}");
        // }
        // Message::Ping(v) => {
        //     println!(">>> sent ping with {v:?}");
        // }
    }
    ControlFlow::Continue(Ok(()))
}
