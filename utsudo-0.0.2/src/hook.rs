#[no_mangle]
pub unsafe extern "C" fn deregister_hook(mut hook: *mut sudo_hook) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    //define debug_decl(deregister_hook,SUDO_DEBUG_HOOKS);
    debug_decl!(deregister_hook, SUDO_DEBUG_HOOKS);
    //end of define;

    if (*hook).hook_version >> 16 != 1 {
        ret = -1;
    } else {
        match (*hook).hook_type {
            4 => {
                deregister_hook_internal(
                    &mut sudo_hook_getenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            3 => {
                deregister_hook_internal(
                    &mut sudo_hook_putenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            1 => {
                deregister_hook_internal(
                    &mut sudo_hook_setenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            2 => {
                deregister_hook_internal(
                    &mut sudo_hook_unsetenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            _ => {
                ret = 1;
            }
        }
    }
    //define debug_return_int(ret);
    debug_return_int!(ret);
    //end of define
}