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

use std::os::raw::c_int;

use bytes::Bytes;
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

/// A trait to define common buffer behavior
pub trait PyBufferProtocol<'py>: IntoPyObject<'py> {
    fn as_slice(&self) -> &[u8];

    /// Consume self to build a bytes
    fn into_bytes(self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        let buffer = self.into_py_any(py)?;
        unsafe { PyObject::from_owned_ptr_or_err(py, ffi::PyBytes_FromObject(buffer.as_ptr())) }
    }

    /// Consume self to build a bytes
    fn into_bytes_ref(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let buffer = self.into_py_any(py)?;
        let view =
            unsafe { Bound::from_owned_ptr_or_err(py, ffi::PyBytes_FromObject(buffer.as_ptr()))? };
        Ok(view)
    }
}

/// A bytes-like object that implements buffer protocol.
#[pyclass]
pub struct Buffer {
    inner: Vec<u8>,
}

impl PyBufferProtocol<'_> for Buffer {
    fn as_slice(&self) -> &[u8] {
        &self.inner
    }
}

impl Buffer {
    pub fn new(inner: Vec<u8>) -> Self {
        Buffer { inner }
    }
}

#[pymethods]
impl Buffer {
    unsafe fn __getbuffer__(
        slf: PyRefMut<Self>,
        view: *mut ffi::Py_buffer,
        flags: c_int,
    ) -> PyResult<()> {
        fill_buffer_info(slf.as_slice(), slf.as_ptr(), view, flags, slf.py())
    }
}

/// A bytes-like object that implements buffer protocol.
#[pyclass]
pub struct BytesBuffer {
    inner: Bytes,
}

impl PyBufferProtocol<'_> for BytesBuffer {
    fn as_slice(&self) -> &[u8] {
        &self.inner
    }
}

impl BytesBuffer {
    pub fn new(inner: Bytes) -> Self {
        BytesBuffer { inner }
    }
}

#[pymethods]
impl BytesBuffer {
    unsafe fn __getbuffer__(
        slf: PyRefMut<Self>,
        view: *mut ffi::Py_buffer,
        flags: c_int,
    ) -> PyResult<()> {
        fill_buffer_info(slf.as_slice(), slf.as_ptr(), view, flags, slf.py())
    }
}

/// A helper function to fill buffer info
unsafe fn fill_buffer_info(
    bytes: &[u8],
    obj_ptr: *mut pyo3::ffi::PyObject,
    view: *mut ffi::Py_buffer,
    flags: c_int,
    py: Python,
) -> PyResult<()> {
    let ret = ffi::PyBuffer_FillInfo(
        view,
        obj_ptr as *mut _,
        bytes.as_ptr() as *mut _,
        bytes.len().try_into().map_err(|_| PyErr::fetch(py))?,
        1, // read only
        flags,
    );
    if ret == -1 {
        return Err(PyErr::fetch(py));
    }
    Ok(())
}
