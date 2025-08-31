use std::time::Duration;

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot::{self, Sender},
};

use super::{Error, Message, Utf8Bytes, ws};

pub enum Command {
    Send(Message, Sender<PyResult<()>>),
    Recv(Option<Duration>, Sender<PyResult<Option<Message>>>),
    Close(Option<u16>, Option<PyBackedStr>, Sender<PyResult<()>>),
}

pub async fn task(websocket: ws::WebSocket, mut command_rx: UnboundedReceiver<Command>) {
    let (mut sender, mut receiver) = websocket.split();

    loop {
        tokio::select! {
            command = command_rx.recv() => {
                match command {
                    Some(Command::Send(msg, tx)) => {
                        let res = sender
                            .send(msg.0)
                            .await
                            .map_err(Error::Library)
                            .map_err(Into::into);

                        let _ = tx.send(res);
                    }

                    Some(Command::Recv(timeout, tx)) => {
                        let fut = async {
                            receiver
                                .try_next()
                                .await
                                .map(|opt| opt.map(Message))
                                .map_err(Error::Library)
                                .map_err(Into::into)
                        };

                        if let Some(timeout) = timeout {
                            match tokio::time::timeout(timeout, fut).await {
                                Ok(res) => {
                                    let _ = tx.send(res);
                                }
                                Err(err) => {
                                    let _ = tx.send(Err(Error::Timeout(err).into()));
                                }
                            }
                        } else {
                            let _ = tx.send(fut.await);
                        }
                    }

                    Some(Command::Close(code, reason, response_tx)) => {
                        let code = code
                            .map(ws::message::CloseCode)
                            .unwrap_or(ws::message::CloseCode::NORMAL);
                        let reason = reason
                            .map(Bytes::from_owner)
                            .and_then(|b| Utf8Bytes::try_from(b).ok())
                            .unwrap_or_else(|| Utf8Bytes::from_static("Goodbye"));
                        let msg =
                            ws::message::Message::Close(Some(ws::message::CloseFrame { code, reason }));

                        let _ = sender.send(msg).await;
                        let _ = sender.flush().await;
                        let _ = sender.close().await;
                        let _ = response_tx.send(Ok(()));
                        break;
                    }

                    None => {
                        // Command channel closed
                        break;
                    }
                }
            }
        }
    }
}

pub async fn recv(
    tx: UnboundedSender<Command>,
    timeout: Option<Duration>,
) -> PyResult<Option<Message>> {
    if tx.is_closed() {
        return Err(Error::WebSocketDisconnected.into());
    }

    let (response_tx, response_rx) = oneshot::channel();
    tx.send(Command::Recv(timeout, response_tx))
        .map_err(|_| Error::WebSocketDisconnected)?;

    response_rx
        .await
        .map_err(|_| Error::WebSocketDisconnected)?
}

pub async fn send(tx: UnboundedSender<Command>, message: Message) -> PyResult<()> {
    if tx.is_closed() {
        return Err(Error::WebSocketDisconnected.into());
    }

    let (response_tx, response_rx) = oneshot::channel();
    tx.send(Command::Send(message, response_tx))
        .map_err(|_| Error::WebSocketDisconnected)?;

    response_rx
        .await
        .map_err(|_| Error::WebSocketDisconnected)?
}

pub async fn close(
    tx: UnboundedSender<Command>,
    code: Option<u16>,
    reason: Option<PyBackedStr>,
) -> PyResult<()> {
    if tx.is_closed() {
        return Err(Error::WebSocketDisconnected.into());
    }

    let (response_tx, response_rx) = oneshot::channel();
    tx.send(Command::Close(code, reason, response_tx))
        .map_err(|_| Error::WebSocketDisconnected)?;

    response_rx
        .await
        .map_err(|_| Error::WebSocketDisconnected)?
}
