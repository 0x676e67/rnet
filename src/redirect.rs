use std::fmt::Display;

use pyo3::prelude::*;
use tokio::task;

use crate::{header::HeaderMap, http::StatusCode};

/// Represents the redirect policy for HTTP requests.
#[derive(Clone)]
#[pyclass(frozen, str)]
pub struct Policy(pub wreq::redirect::Policy);

/// A type that holds information on the next request and previous requests
/// in redirect chain.
#[pyclass]
pub struct Attempt {
    #[pyo3(get)]
    status: StatusCode,

    #[pyo3(get)]
    headers: HeaderMap,

    #[pyo3(get)]
    next: String,

    #[pyo3(get)]
    previous: Vec<String>,
}

/// An action to perform when a redirect status code is found.
#[derive(Clone)]
#[pyclass(frozen, str)]
pub struct Action {
    kind: ActionKind,
}

#[derive(Clone)]
enum ActionKind {
    Follow,
    Stop,
    Error(String),
}

// ===== impl Policy =====

#[pymethods]
impl Policy {
    /// Create a [`Policy`] with a maximum number of redirects.
    ///
    /// An `Error` will be returned if the max is reached.
    #[staticmethod]
    pub fn limited(max: usize) -> Self {
        Self(wreq::redirect::Policy::limited(max))
    }

    /// Create a [`Policy`] that does not follow any redirect.
    #[staticmethod]
    pub fn none() -> Self {
        Self(wreq::redirect::Policy::none())
    }

    /// Create a custom `Policy` using the passed function.
    #[staticmethod]
    pub fn custom(callback: Py<PyAny>) -> Self {
        let polciy = wreq::redirect::Policy::custom(move |attempt| {
            let args = Attempt::from(&attempt);
            let kind = task::block_in_place(|| {
                Python::attach(|py| {
                    callback
                        .call1(py, (args,))
                        .and_then(|result| result.extract::<Action>(py).map_err(PyErr::from))
                        .map(|action| action.kind)
                        .unwrap_or_else(|err| ActionKind::Error(err.to_string()))
                })
            });

            match kind {
                ActionKind::Follow => attempt.follow(),
                ActionKind::Stop => attempt.stop(),
                ActionKind::Error(msg) => attempt.error(msg),
            }
        });

        Self(polciy)
    }
}

impl Display for Policy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// ===== impl Attempt =====

#[pymethods]
impl Attempt {
    /// Returns an action meaning the client should follow the next URI.
    #[inline]
    pub fn follow(&self) -> Action {
        Action {
            kind: ActionKind::Follow,
        }
    }

    /// Returns an action meaning the client should not follow the next URI.
    ///
    /// The 30x response will be returned as the result.
    #[inline]
    pub fn stop(&self) -> Action {
        Action {
            kind: ActionKind::Stop,
        }
    }

    /// Returns an action failing the redirect with an error.
    ///
    /// The error will be returned for the result of the sent request.
    #[inline]
    pub fn error(&self, message: String) -> Action {
        Action {
            kind: ActionKind::Error(message),
        }
    }
}

impl From<&wreq::redirect::Attempt<'_>> for Attempt {
    fn from(attempt: &wreq::redirect::Attempt<'_>) -> Self {
        Attempt {
            status: StatusCode(attempt.status()),
            headers: HeaderMap(attempt.headers().clone()),
            next: attempt.uri().to_string(),
            previous: attempt.previous().iter().map(ToString::to_string).collect(),
        }
    }
}

impl Display for Attempt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Attempt {{ status: {}, next: {}, previous: {:?} }}",
            self.status, self.next, self.previous
        )
    }
}

// ===== impl Action =====

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ActionKind::Follow => write!(f, "Action::Follow"),
            ActionKind::Stop => write!(f, "Action::Stop"),
            ActionKind::Error(msg) => write!(f, "Action::Error({})", msg),
        }
    }
}
