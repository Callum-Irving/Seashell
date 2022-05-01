/*
* Commands to implement:
*
* - [ ] alias
* - [ ] bg
* - [ ] bind
* - [ ] break
* - [ ] builtin
* - [ ] caller
* - [x] cd (needs more work)
* - [ ] command
* - [ ] compgen
* - [ ] complete
* - [ ] compopt
* - [ ] continue
+ - [ ] coproc
* - [ ] declare
* - [ ] dirs
* - [ ] declare
* - [x] echo ???
* - [ ] enable
* - [ ] eval
* - [x] exit
* - [x] export
* - [ ] true / false
* - [ ] fc
* - [ ] fg
* - [ ] for
* - [ ] function
* - [ ] getopts
* - [ ] hash
* - [ ] help
* - [ ] history
* - [ ] if
* - [ ] jobs
* - [ ] kill
* - [ ] let
* - [ ] local
* - [ ] logout
* - [ ] mapfile
* - [ ] popd
* - [ ] printf
* - [ ] pushd
* - [x] pwd
* - [ ] read
* - [ ] readarray
* - [ ] readonly
* - [ ] return
* - [ ] select
* - [ ] set
* - [ ] shift
* - [ ] shopt
* - [ ] source
* - [ ] suspend
* - [ ] test
* - [ ] time
* - [ ] times
* - [ ] trap
* - [ ] type
* - [ ] typeset
* - [ ] ulimit
* - [ ] umask
* - [ ] unalias
* - [ ] unset
* - [ ] until
* - [ ] variables
* - [ ] wait
* - [ ] while
*/

use crate::Context;

use phf::phf_map;
use std::ffi::CString;

// type Builtin = fn(&[&str]);
type Builtin = fn(&[String], &mut Context) -> i32;

pub(crate) fn get_builtin(name: &str) -> Option<&Builtin> {
    return BUILTINS.get(name);
}

static BUILTINS: phf::Map<&'static str, Builtin> = phf_map! {
    "cd" => cd,
    "pwd" => pwd,
    "exit" => exit,
    "export" => export,
    "alias" => alias
};

extern "C" {
    // Used for `cd` command
    fn chdir(path: *const u8) -> i32;
}

// TODO: Better error messages
fn cd(args: &[String], _: &mut Context) -> i32 {
    unsafe {
        let cstr = CString::new(args[1].as_bytes()).unwrap();
        let res = chdir(cstr.as_ptr() as *const u8);
        if res != 0 {
            eprintln!("Seashell error: {}", res);
        }

        res
    }
}

fn pwd(_: &[String], _: &mut Context) -> i32 {
    println!("{:?}", std::env::current_dir().unwrap());
    0
}

fn exit(args: &[String], _: &mut Context) -> i32 {
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

fn export(args: &[String], _: &mut Context) -> i32 {
    use std::env;
    // args 1 = variable name
    // args 2 = '='
    // args 3 = variable value
    env::set_var(&args[1], &args[3]);
    0
}

fn alias(args: &[String], ctx: &mut Context) -> i32 {
    ctx.aliases.insert(args[1].clone(), args[3].clone());
    0
}
