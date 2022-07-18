//! Process management syscalls

use crate::config::{MAX_SYSCALL_NUM, PAGE_SIZE};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, current_task_status, current_task_time, current_task_syscall_times, task_mmap, task_munmap};
use crate::timer::get_time_us;
use crate::mm::get_phy_addr_from;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let ts = get_phy_addr_from(current_user_token(), (ts as usize).into()) as *mut TimeVal;
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }

    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    task_mmap(start, len, port)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    task_munmap(start, len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let ti = get_phy_addr_from(current_user_token(), (ti as usize).into()) as *mut TaskInfo;
    unsafe {
        *ti = TaskInfo {
            status: current_task_status(),
            syscall_times: current_task_syscall_times(),
            time: current_task_time(),
        }
    }
    
    0
}
