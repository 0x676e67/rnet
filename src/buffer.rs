// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

// This ignores bug warnings for macro-generated code
#![allow(unsafe_op_in_unsafe_fn)]

use std::os::raw::c_int;

use bytes::Bytes;
use pyo3::{IntoPyObjectExt, ffi, prelude::*};
use wreq::header::{HeaderName, HeaderValue};

pub struct PyBuffer(BufferView);

#[pyclass(frozen)]
struct BufferView(Bytes);

impl<'a> IntoPyObject<'a> for PyBuffer {
    type Target = PyAny;
    type Output = Bound<'a, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'a>) -> Result<Self::Output, Self::Error> {
        let buffer = self.0.into_py_any(py)?;
        let view =
            unsafe { Bound::from_owned_ptr_or_err(py, ffi::PyBytes_FromObject(buffer.as_ptr()))? };
        Ok(view)
    }
}

#[pymethods]
impl BufferView {
    unsafe fn __getbuffer__(
        slf: PyRef<Self>,
        view: *mut ffi::Py_buffer,
        flags: c_int,
    ) -> PyResult<()> {
        unsafe { fill_buffer_info(&slf.0, slf.as_ptr(), view, flags, slf.py()) }
    }
}

impl From<Vec<u8>> for PyBuffer {
    fn from(value: Vec<u8>) -> Self {
        Self::from(Bytes::from(value))
    }
}

impl From<&Bytes> for PyBuffer {
    fn from(value: &Bytes) -> Self {
        Self::from(value.clone())
    }
}

impl From<Bytes> for PyBuffer {
    fn from(value: Bytes) -> Self {
        PyBuffer(BufferView(value))
    }
}

impl From<HeaderName> for PyBuffer {
    fn from(value: HeaderName) -> Self {
        Self::from(Bytes::from_owner(value))
    }
}

impl From<HeaderValue> for PyBuffer {
    fn from(value: HeaderValue) -> Self {
        Self::from(Bytes::from_owner(value))
    }
}

/// A helper function to fill buffer info
unsafe fn fill_buffer_info(
    bytes: &[u8],
    obj_ptr: *mut ffi::PyObject,
    view: *mut ffi::Py_buffer,
    flags: c_int,
    py: Python,
) -> PyResult<()> {
    let ret = unsafe {
        ffi::PyBuffer_FillInfo(
            view,
            obj_ptr as *mut _,
            bytes.as_ptr() as *mut _,
            bytes.len() as _,
            1,
            flags,
        )
    };
    if ret == -1 {
        return Err(PyErr::fetch(py));
    }
    Ok(())
}
