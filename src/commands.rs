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

use phf::phf_map;
use std::ffi::CString;

type Builtin = fn(&[&str]);

pub fn get_builtin(name: &str) -> Option<&Builtin> {
    return BUILTINS.get(name);
}

static BUILTINS: phf::Map<&'static str, Builtin> = phf_map! {
    "cd" => cd,
    "pwd" => pwd,
    "exit" => exit,
};

extern "C" {
    // Used for `cd` command
    fn chdir(path: *const u8) -> i32;
}

// TODO: Better error messages
fn cd(args: &[&str]) {
    unsafe {
        let cstr = CString::new(args[1]).unwrap();
        let res = chdir(cstr.as_ptr() as *const u8);
        if res != 0 {
            eprintln!("Seashell error: {}", res);
        }
    }
}

fn pwd(_: &[&str]) {
    println!("{:?}", std::env::current_dir().unwrap());
}

fn exit(args: &[&str]) {
    if let Some(code) = args.get(1) {
        // Exit with code
        let code = code.parse::<i32>();
        if let Ok(code) = code {
            std::process::exit(code);
        } else {
            eprintln!("Seashell error: bad exit code");
            std::process::exit(2);
        }
    } else {
        std::process::exit(0);
    }
}
