#[macro_use]
extern crate log;
extern crate simple_logger;


extern crate config;

use std::path::{Path, PathBuf};
use std::env;
use config::*;



use std::io;
use std::process::{Command, ExitStatus, exit, Stdio};
use std::ffi::OsStr;


pub fn get_config(key: &str) -> String {
    info!("read the config file");
    let mut settings = Config::default();


    let mut homeconfig = PathBuf::new();
    homeconfig.push(env::home_dir().unwrap());
    homeconfig.push(r".uopen_config.json");


    let pp = homeconfig.to_str().unwrap();
    info!("read home config file! {:?}", pp);


    settings.merge(File::with_name(pp)).unwrap();

    settings.get_str(key).unwrap()
}


pub fn open(opt: &str)  {
    info!("the opt:{:?}", opt);

    if opt.starts_with("http") || opt.starts_with("HTTP")  {
        that(opt);
        exit(1);
    }
    // local file
    let mut abspath = PathBuf::new();

    if opt.starts_with("/") {
        abspath.push(opt);
    } else {
        abspath.push(env::current_dir().unwrap());
        abspath.push(opt);
    }

    info!("the absolute path {:?} exists: {}", abspath, abspath.exists());
    if abspath.exists() {
        filemanager(abspath);
    }else {
        error!("absolute path not found: {:?}", abspath);
    }
}


#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn filemanager<T:AsRef<OsStr>+Sized>(path: T) -> io::Result<ExitStatus> {
    let mut last_err: io::Result<ExitStatus> = Err(io::Error::from_raw_os_error(0));

    match Command::new(get_config("default-file-manager")).arg(path.as_ref()).stdout(Stdio::null()).spawn() {
        Ok(mut child) => {
            exit(1);
            return child.wait()
        },
        Err(err) => {
            last_err = Err(err);
        },
    }
    last_err
}



#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn that<T:AsRef<OsStr>+Sized>(path: T) -> io::Result<ExitStatus> {
    let mut last_err: io::Result<ExitStatus> = Err(io::Error::from_raw_os_error(0));
    for program in &["xdg-open", "gnome-open", "kde-open"] {
        match Command::new(program).arg(path.as_ref()).stdout(Stdio::null()).spawn() {
            Ok(mut child) =>  {
                // exit(1);
                return child.wait()
            },
            Err(err) => {
                last_err = Err(err);
                continue;
            },
        }
    }
    last_err
}

#[cfg(target_os = "windows")]
pub fn that<T:AsRef<OsStr>+Sized>(path: T) -> io::Result<ExitStatus> {
    try!(Command::new("cmd").arg("/C").arg("start").arg(path.as_ref()).spawn()).wait()
}

#[cfg(target_os = "macos")]
pub fn that<T:AsRef<OsStr>+Sized>(path: T) -> io::Result<ExitStatus> {
    try!(Command::new("open").arg(path.as_ref()).spawn()).wait()
}




#[test]
fn test_get_config() {
    let fm = get_config("default-file-manager");
    assert_eq!(fm, "nautilus");
}
