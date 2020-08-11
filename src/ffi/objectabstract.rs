use crate::ffi::object::*;
use crate::ffi::pyport::Py_ssize_t;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

#[inline]
#[cfg_attr(PyPy, link_name = "PyPyObject_DelAttrString")]
pub unsafe fn PyObject_DelAttrString(o: *mut PyObject, attr_name: *const c_char) -> c_int {
    #[cfg_attr(PyPy, link_name = "PyPyObject_SetAttr")]
    PyObject_SetAttrString(o, attr_name, ptr::null_mut())
}

#[inline]
pub unsafe fn PyObject_DelAttr(o: *mut PyObject, attr_name: *mut PyObject) -> c_int {
    PyObject_SetAttr(o, attr_name, ptr::null_mut())
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyObject_Call")]
    pub fn PyObject_Call(
        callable_object: *mut PyObject,
        args: *mut PyObject,
        kw: *mut PyObject,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_CallObject")]
    pub fn PyObject_CallObject(
        callable_object: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_CallFunction")]
    pub fn PyObject_CallFunction(
        callable_object: *mut PyObject,
        format: *const c_char,
        ...
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_CallMethod")]
    pub fn PyObject_CallMethod(
        o: *mut PyObject,
        method: *const c_char,
        format: *const c_char,
        ...
    ) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPyObject_CallFunctionObjArgs")]
    pub fn PyObject_CallFunctionObjArgs(callable: *mut PyObject, ...) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_CallMethodObjArgs")]
    pub fn PyObject_CallMethodObjArgs(
        o: *mut PyObject,
        method: *mut PyObject,
        ...
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Type")]
    pub fn PyObject_Type(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Size")]
    pub fn PyObject_Size(o: *mut PyObject) -> Py_ssize_t;
}

#[inline]
pub unsafe fn PyObject_Length(o: *mut PyObject) -> Py_ssize_t {
    PyObject_Size(o)
}

extern "C" {
    #[cfg(not(Py_LIMITED_API))]
    #[cfg_attr(PyPy, link_name = "PyPyObject_LengthHint")]
    pub fn PyObject_LengthHint(o: *mut PyObject, arg1: Py_ssize_t) -> Py_ssize_t;

    #[cfg_attr(PyPy, link_name = "PyPyObject_GetItem")]
    pub fn PyObject_GetItem(o: *mut PyObject, key: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_SetItem")]
    pub fn PyObject_SetItem(o: *mut PyObject, key: *mut PyObject, v: *mut PyObject) -> c_int;
    pub fn PyObject_DelItemString(o: *mut PyObject, key: *const c_char) -> c_int;
    pub fn PyObject_DelItem(o: *mut PyObject, key: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_AsCharBuffer")]
    pub fn PyObject_AsCharBuffer(
        obj: *mut PyObject,
        buffer: *mut *const c_char,
        buffer_len: *mut Py_ssize_t,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_CheckReadBuffer")]
    pub fn PyObject_CheckReadBuffer(obj: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_AsReadBuffer")]
    pub fn PyObject_AsReadBuffer(
        obj: *mut PyObject,
        buffer: *mut *const c_void,
        buffer_len: *mut Py_ssize_t,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_AsWriteBuffer")]
    pub fn PyObject_AsWriteBuffer(
        obj: *mut PyObject,
        buffer: *mut *mut c_void,
        buffer_len: *mut Py_ssize_t,
    ) -> c_int;
}

#[cfg(not(Py_LIMITED_API))]
#[inline]
pub unsafe fn PyObject_CheckBuffer(o: *mut PyObject) -> c_int {
    let tp_as_buffer = (*Py_TYPE(o)).tp_as_buffer;
    (!tp_as_buffer.is_null() && (*tp_as_buffer).bf_getbuffer.is_some()) as c_int
}

#[cfg(not(Py_LIMITED_API))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyObject_GetBuffer")]
    pub fn PyObject_GetBuffer(obj: *mut PyObject, view: *mut Py_buffer, flags: c_int) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_GetPointer")]
    pub fn PyBuffer_GetPointer(view: *mut Py_buffer, indices: *mut Py_ssize_t) -> *mut c_void;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_ToContiguous")]
    pub fn PyBuffer_ToContiguous(
        buf: *mut c_void,
        view: *mut Py_buffer,
        len: Py_ssize_t,
        order: c_char,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_FromContiguous")]
    pub fn PyBuffer_FromContiguous(
        view: *mut Py_buffer,
        buf: *mut c_void,
        len: Py_ssize_t,
        order: c_char,
    ) -> c_int;
    pub fn PyObject_CopyData(dest: *mut PyObject, src: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_IsContiguous")]
    pub fn PyBuffer_IsContiguous(view: *const Py_buffer, fort: c_char) -> c_int;
    pub fn PyBuffer_FillContiguousStrides(
        ndims: c_int,
        shape: *mut Py_ssize_t,
        strides: *mut Py_ssize_t,
        itemsize: c_int,
        fort: c_char,
    );
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_FillInfo")]
    pub fn PyBuffer_FillInfo(
        view: *mut Py_buffer,
        o: *mut PyObject,
        buf: *mut c_void,
        len: Py_ssize_t,
        readonly: c_int,
        flags: c_int,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_Release")]
    pub fn PyBuffer_Release(view: *mut Py_buffer);
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyObject_Format")]
    pub fn PyObject_Format(obj: *mut PyObject, format_spec: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_GetIter")]
    pub fn PyObject_GetIter(arg1: *mut PyObject) -> *mut PyObject;
}

#[cfg(not(Py_LIMITED_API))]
#[inline]
#[cfg_attr(PyPy, link_name = "PyPyIter_Check")]
pub unsafe fn PyIter_Check(o: *mut PyObject) -> c_int {
    (match (*Py_TYPE(o)).tp_iternext {
        Some(tp_iternext) => {
            tp_iternext as *const c_void
                != crate::ffi::object::_PyObject_NextNotImplemented as *const c_void
        }
        None => false,
    }) as c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyIter_Next")]
    pub fn PyIter_Next(arg1: *mut PyObject) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPyNumber_Check")]
    pub fn PyNumber_Check(o: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Add")]
    pub fn PyNumber_Add(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Subtract")]
    pub fn PyNumber_Subtract(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Multiply")]
    pub fn PyNumber_Multiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_MatrixMultiply")]
    pub fn PyNumber_MatrixMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_FloorDivide")]
    pub fn PyNumber_FloorDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_TrueDivide")]
    pub fn PyNumber_TrueDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Remainder")]
    pub fn PyNumber_Remainder(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Divmod")]
    pub fn PyNumber_Divmod(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Power")]
    pub fn PyNumber_Power(o1: *mut PyObject, o2: *mut PyObject, o3: *mut PyObject)
        -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Negative")]
    pub fn PyNumber_Negative(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Positive")]
    pub fn PyNumber_Positive(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Absolute")]
    pub fn PyNumber_Absolute(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Invert")]
    pub fn PyNumber_Invert(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Lshift")]
    pub fn PyNumber_Lshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Rshift")]
    pub fn PyNumber_Rshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_And")]
    pub fn PyNumber_And(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Xor")]
    pub fn PyNumber_Xor(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Or")]
    pub fn PyNumber_Or(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
}

#[cfg(not(Py_LIMITED_API))]
#[inline]
#[cfg_attr(PyPy, link_name = "PyPyIndex_Check")]
pub unsafe fn PyIndex_Check(o: *mut PyObject) -> c_int {
    let tp_as_number = (*Py_TYPE(o)).tp_as_number;
    (!tp_as_number.is_null() && (*tp_as_number).nb_index.is_some()) as c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Index")]
    pub fn PyNumber_Index(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_AsSsize_t")]
    pub fn PyNumber_AsSsize_t(o: *mut PyObject, exc: *mut PyObject) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Long")]
    pub fn PyNumber_Long(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_Float")]
    pub fn PyNumber_Float(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceAdd")]
    pub fn PyNumber_InPlaceAdd(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceSubtract")]
    pub fn PyNumber_InPlaceSubtract(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceMultiply")]
    pub fn PyNumber_InPlaceMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceMatrixMultiply")]
    pub fn PyNumber_InPlaceMatrixMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceFloorDivide")]
    pub fn PyNumber_InPlaceFloorDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceTrueDivide")]
    pub fn PyNumber_InPlaceTrueDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceRemainder")]
    pub fn PyNumber_InPlaceRemainder(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlacePower")]
    pub fn PyNumber_InPlacePower(
        o1: *mut PyObject,
        o2: *mut PyObject,
        o3: *mut PyObject,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceLshift")]
    pub fn PyNumber_InPlaceLshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceRshift")]
    pub fn PyNumber_InPlaceRshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceAnd")]
    pub fn PyNumber_InPlaceAnd(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceXor")]
    pub fn PyNumber_InPlaceXor(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyNumber_InPlaceOr")]
    pub fn PyNumber_InPlaceOr(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_ToBase(n: *mut PyObject, base: c_int) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPySequence_Check")]
    pub fn PySequence_Check(o: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPySequence_Size")]
    pub fn PySequence_Size(o: *mut PyObject) -> Py_ssize_t;
}

#[inline]
#[cfg_attr(PyPy, link_name = "PyPySequence_Length")]
pub unsafe fn PySequence_Length(o: *mut PyObject) -> Py_ssize_t {
    PySequence_Size(o)
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPySequence_Concat")]
    pub fn PySequence_Concat(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPySequence_Repeat")]
    pub fn PySequence_Repeat(o: *mut PyObject, count: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPySequence_GetItem")]
    pub fn PySequence_GetItem(o: *mut PyObject, i: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPySequence_GetSlice")]
    pub fn PySequence_GetSlice(o: *mut PyObject, i1: Py_ssize_t, i2: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPySequence_SetItem")]
    pub fn PySequence_SetItem(o: *mut PyObject, i: Py_ssize_t, v: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPySequence_DelItem")]
    pub fn PySequence_DelItem(o: *mut PyObject, i: Py_ssize_t) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPySequence_SetSlice")]
    pub fn PySequence_SetSlice(
        o: *mut PyObject,
        i1: Py_ssize_t,
        i2: Py_ssize_t,
        v: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPySequence_DelSlice")]
    pub fn PySequence_DelSlice(o: *mut PyObject, i1: Py_ssize_t, i2: Py_ssize_t) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPySequence_Tuple")]
    pub fn PySequence_Tuple(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPySequence_List")]
    pub fn PySequence_List(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPySequence_Fast")]
    pub fn PySequence_Fast(o: *mut PyObject, m: *const c_char) -> *mut PyObject;
    // TODO: PySequence_Fast macros
    pub fn PySequence_Count(o: *mut PyObject, value: *mut PyObject) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPySequence_Contains")]
    pub fn PySequence_Contains(seq: *mut PyObject, ob: *mut PyObject) -> c_int;
}

#[inline]
pub unsafe fn PySequence_In(o: *mut PyObject, value: *mut PyObject) -> c_int {
    PySequence_Contains(o, value)
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPySequence_Index")]
    pub fn PySequence_Index(o: *mut PyObject, value: *mut PyObject) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPySequence_InPlaceConcat")]
    pub fn PySequence_InPlaceConcat(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPySequence_InPlaceRepeat")]
    pub fn PySequence_InPlaceRepeat(o: *mut PyObject, count: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyMapping_Check")]
    pub fn PyMapping_Check(o: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyMapping_Size")]
    pub fn PyMapping_Size(o: *mut PyObject) -> Py_ssize_t;
}

#[inline]
#[cfg_attr(PyPy, link_name = "PyPyMapping_Length")]
pub unsafe fn PyMapping_Length(o: *mut PyObject) -> Py_ssize_t {
    PyMapping_Size(o)
}

#[inline]
pub unsafe fn PyMapping_DelItemString(o: *mut PyObject, key: *mut c_char) -> c_int {
    PyObject_DelItemString(o, key)
}

#[inline]
pub unsafe fn PyMapping_DelItem(o: *mut PyObject, key: *mut PyObject) -> c_int {
    PyObject_DelItem(o, key)
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyMapping_HasKeyString")]
    pub fn PyMapping_HasKeyString(o: *mut PyObject, key: *const c_char) -> c_int;
    pub fn PyMapping_HasKey(o: *mut PyObject, key: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyMapping_Keys")]
    pub fn PyMapping_Keys(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyMapping_Values")]
    pub fn PyMapping_Values(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyMapping_Items")]
    pub fn PyMapping_Items(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyMapping_GetItemString")]
    pub fn PyMapping_GetItemString(o: *mut PyObject, key: *const c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyMapping_SetItemString")]
    pub fn PyMapping_SetItemString(
        o: *mut PyObject,
        key: *const c_char,
        value: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_IsInstance")]
    pub fn PyObject_IsInstance(object: *mut PyObject, typeorclass: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_IsSubclass")]
    pub fn PyObject_IsSubclass(object: *mut PyObject, typeorclass: *mut PyObject) -> c_int;
}