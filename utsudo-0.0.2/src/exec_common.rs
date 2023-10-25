/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unused_imports, clashing_extern_declarations)]
use crate::struct_macro::*;
use crate::_PATH_SUDO_BSHELL;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
extern "C" {
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn strncmp(__s1: *const libc::c_char, __s2: *const libc::c_char, __n: size_t) -> libc::c_int;
    fn strlen(__s: *const libc::c_char) -> size_t;
    fn sudo_strsplit_v1(
        str: *const libc::c_char,
        endstr: *const libc::c_char,
        sep: *const libc::c_char,
        last: *mut *const libc::c_char,
    ) -> *const libc::c_char;
    fn memcmp(
        __s1: *const libc::c_void,
        __s2: *const libc::c_void,
        __n: libc::c_ulong,
    ) -> libc::c_int;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn memcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __len: size_t,
    ) -> *mut libc::c_void;
    fn sudo_new_key_val_v1(
        key: *const libc::c_char,
        value: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn sudo_conf_noexec_path_v1() -> *const libc::c_char;
    fn fexecve(
        __fd: libc::c_int,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...);
}
pub const RTLD_PRELOAD_VAR: &str = "LD_PRELOAD";
pub const RTLD_PRELOAD_DELIM: &str = ":";
pub const ENOEXEC: libc::c_int = 8;

#[inline]
unsafe extern "C" fn preload_dso(
    mut envp: *mut *mut libc::c_char,
    mut dso_file: *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut preload: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut env_len: libc::c_int = 0;
    let mut preload_idx: libc::c_int = -1;
    let mut present: bool = false;
    let enabled: bool = true;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
    while !(*envp.offset(env_len as isize)).is_null() {
        if preload_idx == -1
            && strncmp(
                *envp.offset(env_len as isize),
                (RTLD_PRELOAD_VAR.to_string() + ("=")).as_ptr() as *const u8 as *const libc::c_char, //as_ptr()
                std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
            ) == 0
        {
            let mut cp: *const libc::c_char = (*envp.offset(env_len as isize))
                .offset(std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as isize);
            let mut end: *const libc::c_char = cp.offset(strlen(cp) as isize);
            let mut ep: *const libc::c_char = 0 as *const libc::c_char;
            let dso_len: size_t = strlen(dso_file);
            cp = sudo_strsplit_v1(
                cp,
                end,
                RTLD_PRELOAD_DELIM.as_ptr() as *const u8 as *const libc::c_char,
                &mut ep,
            );
            while !cp.is_null() {
                if ep.offset_from(cp) as libc::c_long as size_t == dso_len {
                    if memcmp(
                        cp as *const libc::c_void,
                        dso_file as *const libc::c_void,
                        dso_len,
                    ) == 0
                    {
                        present = true;
                        break;
                    }
                }
                cp = sudo_strsplit_v1(
                    0 as *const libc::c_char,
                    end,
                    RTLD_PRELOAD_DELIM.as_ptr() as *const u8 as *const libc::c_char,
                    &mut ep,
                );
            }
            preload_idx = env_len;
            continue;
        }
        env_len += 1;
    }

    if preload_idx == -1 || !enabled {
        let env_size: libc::c_int = env_len
            + 1
            + (preload_idx == -(1 as libc::c_int)) as libc::c_int
            + enabled as libc::c_int;
        let mut nenvp: *mut *mut libc::c_char = reallocarray(
            0 as *mut libc::c_void,
            env_size as size_t,
            std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        if nenvp.is_null() {
            sudo_fatalx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr() as *const libc::c_char,
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        }
        memcpy(
            nenvp as *mut libc::c_void,
            envp as *const libc::c_void,
            (env_len as libc::c_ulong)
                .wrapping_mul(std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        );
        let ref mut nenvp = *nenvp.offset(env_len as isize);
        *nenvp = 0 as *mut libc::c_char;
        envp = nenvp;
    }
    if !present {
        if preload_idx == -1 {
            preload = sudo_new_key_val_v1(
                RTLD_PRELOAD_VAR.as_ptr() as *const u8 as *const libc::c_char,
                dso_file,
            );
            if preload.is_null() {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    stdext::function_name!().as_ptr() as *const libc::c_char,
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
            let env_len0 = env_len;
            env_len = env_len + 1;
            let ref mut env_len1 = *envp.offset(env_len0 as isize);
            *env_len1 = preload;
            let ref mut envp = *envp.offset(env_len as isize);
            *envp = 0 as *mut libc::c_char;
        } else {
            let mut len: libc::c_int = asprintf(
                &mut preload as *mut *mut libc::c_char,
                b"%s=%s%s%s\0" as *const u8 as *const libc::c_char,
                RTLD_PRELOAD_VAR.as_ptr() as *const u8 as *const libc::c_char,
                dso_file,
                RTLD_PRELOAD_DELIM.as_ptr() as *const u8 as *const libc::c_char,
                *envp.offset(preload_idx as isize),
            );
            if len == -1 {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    stdext::function_name!().as_ptr() as *const libc::c_char,
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
            let ref mut envp = *envp.offset(preload_idx as isize);
            *envp = preload;
        }
    }
    debug_return_ptr!(envp);
}

#[no_mangle]
pub unsafe extern "C" fn disable_execute(
    mut envp: *mut *mut libc::c_char,
    mut dso: *const libc::c_char,
) -> *mut *mut libc::c_char {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
    if !dso.is_null() {
        envp = preload_dso(envp, dso);
    }
    debug_return_ptr!(envp);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_execve(
    mut fd: libc::c_int,
    mut path: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut envp: *mut *mut libc::c_char,
    mut noexec: bool,
) -> libc::c_int {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
    sudo_debug_execve!(SUDO_DEBUG_INFO, path, argv, envp);
    if noexec {
        envp = disable_execute(envp, sudo_conf_noexec_path_v1());
    }
    if fd != -1 {
        fexecve(fd, argv, envp as *const *mut libc::c_char);
    } else {
        execve(path, argv, envp as *const *mut libc::c_char);
    }
    if fd == -1 && *__errno_location() == ENOEXEC {
        let mut argc: libc::c_int = 0;
        let mut nargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        while !(*argv.offset(argc as isize)).is_null() {
            argc += 1;
        }
        nargv = reallocarray(
            0 as *mut libc::c_void,
            (argc + 2) as size_t,
            std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        if !nargv.is_null() {
            let ref mut nargv0 = *nargv.offset(0 as libc::c_int as isize);
            *nargv0 = b"sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            let ref mut nargv1 = *nargv.offset(1 as libc::c_int as isize);
            *nargv1 = path as *mut libc::c_char;
            memcpy(
                nargv.offset(2 as isize) as *mut libc::c_void,
                argv.offset(1 as isize) as *const libc::c_void,
                (argc as libc::c_ulong)
                    .wrapping_mul(std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
            );
            execve(
                _PATH_SUDO_BSHELL!(),
                nargv as *const *mut libc::c_char,
                envp as *const *mut libc::c_char,
            );
            free(nargv as *mut libc::c_void);
        }
    }
    debug_return_int!(-(1 as libc::c_int));
}
