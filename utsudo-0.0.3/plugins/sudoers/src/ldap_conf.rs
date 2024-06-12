/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    improper_ctypes,
    unused_variables,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//crate
use crate::common::*;
use libc::strdup;
//include extern C
extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn ber_set_option(
        item: *mut libc::c_void,
        option: libc::c_int,
        invalue: *const libc::c_void,
    ) -> libc::c_int;
    fn ldap_set_option(
        ld: *mut LDAP,
        option: libc::c_int,
        invalue: *const libc::c_void,
    ) -> libc::c_int;
    fn ldap_err2string(err: libc::c_int) -> *mut libc::c_char;
    fn sudo_parseln_v2(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        lineno: *mut libc::c_uint,
        fp: *mut FILE,
        flags: libc::c_int,
    ) -> ssize_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut path_ldap_conf: *const libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    static mut path_ldap_secret: *const libc::c_char;
    fn sudo_strtobool_v1(str: *const libc::c_char) -> libc::c_int;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_strlcat(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn base64_decode(str: *const libc::c_char, dst: *mut libc::c_uchar, dsize: size_t) -> size_t;
}
//be used in ldap.rs structs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config_table {
    pub conf_str: *const libc::c_char,
    pub type_0: libc::c_int,
    pub opt_val: libc::c_int,
    pub valp: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config {
    pub port: libc::c_int,
    pub version: libc::c_int,
    pub debug: libc::c_int,
    pub ldap_debug: libc::c_int,
    pub tls_checkpeer: libc::c_int,
    pub tls_reqcert: libc::c_int,
    pub timelimit: libc::c_int,
    pub timeout: libc::c_int,
    pub bind_timelimit: libc::c_int,
    pub use_sasl: libc::c_int,
    pub rootuse_sasl: libc::c_int,
    pub ssl_mode: libc::c_int,
    pub timed: libc::c_int,
    pub deref: libc::c_int,
    pub host: *mut libc::c_char,
    pub uri: ldap_config_str_list,
    pub binddn: *mut libc::c_char,
    pub bindpw: *mut libc::c_char,
    pub rootbinddn: *mut libc::c_char,
    pub base: ldap_config_str_list,
    pub netgroup_base: ldap_config_str_list,
    pub search_filter: *mut libc::c_char,
    pub netgroup_search_filter: *mut libc::c_char,
    pub ssl: *mut libc::c_char,
    pub tls_cacertfile: *mut libc::c_char,
    pub tls_cacertdir: *mut libc::c_char,
    pub tls_random_file: *mut libc::c_char,
    pub tls_cipher_suite: *mut libc::c_char,
    pub tls_certfile: *mut libc::c_char,
    pub tls_keyfile: *mut libc::c_char,
    pub tls_keypw: *mut libc::c_char,
    pub sasl_mech: *mut libc::c_char,
    pub sasl_auth_id: *mut libc::c_char,
    pub rootsasl_auth_id: *mut libc::c_char,
    pub sasl_secprops: *mut libc::c_char,
    pub krb5_ccname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config_str {
    pub entries: ldap_config_str_mid,
    pub val: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config_str_mid {
    pub stqe_next: *mut ldap_config_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config_str_list {
    pub stqh_first: *mut ldap_config_str,
    pub stqh_last: *mut *mut ldap_config_str,
}
//constants && init
pub const CONF_DEREF_VAL: libc::c_int = 5;
pub const CONF_REQCERT_VAL: libc::c_int = 6;
pub const CONF_BOOL: libc::c_int = 0;
pub const CONF_INT: libc::c_int = 1;
pub const CONF_STR: libc::c_int = 2;
pub const CONF_LIST_STR: libc::c_int = 4;
pub const PARSELN_COMM_BOL: libc::c_int = 1;
pub const PARSELN_CONT_IGN: libc::c_int = 2;
pub const LDAP_OPT_X_TLS_NEVER: libc::c_int = 0;
pub const LDAP_OPT_X_TLS_ALLOW: libc::c_int = 3;
pub const LDAP_OPT_X_TLS_TRY: libc::c_int = 4;
pub const LDAP_OPT_X_TLS_HARD: libc::c_int = 1;
pub const LDAP_OPT_X_TLS_DEMAND: libc::c_int = 2;
pub const SUDO_LDAP_STARTTLS: libc::c_int = 2;
pub const SUDO_LDAP_SSL: libc::c_int = 1;
pub const LDAPS_PORT: libc::c_int = 636;
pub const LDAP_PORT: libc::c_int = 389;
pub const LDAP_SUCCESS: libc::c_int = 0;
pub const LDAP_OPT_SUCCESS: libc::c_int = 0;
pub const LBER_OPT_DEBUG_LEVEL: libc::c_int = 2;
pub const LDAP_OPT_TIMEOUT: libc::c_int = 20482;
pub const LDAP_OPT_NETWORK_TIMEOUT: libc::c_int = 20485;
pub const LDAP_OPT_X_TLS: libc::c_int = 24576;
#[no_mangle]
pub static mut ldap_conf: ldap_config = ldap_config {
    port: 0,
    version: 0,
    debug: 0,
    ldap_debug: 0,
    tls_checkpeer: 0,
    tls_reqcert: 0,
    timelimit: 0,
    timeout: 0,
    bind_timelimit: 0,
    use_sasl: 0,
    rootuse_sasl: 0,
    ssl_mode: 0,
    timed: 0,
    deref: 0,
    host: 0 as *const libc::c_char as *mut libc::c_char,
    uri: ldap_config_str_list {
        stqh_first: 0 as *const ldap_config_str as *mut ldap_config_str,
        stqh_last: 0 as *const *mut ldap_config_str as *mut *mut ldap_config_str,
    },
    binddn: 0 as *const libc::c_char as *mut libc::c_char,
    bindpw: 0 as *const libc::c_char as *mut libc::c_char,
    rootbinddn: 0 as *const libc::c_char as *mut libc::c_char,
    base: ldap_config_str_list {
        stqh_first: 0 as *const ldap_config_str as *mut ldap_config_str,
        stqh_last: 0 as *const *mut ldap_config_str as *mut *mut ldap_config_str,
    },
    netgroup_base: ldap_config_str_list {
        stqh_first: 0 as *const ldap_config_str as *mut ldap_config_str,
        stqh_last: 0 as *const *mut ldap_config_str as *mut *mut ldap_config_str,
    },
    search_filter: 0 as *const libc::c_char as *mut libc::c_char,
    netgroup_search_filter: 0 as *const libc::c_char as *mut libc::c_char,
    ssl: 0 as *const libc::c_char as *mut libc::c_char,
    tls_cacertfile: 0 as *const libc::c_char as *mut libc::c_char,
    tls_cacertdir: 0 as *const libc::c_char as *mut libc::c_char,
    tls_random_file: 0 as *const libc::c_char as *mut libc::c_char,
    tls_cipher_suite: 0 as *const libc::c_char as *mut libc::c_char,
    tls_certfile: 0 as *const libc::c_char as *mut libc::c_char,
    tls_keyfile: 0 as *const libc::c_char as *mut libc::c_char,
    tls_keypw: 0 as *const libc::c_char as *mut libc::c_char,
    sasl_mech: 0 as *const libc::c_char as *mut libc::c_char,
    sasl_auth_id: 0 as *const libc::c_char as *mut libc::c_char,
    rootsasl_auth_id: 0 as *const libc::c_char as *mut libc::c_char,
    sasl_secprops: 0 as *const libc::c_char as *mut libc::c_char,
    krb5_ccname: 0 as *const libc::c_char as *mut libc::c_char,
};


static mut ldap_conf_global: [ldap_config_table; 31] = [ldap_config_table {
    conf_str: 0 as *const libc::c_char,
    type_0: 0,
    opt_val: 0,
    valp: 0 as *mut libc::c_void,
}; 31];
static mut ldap_conf_conn: [ldap_config_table; 8] = [ldap_config_table {
    conf_str: 0 as *const libc::c_char,
    type_0: 0,
    opt_val: 0,
    valp: 0 as *mut libc::c_void,
}; 8];
//pub static mut ldap_conf_global : [ldap_config_table;31] = unsafe {
//pub static mut ldap_conf_global : [ldap_config_table;31] =
unsafe extern "C" fn run_static_initializers() {
    ldap_conf_global = [
        {
            let mut init = ldap_config_table {
                conf_str: b"sudoers_debug\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.debug as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"host\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.host as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"port\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.port as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"ssl\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.ssl as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sslpath\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.tls_certfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"uri\0" as *const u8 as *const libc::c_char,
                type_0: 4 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.uri as *mut ldap_config_str_list as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"debug\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: 0x5001 as libc::c_int,
                valp: &mut ldap_conf.ldap_debug as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_checkpeer\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                opt_val: 0x6006 as libc::c_int,
                valp: &mut ldap_conf.tls_checkpeer as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_reqcert\0" as *const u8 as *const libc::c_char,
                type_0: 6 as libc::c_int,
                opt_val: 0x6006 as libc::c_int,
                valp: &mut ldap_conf.tls_reqcert as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_cacertfile\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6002 as libc::c_int,
                valp: &mut ldap_conf.tls_cacertfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_cacert\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6002 as libc::c_int,
                valp: &mut ldap_conf.tls_cacertfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_cacertdir\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6003 as libc::c_int,
                valp: &mut ldap_conf.tls_cacertdir as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_randfile\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6009 as libc::c_int,
                valp: &mut ldap_conf.tls_random_file as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_ciphers\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6008 as libc::c_int,
                valp: &mut ldap_conf.tls_cipher_suite as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_cert\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6004 as libc::c_int,
                valp: &mut ldap_conf.tls_certfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_key\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6005 as libc::c_int,
                valp: &mut ldap_conf.tls_keyfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"binddn\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.binddn as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"bindpw\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.bindpw as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"rootbinddn\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.rootbinddn as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sudoers_base\0" as *const u8 as *const libc::c_char,
                type_0: 4 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.base as *mut ldap_config_str_list as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sudoers_timed\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.timed as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sudoers_search_filter\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.search_filter as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"netgroup_base\0" as *const u8 as *const libc::c_char,
                type_0: 4 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.netgroup_base as *mut ldap_config_str_list
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"netgroup_search_filter\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.netgroup_search_filter as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"use_sasl\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.use_sasl as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sasl_mech\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.sasl_mech as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sasl_auth_id\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.sasl_auth_id as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"rootuse_sasl\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.rootuse_sasl as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"rootsasl_auth_id\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.rootsasl_auth_id as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"krb5_ccname\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.krb5_ccname as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: 0 as *const libc::c_char,
                type_0: 0,
                opt_val: 0,
                valp: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    //}
    
 //#[no_mangle]
    //pub static mut ldap_conf_conn : [ldap_config_table;8] = unsafe {
    //unsafe extern "C" fn run_static_initializers() {
    ldap_conf_conn = [
        //pub static mut ldap_conf_conn : [ldap_config_table;8] = unsafe {
        //    [
        {
            let mut init = ldap_config_table {
                conf_str: b"ldap_version\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: 0x11 as libc::c_int,
                valp: &mut ldap_conf.version as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"bind_timelimit\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.bind_timelimit as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"network_timeout\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.bind_timelimit as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"timelimit\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: 0x4 as libc::c_int,
                valp: &mut ldap_conf.timelimit as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"timeout\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.timeout as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"deref\0" as *const u8 as *const libc::c_char,
                type_0: 5 as libc::c_int,
                opt_val: 0x2 as libc::c_int,
                valp: &mut ldap_conf.deref as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sasl_secprops\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6106 as libc::c_int,
                valp: &mut ldap_conf.sasl_secprops as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: 0 as *const libc::c_char,
                type_0: 0,
                opt_val: 0,
                valp: 0 as *mut libc::c_void,
            };
            init
        },
    ];
}

//functions
unsafe extern "C" fn sudo_ldap_conf_add_ports() -> bool {
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut defport: [libc::c_char; 13] = [0; 13];
    let mut hostbuf: [libc::c_char; 4096] = [0; 4096];
    let mut len: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_LDAP!());
    hostbuf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    len = snprintf(
        defport.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
        b":%d\0" as *const u8 as *const libc::c_char,
        ldap_conf.port,
    );
    if len < 0 as libc::c_int
        || len as libc::c_long
            >= ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as ssize_t
    {
        sudo_warnx!(
            b"sudo_ldap_conf_add_ports: port too large\0" as *const u8 as *const libc::c_char,
        );
        debug_return_bool!(false);
    }
    host = strtok_r(
        ldap_conf.host,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut last,
    );
    loop {
        if host.is_null() {
            break;
        }
        if hostbuf[0 as libc::c_int as usize] as libc::c_int != '\u{0}' as i32 {
            if sudo_strlcat(
                hostbuf.as_mut_ptr(),
                b" \0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            ) >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
            {
                sudo_warnx!(
                    b"internal error, %s overflow\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                );
                debug_return_bool!(false);
            }
        }
        if sudo_strlcat(
            hostbuf.as_mut_ptr(),
            host,
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        ) >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
        {
            sudo_warnx!(
                b"internal error, %s overflow\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
            );
            debug_return_bool!(false);
        }
        /* Append port if there is not one already. */
        //
        port = strrchr(host, ':' as i32);
        if port.is_null()
            || *(*__ctype_b_loc()).offset(
                *port.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int as isize
            ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            if sudo_strlcat(
                hostbuf.as_mut_ptr(),
                defport.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            ) >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
            {
                sudo_warnx!(
                    b"internal error, %s overflow\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                );
                debug_return_bool!(false);
            }
        }
        host = strtok_r(
            0 as *mut libc::c_char,
            b" \t\0" as *const u8 as *const libc::c_char,
            &mut last,
        );
    }
    free(ldap_conf.host as *mut libc::c_void);
    ldap_conf.host = strdup(hostbuf.as_mut_ptr());
    if (ldap_conf.host).is_null() {
        sudo_warnx!(
            b"internal error, %s overflow\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
        );
    }
    debug_return_bool!(!(ldap_conf.host).is_null());
    //overflow:
    //sudo_warnx!(b"internal error, %s overflow\0" as *const u8 as *const libc::c_char,get_function_name!(),);
    //debug_return_bool!(false);
}

unsafe extern "C" fn sudo_ldap_decode_secret(mut secret: *const libc::c_char) -> *mut libc::c_char {
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    let mut reslen: size_t = 0;
    debug_decl!(SUDOERS_DEBUG_LDAP!());
    if strncasecmp(
        secret,
        b"base64:\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        secret = secret.offset(
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
        reslen = (strlen(secret))
            .wrapping_add(3 as libc::c_int as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong);
        result =
            malloc(reslen.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_uchar;
        if result.is_null() {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        } else {
            len = base64_decode(secret, result, reslen);
            if len == -(1 as libc::c_int) as size_t {
                free(result as *mut libc::c_void);
                result = 0 as *mut libc::c_uchar;
            } else {
                *result.offset(len as isize) = '\u{0}' as i32 as libc::c_uchar;
            }
        }
    }
    debug_return_str!(result as *mut libc::c_char);
}
unsafe extern "C" fn sudo_ldap_read_secret(mut path: *const libc::c_char) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linesize: size_t = 0 as libc::c_int as size_t;
    let mut len: ssize_t = 0;
    debug_decl!(SUDOERS_DEBUG_LDAP!());
    fp = fopen(path_ldap_secret, b"r\0" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        len = getdelim(&mut line, &mut linesize, '\n' as i32, fp);
        if len != -(1 as libc::c_int) as libc::c_long {
            while len > 0 as libc::c_int as libc::c_long
                && *line.offset((len - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
                    == '\n' as i32
            {
                len -= 1;
                *line.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
            }
            free(ldap_conf.bindpw as *mut libc::c_void);
            ldap_conf.bindpw = sudo_ldap_decode_secret(line);
            if (ldap_conf.bindpw).is_null() {
                ldap_conf.bindpw = line;
                line = 0 as *mut libc::c_char;
            }
            free(ldap_conf.binddn as *mut libc::c_void);
            ldap_conf.binddn = ldap_conf.rootbinddn;
            ldap_conf.rootbinddn = 0 as *mut libc::c_char;
        }
        fclose(fp);
        free(line as *mut libc::c_void);
    }
    debug_return!();
}
unsafe extern "C" fn sudo_ldap_parse_keyword(
    mut keyword: *const libc::c_char,
    mut value: *const libc::c_char,
    mut table: *mut ldap_config_table,
) -> bool {
    let mut cur: *mut ldap_config_table = 0 as *mut ldap_config_table;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    debug_decl!(SUDOERS_DEBUG_LDAP!());
    cur = table;
    while !((*cur).conf_str).is_null() {
        if strcasecmp(keyword, (*cur).conf_str) == 0 as libc::c_int {
            match (*cur).type_0 {
                CONF_DEREF_VAL => {
                    if strcasecmp(value, b"searching\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *((*cur).valp as *mut libc::c_int) = 0x1 as libc::c_int;
                    } else if strcasecmp(value, b"finding\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *((*cur).valp as *mut libc::c_int) = 0x2 as libc::c_int;
                    } else if strcasecmp(value, b"always\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *((*cur).valp as *mut libc::c_int) = 0x3 as libc::c_int;
                    } else {
                        *((*cur).valp as *mut libc::c_int) = 0 as libc::c_int;
                    }
                }
                CONF_REQCERT_VAL => {
                    if strcasecmp(value, b"never\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *((*cur).valp as *mut libc::c_int) = 0 as libc::c_int;
                    } else if strcasecmp(value, b"allow\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *((*cur).valp as *mut libc::c_int) = 3 as libc::c_int;
                    } else if strcasecmp(value, b"try\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *((*cur).valp as *mut libc::c_int) = 4 as libc::c_int;
                    } else if strcasecmp(value, b"hard\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *((*cur).valp as *mut libc::c_int) = 1 as libc::c_int;
                    } else if strcasecmp(value, b"demand\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *((*cur).valp as *mut libc::c_int) = 2 as libc::c_int;
                    }
                }
                CONF_BOOL => {
                    *((*cur).valp as *mut libc::c_int) =
                        (sudo_strtobool_v1(value) == 1 as libc::c_int) as libc::c_int;
                }
                CONF_INT => {
                    *((*cur).valp as *mut libc::c_int) = sudo_strtonum(
                        value,
                        (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong,
                        2147483647 as libc::c_int as libc::c_longlong,
                        &mut errstr,
                    ) as libc::c_int;
                    if !errstr.is_null() {
                        sudo_warnx!(
                            b"%s: %s: %s: %s\0" as *const u8 as *const libc::c_char,
                            path_ldap_conf,
                            keyword,
                            value,
                            errstr,
                        );
                    }
                }
                CONF_STR => {
                    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                    free(*((*cur).valp as *mut *mut libc::c_char) as *mut libc::c_void);
                    if *value as libc::c_int != 0 && {
                        cp = strdup(value);
                        cp.is_null()
                    } {
                        sudo_warnx!(
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            get_function_name!(),
                            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                        );
                        debug_return_bool!(false);
                    }
                    let ref mut fresh0 = *((*cur).valp as *mut *mut libc::c_char);
                    *fresh0 = cp;
                }
                CONF_LIST_STR => {
                    let mut head: *mut ldap_config_str_list = 0 as *mut ldap_config_str_list;
                    let mut str0: *mut ldap_config_str = 0 as *mut ldap_config_str;
                    let mut len: size_t = strlen(value);
                    if len > 0 as libc::c_int as libc::c_ulong {
                        head = (*cur).valp as *mut ldap_config_str_list;
                        str0 = malloc(
                            (::std::mem::size_of::<ldap_config_str>() as libc::c_ulong)
                                .wrapping_add(len),
                        ) as *mut ldap_config_str;
                        if str0.is_null() {
                            sudo_warnx!(
                                b"%s: %s\0" as *const u8 as *const libc::c_char,
                                get_function_name!(),
                                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                            );
                            debug_return_bool!(false);
                        }
                        memcpy(
                            ((*str0).val).as_mut_ptr() as *mut libc::c_void,
                            value as *const libc::c_void,
                            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        let ref mut fresh1 = (*str0).entries.stqe_next;
                        *fresh1 = 0 as *mut ldap_config_str;
                        let ref mut fresh2 = *(*head).stqh_last;
                        *fresh2 = str0;
                        let ref mut fresh3 = (*head).stqh_last;
                        *fresh3 = &mut (*str0).entries.stqe_next;
                    }
                }
                _ => {}
            } // match
            debug_return_bool!(true);
        } // if
        cur = cur.offset(1);
    } //while
    debug_return_bool!(false);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_krb5_ccname_path(
    mut old_ccname: *const libc::c_char,
) -> *const libc::c_char {
    let mut ccname: *const libc::c_char = old_ccname;
    debug_decl!(SUDOERS_DEBUG_LDAP!());
    match *ccname.offset(0 as libc::c_int as isize) as u8 as char {
        'F' | 'f' => {
            if strncasecmp(
                ccname,
                b"FILE:\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                ccname = ccname.offset(5 as libc::c_int as isize);
            }
        }
        'W' | 'w' => {
            if strncasecmp(
                ccname,
                b"WRFILE:\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                ccname = ccname.offset(7 as libc::c_int as isize);
            }
        }
        _ => {}
    }
    sudo_debug_printf!(
        SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
        b"ccache %s -> %s\0" as *const u8 as *const libc::c_char,
        old_ccname,
        ccname
    );
    debug_return_const_str!(if *ccname as libc::c_int == '/' as i32 {
        ccname
    } else {
        0 as *const libc::c_char
    });
}
unsafe extern "C" fn sudo_check_krb5_ccname(mut ccname: *const libc::c_char) -> bool {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut ccname_path: *const libc::c_char = 0 as *const libc::c_char;
    debug_decl!(SUDOERS_DEBUG_LDAP!());
    ccname_path = sudo_krb5_ccname_path(ccname);
    if ccname_path.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
            b"unsupported krb5 credential cache path: %s\0" as *const u8 as *const libc::c_char,
            ccname
        );
        debug_return_bool!(false);
    }
    fd = open(
        ccname_path,
        0 as libc::c_int | 0o4000 as libc::c_int,
        0 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
            b"unable to open krb5 credential cache: %s\0" as *const u8 as *const libc::c_char,
            ccname_path
        );
        debug_return_bool!(false);
    }
    close(fd);
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"using krb5 credential cache: %s\0" as *const u8 as *const libc::c_char,
        ccname_path
    );
    debug_return_bool!(true);
}
#[no_mangle]
pub unsafe extern "C" fn sudo_ldap_read_config() -> bool {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keyword: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut conf_str: *mut ldap_config_str = 0 as *mut ldap_config_str;
    let mut linesize: size_t = 0 as libc::c_int as size_t;
    let mut fp: *mut FILE = 0 as *mut FILE;
    debug_decl!(SUDOERS_DEBUG_LDAP!());
    ldap_conf.version = 3 as libc::c_int;
    ldap_conf.port = -(1 as libc::c_int);
    ldap_conf.tls_checkpeer = -(1 as libc::c_int);
    ldap_conf.tls_reqcert = -(1 as libc::c_int);
    ldap_conf.timelimit = -(1 as libc::c_int);
    ldap_conf.timeout = -(1 as libc::c_int);
    ldap_conf.bind_timelimit = -(1 as libc::c_int);
    ldap_conf.use_sasl = -(1 as libc::c_int);
    ldap_conf.rootuse_sasl = -(1 as libc::c_int);
    ldap_conf.deref = -(1 as libc::c_int);
    ldap_conf.search_filter =
        strdup(b"(objectClass=sudoRole)\0" as *const u8 as *const libc::c_char);
    ldap_conf.netgroup_search_filter =
        strdup(b"(objectClass=nisNetgroup)\0" as *const u8 as *const libc::c_char);
    ldap_conf.uri.stqh_first = 0 as *mut ldap_config_str;
    ldap_conf.uri.stqh_last = &mut ldap_conf.uri.stqh_first;
    ldap_conf.base.stqh_first = 0 as *mut ldap_config_str;
    ldap_conf.base.stqh_last = &mut ldap_conf.base.stqh_first;
    ldap_conf.netgroup_base.stqh_first = 0 as *mut ldap_config_str;
    ldap_conf.netgroup_base.stqh_last = &mut ldap_conf.netgroup_base.stqh_first;
    if (ldap_conf.search_filter).is_null() || (ldap_conf.netgroup_search_filter).is_null() {
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    fp = fopen(path_ldap_conf, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        debug_return_bool!(false);
    }
    while sudo_parseln_v2(
        &mut line,
        &mut linesize,
        0 as *mut libc::c_uint,
        fp,
        0x1 as libc::c_int | 0x2 as libc::c_int,
    ) != -(1 as libc::c_int) as libc::c_long
    {
        if *line as libc::c_int == '\u{0}' as i32 {
            continue;
        }
        cp = line;
        keyword = cp;
        while *cp as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            cp = cp.offset(1);
        }
        if *cp != 0 {
            let fresh4 = cp;
            cp = cp.offset(1);
            *fresh4 = '\u{0}' as i32 as libc::c_char;
        }
        while *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            cp = cp.offset(1);
        }
        value = cp;
        if !sudo_ldap_parse_keyword(keyword, value, ldap_conf_global.as_mut_ptr()) {
            sudo_ldap_parse_keyword(keyword, value, ldap_conf_conn.as_mut_ptr());
        }
    } //end of while
    free(line as *mut libc::c_void);
    fclose(fp);
    if (ldap_conf.host).is_null() {
        ldap_conf.host = strdup(b"localhost\0" as *const u8 as *const libc::c_char);
        if (ldap_conf.host).is_null() {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            debug_return_bool!(false);
        }
    }
    dprintf1!(b"LDAP Config Summary\0" as *const u8 as *const libc::c_char,);
    dprintf1!(b"==================\0" as *const u8 as *const libc::c_char,);
    if !(ldap_conf.uri.stqh_first).is_null() {
        conf_str = ldap_conf.uri.stqh_first;
        while !conf_str.is_null() {
            dprintf1!(
                b"uri             %s\0" as *const u8 as *const libc::c_char,
                (*conf_str).val
            );
            conf_str = (*conf_str).entries.stqe_next;
        }
    } else {
        dprintf1!(
            b"host             %s\0" as *const u8 as *const libc::c_char,
            if !(ldap_conf.host).is_null() {
                ldap_conf.host
            } else {
                b"(NONE)\0" as *const u8 as *const libc::c_char
            }
        );
        dprintf1!(
            b"port             %d\0" as *const u8 as *const libc::c_char,
            ldap_conf.port
        );
    }
    dprintf1!(
        b"ldap_version     %d\0" as *const u8 as *const libc::c_char,
        ldap_conf.version
    );
    if !(ldap_conf.base.stqh_first).is_null() {
        conf_str = ldap_conf.base.stqh_first;
        while !conf_str.is_null() {
            dprintf1!(
                b"sudoers_base     %s\0" as *const u8 as *const libc::c_char,
                (*conf_str).val
            );
            conf_str = (*conf_str).entries.stqe_next;
        }
    } else {
        dprintf1!(
            b"sudoers_base     %s\0" as *const u8 as *const libc::c_char,
            b"(NONE: LDAP disabled)\0" as *const u8 as *const libc::c_char
        );
    }
    if !(ldap_conf.search_filter).is_null() {
        dprintf1!(
            b"search_filter    %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.search_filter
        );
    }
    if !(ldap_conf.netgroup_base.stqh_first).is_null() {
        conf_str = ldap_conf.netgroup_base.stqh_first;
        while !conf_str.is_null() {
            dprintf1!(
                b"netgroup_base    %s\0" as *const u8 as *const libc::c_char,
                (*conf_str).val
            );
            conf_str = (*conf_str).entries.stqe_next;
        }
    } else {
        dprintf1!(
            b"netgroup_base %s\0" as *const u8 as *const libc::c_char,
            b"(NONE: will use nsswitch)\0" as *const u8 as *const libc::c_char
        );
    }
    if !(ldap_conf.netgroup_search_filter).is_null() {
        dprintf1!(
            b"netgroup_search_filter %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.netgroup_search_filter
        );
    }
    dprintf1!(
        b"binddn           %s\0" as *const u8 as *const libc::c_char,
        if !(ldap_conf.binddn).is_null() {
            ldap_conf.binddn
        } else {
            b"(anonymous)\0" as *const u8 as *const libc::c_char
        }
    );
    dprintf1!(
        b"bindpw           %s\0" as *const u8 as *const libc::c_char,
        if !(ldap_conf.bindpw).is_null() {
            ldap_conf.bindpw
        } else {
            b"(anonymous)\0" as *const u8 as *const libc::c_char
        }
    );
    if ldap_conf.bind_timelimit > 0 as libc::c_int {
        dprintf1!(
            b"bind_timelimit   %d\0" as *const u8 as *const libc::c_char,
            ldap_conf.bind_timelimit
        );
    }
    if ldap_conf.timelimit > 0 as libc::c_int {
        dprintf1!(
            b"timelimit        %d\0" as *const u8 as *const libc::c_char,
            ldap_conf.timelimit
        );
    }
    if ldap_conf.deref != -(1 as libc::c_int) {
        dprintf1!(
            b"deref            %d\0" as *const u8 as *const libc::c_char,
            ldap_conf.deref
        );
    }
    dprintf1!(
        b"ssl              %s\0" as *const u8 as *const libc::c_char,
        if !(ldap_conf.ssl).is_null() {
            ldap_conf.ssl
        } else {
            b"(no)\0" as *const u8 as *const libc::c_char
        }
    );
    if ldap_conf.tls_checkpeer != -(1 as libc::c_int) {
        dprintf1!(
            b"tls_checkpeer    %s\0" as *const u8 as *const libc::c_char,
            if ldap_conf.tls_checkpeer != 0 {
                b"(yes)\0" as *const u8 as *const libc::c_char
            } else {
                b"(no)\0" as *const u8 as *const libc::c_char
            }
        );
    }
    if ldap_conf.tls_reqcert != -(1 as libc::c_int) {
        dprintf1!(
            b"tls_reqcert   %s\0" as *const u8 as *const libc::c_char,
            if ldap_conf.tls_reqcert == LDAP_OPT_X_TLS_NEVER as libc::c_int {
                b"hard\0" as *const u8 as *const libc::c_char
            } else if ldap_conf.tls_reqcert == LDAP_OPT_X_TLS_ALLOW as libc::c_int {
                b"allow\0" as *const u8 as *const libc::c_char
            } else if ldap_conf.tls_reqcert == LDAP_OPT_X_TLS_TRY as libc::c_int {
                b"try\0" as *const u8 as *const libc::c_char
            } else if ldap_conf.tls_reqcert == LDAP_OPT_X_TLS_HARD as libc::c_int {
                b"hard\0" as *const u8 as *const libc::c_char
            } else if ldap_conf.tls_reqcert == LDAP_OPT_X_TLS_DEMAND as libc::c_int {
                b"demand\0" as *const u8 as *const libc::c_char
            } else {
                b"unknown\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !(ldap_conf.tls_cacertfile).is_null() {
        dprintf1!(
            b"tls_cacertfile   %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.tls_cacertfile
        );
    }
    if !(ldap_conf.tls_cacertdir).is_null() {
        dprintf1!(
            b"tls_cacertdir    %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.tls_cacertdir
        );
    }
    if !(ldap_conf.tls_random_file).is_null() {
        dprintf1!(
            b"tls_random_file  %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.tls_random_file
        );
    }
    if !(ldap_conf.tls_cipher_suite).is_null() {
        dprintf1!(
            b"tls_cipher_suite %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.tls_cipher_suite
        );
    }
    if !(ldap_conf.tls_certfile).is_null() {
        dprintf1!(
            b"tls_certfile     %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.tls_certfile
        );
    }
    if !(ldap_conf.tls_keyfile).is_null() {
        dprintf1!(
            b"tls_keyfile      %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.tls_keyfile
        );
    }
    if ldap_conf.use_sasl != -(1 as libc::c_int) {
        if (ldap_conf.sasl_mech).is_null() {
            ldap_conf.sasl_mech = strdup(b"GSSAPI\0" as *const u8 as *const libc::c_char);
            if (ldap_conf.sasl_mech).is_null() {
                sudo_warnx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
                debug_return_bool!(false);
            }
        }
        dprintf1!(
            b"use_sasl         %s\0" as *const u8 as *const libc::c_char,
            if ldap_conf.use_sasl != 0 {
                b"(yes)\0" as *const u8 as *const libc::c_char
            } else {
                b"(no)\0" as *const u8 as *const libc::c_char
            }
        );
        dprintf1!(
            b"sasl_mech        %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.sasl_mech
        );
        dprintf1!(
            b"sasl_auth_id     %s\0" as *const u8 as *const libc::c_char,
            if !(ldap_conf.sasl_auth_id).is_null() {
                ldap_conf.sasl_auth_id
            } else {
                b"(NONE)\0" as *const u8 as *const libc::c_char
            }
        );
        dprintf1!(
            b"rootuse_sasl     %s\0" as *const u8 as *const libc::c_char,
            ldap_conf.rootuse_sasl
        );
        dprintf1!(
            b"rootsasl_auth_id %s\0" as *const u8 as *const libc::c_char,
            if !(ldap_conf.rootsasl_auth_id).is_null() {
                ldap_conf.rootsasl_auth_id
            } else {
                b"(NONE)\0" as *const u8 as *const libc::c_char
            }
        );
        dprintf1!(
            b"sasl_secprops    %s\0" as *const u8 as *const libc::c_char,
            if !(ldap_conf.sasl_secprops).is_null() {
                ldap_conf.sasl_secprops
            } else {
                b"(NONE)\0" as *const u8 as *const libc::c_char
            }
        );
        dprintf1!(
            b"krb5_ccname      %s\0" as *const u8 as *const libc::c_char,
            if !(ldap_conf.krb5_ccname).is_null() {
                ldap_conf.krb5_ccname
            } else {
                b"(NONE)\0" as *const u8 as *const libc::c_char
            }
        );
    }
    dprintf1!(b"===================\0" as *const u8 as *const libc::c_char,);
    if (ldap_conf.base.stqh_first).is_null() {
        debug_return_bool!(false);
    }
    if ldap_conf.bind_timelimit > 0 as libc::c_int {
        ldap_conf.bind_timelimit *= 1000 as libc::c_int;
    }
    if !(ldap_conf.ssl).is_null() {
        if strcasecmp(
            ldap_conf.ssl,
            b"start_tls\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            ldap_conf.ssl_mode = SUDO_LDAP_STARTTLS as libc::c_int;
        } else if sudo_strtobool_v1(ldap_conf.ssl) == 1 as libc::c_int {
            ldap_conf.ssl_mode = SUDO_LDAP_SSL as libc::c_int;
        }
    }
    if (ldap_conf.uri.stqh_first).is_null() {
        if ldap_conf.port < 0 as libc::c_int {
            ldap_conf.port = if ldap_conf.ssl_mode == SUDO_LDAP_SSL as libc::c_int {
                LDAPS_PORT
            } else {
                LDAP_PORT
            };
        }
        if ldap_conf.port != LDAP_PORT as libc::c_int {
            if !sudo_ldap_conf_add_ports() {
                debug_return_bool!(false);
            }
        }
    }
    if !(ldap_conf.search_filter).is_null()
        && *(ldap_conf.search_filter).offset(0 as libc::c_int as isize) as libc::c_int != '(' as i32
    {
        let mut len: size_t = strlen(ldap_conf.search_filter);
        cp = ldap_conf.search_filter;
        ldap_conf.search_filter =
            malloc(len.wrapping_add(3 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
        if (ldap_conf.search_filter).is_null() {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            debug_return_bool!(false);
        }
        *(ldap_conf.search_filter).offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;
        memcpy(
            (ldap_conf.search_filter).offset(1 as libc::c_int as isize) as *mut libc::c_void,
            cp as *const libc::c_void,
            len,
        );
        *(ldap_conf.search_filter)
            .offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) =
            ')' as i32 as libc::c_char;
        *(ldap_conf.search_filter)
            .offset(len.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) =
            '\u{0}' as i32 as libc::c_char;
        free(cp as *mut libc::c_void);
    }
    if !(ldap_conf.rootbinddn).is_null() {
        sudo_ldap_read_secret(path_ldap_secret);
    } else if !(ldap_conf.bindpw).is_null() {
        cp = sudo_ldap_decode_secret(ldap_conf.bindpw);
        if !cp.is_null() {
            free(ldap_conf.bindpw as *mut libc::c_void);
            ldap_conf.bindpw = cp;
        }
    }
    if !(ldap_conf.tls_keypw).is_null() {
        cp = sudo_ldap_decode_secret(ldap_conf.tls_keypw);
        if !cp.is_null() {
            free(ldap_conf.tls_keypw as *mut libc::c_void);
            ldap_conf.tls_keypw = cp;
        }
    }
    if !(ldap_conf.krb5_ccname).is_null() {
        if !sudo_check_krb5_ccname(ldap_conf.krb5_ccname) {
            ldap_conf.krb5_ccname = 0 as *mut libc::c_char;
        }
    }
    debug_return_bool!(true);
}


