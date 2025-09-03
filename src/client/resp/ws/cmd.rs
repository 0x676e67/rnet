//! WebSocket Command Utilities
//!
//! This module defines the `Command` enum for representing WebSocket operations
//! (send, receive, close) and provides async helpers for sending commands to the
//! WebSocket background task. It enables safe, concurrent, and ergonomic control
//! of WebSocket communication from Python bindings.

use std::time::Duration;

use bytes::Bytes;
use futures_util::TryStreamExt;
use pyo3::{prelude::*, pybacked::PyBackedStr};
use tokio::sync::{
    mpsc::{self, Receiver},
    oneshot::{self, Sender},
};

use super::{
    Error, Message, Utf8Bytes,
    ws::{self, WebSocket},
};

/// Commands for WebSocket operations.
pub enum Command {
    /// Send a WebSocket message.
    ///
    /// Contains the message to send and a oneshot sender for the result.
    Send(Message, Sender<PyResult<()>>),

    /// Receive a WebSocket message.
    ///
    /// Contains an optional timeout and a oneshot sender for the result.
    Recv(Option<Duration>, Sender<PyResult<Option<Message>>>),

    /// Close the WebSocket connection.
    ///
    /// Contains an optional close code, optional reason, and a oneshot sender for the result.
    Close(Option<u16>, Option<PyBackedStr>, Sender<PyResult<()>>),
}

/// The main background task that processes incoming [`Command`]s and interacts with the WebSocket.
///
/// Handles sending, receiving, and closing the WebSocket connection based on received commands.
pub async fn task(mut ws: WebSocket, mut cmd: Receiver<Command>) {
    while let Some(command) = cmd.recv().await {
        match command {
            // Handle sending a message
            Command::Send(msg, tx) => {
                let res = ws
                    .send(msg.0)
                    .await
                    .map_err(Error::Library)
                    .map_err(Into::into);

                let _ = tx.send(res);
            }
            // Handle receiving a message
            Command::Recv(timeout, tx) => {
                let fut = async {
                    ws.try_next()
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
            // Handle closing the connection
            Command::Close(code, reason, tx) => {
                let code = code
                    .map(ws::message::CloseCode)
                    .unwrap_or(ws::message::CloseCode::NORMAL);
                let reason = reason
                    .map(Bytes::from_owner)
                    .and_then(|b| Utf8Bytes::try_from(b).ok())
                    .unwrap_or_else(|| Utf8Bytes::from_static("Goodbye"));

                let res = ws
                    .close(code, reason)
                    .await
                    .map_err(Error::Library)
                    .map_err(Into::into);
                let _ = tx.send(res);
                break;
            }
        }
    }
}

/// Sends a [`Command::Recv`] to the background task and awaits a message from the WebSocket.
///
/// Returns the received message or an error if the connection is closed or timeout.
pub async fn recv(
    cmd: mpsc::Sender<Command>,
    timeout: Option<Duration>,
) -> PyResult<Option<Message>> {
    if cmd.is_closed() {
        return Err(Error::WebSocketDisconnected.into());
    }

    let (tx, rx) = oneshot::channel();

    let fut = async {
        cmd.send(Command::Recv(timeout, tx))
            .await
            .map_err(|_| Error::WebSocketDisconnected)?;
        rx.await.map_err(|_| Error::WebSocketDisconnected)?
    };

    if let Some(timeout) = timeout {
        tokio::time::timeout(timeout, fut)
            .await
            .map_err(Error::Timeout)?
    } else {
        fut.await
    }
}

/// Sends a [`Command::Send`] to the background task to transmit a message over the WebSocket.
///
/// Returns Ok if the message was sent successfully, or an error otherwise.
pub async fn send(cmd: mpsc::Sender<Command>, message: Message) -> PyResult<()> {
    if cmd.is_closed() {
        return Err(Error::WebSocketDisconnected.into());
    }

    let (tx, rx) = oneshot::channel();
    cmd.send(Command::Send(message, tx))
        .await
        .map_err(|_| Error::WebSocketDisconnected)?;
    rx.await.map_err(|_| Error::WebSocketDisconnected)?
}

/// Sends a [`Command::Close`] to the background task to gracefully close the WebSocket connection.
///
/// Returns Ok if the connection was closed successfully, or an error otherwise.
pub async fn close(
    cmd: mpsc::Sender<Command>,
    code: Option<u16>,
    reason: Option<PyBackedStr>,
) -> PyResult<()> {
    if cmd.is_closed() {
        return Err(Error::WebSocketDisconnected.into());
    }

    let (tx, rx) = oneshot::channel();
    let _ = cmd.send(Command::Close(code, reason, tx)).await;
    let _ = rx.await;
    Ok(())
}
