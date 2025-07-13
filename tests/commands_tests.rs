
use std::fs::{ self, File };
use std::io::{ Write, Read };
use std::env;
use std::path::Path;
use shell::parser::{ input_parser, Command };
use shell::builtins::{ cd::*, pwd::*, rm::*, mv::*, mkdir::*, cp::* };

fn setup_file(name: &str, content: &str) {
    let mut f = File::create(name).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

fn cleanup(name: &str) {
    let path = Path::new(name);
    if path.exists() {
        if path.is_file() {
            let _ = fs::remove_file(name);
        } else if path.is_dir() {
            let _ = fs::remove_dir_all(name);
        }
    }
}

#[test]
fn test_echo() {
    let home = env::var("HOME").unwrap();
    let cmd = input_parser("echo ~".to_string()).unwrap();
    if let Command::Echo(args) = cmd {
        assert_eq!(args[0], home);
    } else {
        panic!("Not echo");
    }
    let cmd = input_parser("echo \"~\"".to_string()).unwrap();
    if let Command::Echo(args) = cmd {
        assert_eq!(args[0], "~");
    } else {
        panic!("Not echo")
    }
}

#[test]
fn test_cd_and_pwd() {
    let orig = env::current_dir().unwrap();
    let tmp = "test_cd_dir";
    fs::create_dir(tmp).unwrap();
    cd(vec![tmp.to_string()]);
    let cur = env::current_dir().unwrap();
    assert!(cur.ends_with(tmp));
    cd(vec![orig.to_str().unwrap().to_string()]);
    let cur2 = env::current_dir().unwrap();
    assert_eq!(cur2, orig);
    fs::remove_dir(tmp).unwrap();
}

// #[test]
// fn test_pwd() {
//     let dir = env::current_dir().unwrap();
//     let out = pwd();
//     assert_eq!(out, dir.to_str().unwrap());
// }

#[test]
fn test_rm() {
    let fname = "test_rm_cmd.txt";
    setup_file(fname, "abc");
    assert!(Path::new(fname).exists());
    rm(vec![fname.to_string()], false);
    assert!(!Path::new(fname).exists());
}

#[test]
fn test_mv() {
    let src = "test_mv_src.txt";
    let dst = "test_mv_dst.txt";
    setup_file(src, "mv content");
    mv(vec![src.to_string(), dst.to_string()]);
    assert!(!Path::new(src).exists());
    assert!(Path::new(dst).exists());
    cleanup(dst);
}

#[test]
fn test_mkdir() {
    let dname = "test_mkdir_dir";
    mkdir(vec![dname.to_string()]);
    assert!(Path::new(dname).exists());
    cleanup(dname);
}

#[test]
fn test_cp() {
    let src = "test_cp_src.txt";
    let dst = "test_cp_dst.txt";
    setup_file(src, "cp content");
    cp(vec![src.to_string(), dst.to_string()]);
    assert!(Path::new(src).exists());
    assert!(Path::new(dst).exists());
    let mut s = String::new();
    File::open(dst).unwrap().read_to_string(&mut s).unwrap();
    assert!(s.contains("cp content"));
    cleanup(src);
    cleanup(dst);
}

#[test]
fn test_exit() {

    let cmd = input_parser("exit 0".to_string()).unwrap();
    if let Command::Exit = cmd {
    } else {
        panic!("Not exit");
    }
    let err = input_parser("exit foo".to_string());
    assert!(err.is_err());
}

#[test]
fn test_echo_hard() {
    let cmd = input_parser("echo foo bar baz".to_string()).unwrap();
    if let Command::Echo(args) = cmd {
        assert_eq!(args, vec!["foo", "bar", "baz"]);
    }
   
    let cmd = input_parser("echo 'a b' \"c d\" e\\ f".to_string()).unwrap();
    if let Command::Echo(args) = cmd {
        assert_eq!(args, vec!["a b", "c d", "e f"]);
    }
  
    let cmd = input_parser("echo".to_string()).unwrap();
    if let Command::Echo(args) = cmd {
        assert!(args.is_empty());
    }
}

#[test]
fn test_cd_hard() {
    let orig = env::current_dir().unwrap();
  
    let bad = "no_such_dir";
    let res = std::panic::catch_unwind(|| cd(vec![bad.to_string()]));
    assert!(res.is_ok());
   
    let home = env::var("HOME").unwrap();
    cd(vec![home.clone()]);
    let cur = env::current_dir().unwrap();
    assert_eq!(cur, Path::new(&home));

    cd(vec!["..".to_string()]);
    let cur2 = env::current_dir().unwrap();
    assert_eq!(cur2, Path::new(&home).parent().unwrap());
    cd(vec![orig.to_str().unwrap().to_string()]);
}

#[test]
fn test_pwd_hard() {
    let orig = env::current_dir().unwrap();
    let tmp = "test_pwd_dir";
    fs::create_dir(tmp).unwrap();
    cd(vec![tmp.to_string()]);
    let out = pwd();
    assert!(out.ends_with(tmp));
    cd(vec![orig.to_str().unwrap().to_string()]);
    fs::remove_dir(tmp).unwrap();
}

#[test]
fn test_rm_hard() {
    let dname = "test_rm_hard_dir";
    fs::create_dir(dname).unwrap();
    rm(vec![dname.to_string()], false);
    assert!(Path::new(dname).exists());
    rm(vec![dname.to_string()], true);
    assert!(!Path::new(dname).exists());
    let fname = "test rm hard.txt";
    setup_file(fname, "abc");
    rm(vec![fname.to_string()], false);
    assert!(!Path::new(fname).exists());
}

#[test]
fn test_mv_hard() {
    let src = "test_mv_hard_src.txt";
    let dst = "test_mv_hard_dst.txt";
    setup_file(src, "src");
    setup_file(dst, "dst");
    mv(vec![src.to_string(), dst.to_string()]);
    assert!(!Path::new(src).exists());
    let mut s = String::new();
    File::open(dst).unwrap().read_to_string(&mut s).unwrap();
    assert_eq!(s, "src");
    cleanup(dst);
    let src2 = "test_mv_hard2.txt";
    setup_file(src2, "x");
    let bad_dst = "no_such_dir/test_mv_hard2.txt";
    let res = std::panic::catch_unwind(|| mv(vec![src2.to_string(), bad_dst.to_string()]));
    assert!(res.is_ok());
    cleanup(src2);
}

#[test]
fn test_mkdir_hard() {
    let dname = "test_mkdir_hard";
    fs::create_dir(dname).unwrap();
    mkdir(vec![dname.to_string()]);
    assert!(Path::new(dname).exists());
    cleanup(dname);
    let nested = "test_mkdir_hard/nested";
    let res = std::panic::catch_unwind(|| mkdir(vec![nested.to_string()]));
    assert!(res.is_ok());
    cleanup("test_mkdir_hard");
}

#[test]
fn test_cp_hard() {
    let src = "test_cp_hard_src.txt";
    let dst = "test_cp_hard_dst.txt";
    setup_file(src, "src");
    setup_file(dst, "dst");
    cp(vec![src.to_string(), dst.to_string()]);
    let mut s = String::new();
    File::open(dst).unwrap().read_to_string(&mut s).unwrap();
    assert_eq!(s, "src");
    cleanup(src);
    cleanup(dst);
    let src2 = "test_cp_hard2.txt";
    setup_file(src2, "x");
    let bad_dst = "no_such_dir/test_cp_hard2.txt";
    let res = std::panic::catch_unwind(|| cp(vec![src2.to_string(), bad_dst.to_string()]));
    assert!(res.is_ok());
    cleanup(src2);
}

#[test]
fn test_exit_hard() {
  
    let cmd = input_parser("exit".to_string()).unwrap();
    if let Command::Exit = cmd {
    } else {
        panic!("Not exit");
    }
    let cmd = input_parser("exit -1".to_string()).unwrap();
    if let Command::Exit = cmd {
    } else {
        panic!("Not exit");
    }
    let cmd = input_parser("exit 99999".to_string()).unwrap();
    if let Command::Exit = cmd {
    } else {
        panic!("Not exit")
    }
}
