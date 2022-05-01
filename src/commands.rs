/*
* Commands to implement:
*
* alias
* bg
* bind
* break
* builtin
* caller
* cd
* command
* compgen
* complete
* compopt
* continue
+ coproc
* declare
* dirs
* declare
* echo ???
* enable
* eval
* exit
* export
* true / false
* fc
* fg
* for
* function
* getopts
* hash
* help
* history
* if
* jobs
* kill
* let
* local
* logout
* mapfile
* popd
* printf
* pushd
* pwd
* read
* readarray
* readonly
* return
* select
* set
* shift
* shopt
* source
* suspend
* test
* time
* times
* trap
* type
* typeset
* ulimit
* umask
* unalias
* unset
* until
* variables
* wait
* while
*/

use std::ffi::CString;

extern "C" {
    fn chdir(path: *const u8) -> i32;
}

pub fn cd(dir: &str) {
    unsafe {
        let cstr = CString::new(dir).unwrap();
        chdir(cstr.as_ptr() as *const u8);
    }
}

pub fn pwd() {
    println!("{:?}", std::env::current_dir().unwrap());
}
