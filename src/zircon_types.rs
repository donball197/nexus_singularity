pub type zx_handle_t = u32;
pub type zx_status_t = i32;
pub const ZX_OK: i32 = 0;
pub const ZX_TIME_INFINITE: i64 = i64::MAX;
pub const ZX_TIMER_SIGNALED: u32 = 1 << 3;
