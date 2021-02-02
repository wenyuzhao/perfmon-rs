use core::slice;
use std::{collections::HashMap, env, ffi::CStr, os::raw::c_char, sync::atomic::{AtomicBool, Ordering}};

#[repr(C)]
struct EventResult {
    name: *const c_char,
    pub value: u64,
}

impl EventResult {
    fn name(&self) -> String {
        let c_str: &CStr = unsafe { CStr::from_ptr(self.name) };
        let str_slice: &str = c_str.to_str().unwrap();
        return str_slice.to_owned();
    }
}

extern {
    fn perfmon_prepare();
    fn perfmon_begin();
    fn perfmon_end(size: *mut i32) -> *const EventResult;
}

static PERFMON_PREPARED: AtomicBool = AtomicBool::new(false);

pub fn initialize() {
    PERFMON_PREPARED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).expect("perfmon can only be initialized once.");
    if env::var("PERF_EVENTS").is_err() {
        env::set_var("PERF_EVENTS", "");
    }
    unsafe {
        perfmon_prepare();
    }
}

#[inline(always)]
pub fn begin() {
    unsafe {
        perfmon_begin();
    }
}

#[inline(always)]
pub fn end() -> HashMap<String, u64> {
    let events = unsafe {
        let mut count = 0i32;
        let data = perfmon_end(&mut count);
        slice::from_raw_parts(data, count as usize)
    };
    let mut events_map = HashMap::new();
    for event in events {
        events_map.insert(event.name(), event.value);
    }
    return events_map;
}
