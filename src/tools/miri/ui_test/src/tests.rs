use std::path::{Path, PathBuf};

use crate::rustc_stderr::Level;
use crate::rustc_stderr::Message;

use super::*;

fn config() -> Config {
    Config {
        args: vec![],
        target: None,
        stderr_filters: vec![],
        stdout_filters: vec![],
        root_dir: PathBuf::from("$RUSTROOT"),
        mode: Mode::Fail,
        path_filter: vec![],
        program: PathBuf::from("cake"),
        output_conflict_handling: OutputConflictHandling::Error,
    }
}

#[test]
fn issue_2156() {
    let s = r"
use std::mem;

fn main() {
    let _x: &i32 = unsafe { mem::transmute(16usize) }; //~ ERROR: encountered a dangling reference (address $HEX is unallocated)
}
    ";
    let path = Path::new("$DIR/<dummy>");
    let comments = Comments::parse(path, s).unwrap();
    let mut errors = vec![];
    let config = config();
    let messages = vec![
        vec![], vec![], vec![], vec![], vec![],
        vec![
            Message {
                message:"Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                level: Level::Error,
            }
        ]
    ];
    check_annotations(messages, vec![], Path::new("moobar"), &mut errors, &config, "", &comments);
    match &errors[..] {
        [
            Error::PatternNotFound { definition_line: 5, .. },
            Error::ErrorsWithoutPattern { path: Some((_, 5)), .. },
        ] => {}
        _ => panic!("{:#?}", errors),
    }
}

#[test]
fn find_pattern() {
    let s = r"
use std::mem;

fn main() {
    let _x: &i32 = unsafe { mem::transmute(16usize) }; //~ ERROR: encountered a dangling reference (address 0x10 is unallocated)
}
    ";
    let comments = Comments::parse(Path::new("<dummy>"), s).unwrap();
    let config = config();
    {
        let messages = vec![vec![], vec![], vec![], vec![], vec![], vec![
                Message {
                    message: "Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                    level: Level::Error,
                }
            ]
        ];
        let mut errors = vec![];
        check_annotations(
            messages,
            vec![],
            Path::new("moobar"),
            &mut errors,
            &config,
            "",
            &comments,
        );
        match &errors[..] {
            [] => {}
            _ => panic!("{:#?}", errors),
        }
    }

    // only difference to above is a wrong line number
    {
        let messages = vec![vec![], vec![], vec![], vec![], vec![
                Message {
                    message: "Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                    level: Level::Error,
                }
            ]
        ];
        let mut errors = vec![];
        check_annotations(
            messages,
            vec![],
            Path::new("moobar"),
            &mut errors,
            &config,
            "",
            &comments,
        );
        match &errors[..] {
            [
                Error::PatternNotFound { definition_line: 5, .. },
                Error::ErrorsWithoutPattern { path: Some((_, 4)), .. },
            ] => {}
            _ => panic!("not the expected error: {:#?}", errors),
        }
    }

    // only difference to first is a wrong level
    {
        let messages = vec![
            vec![], vec![], vec![], vec![], vec![],
            vec![
                Message {
                    message: "Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                    level: Level::Note,
                }
            ]
        ];
        let mut errors = vec![];
        check_annotations(
            messages,
            vec![],
            Path::new("moobar"),
            &mut errors,
            &config,
            "",
            &comments,
        );
        match &errors[..] {
            // Note no `ErrorsWithoutPattern`, because there are no `//~NOTE` in the test file, so we ignore them
            [Error::PatternNotFound { definition_line: 5, .. }] => {}
            _ => panic!("not the expected error: {:#?}", errors),
        }
    }
}

#[test]
fn duplicate_pattern() {
    let s = r"
use std::mem;

fn main() {
    let _x: &i32 = unsafe { mem::transmute(16usize) }; //~ ERROR: encountered a dangling reference (address 0x10 is unallocated)
    //~^ ERROR: encountered a dangling reference (address 0x10 is unallocated)
}
    ";
    let comments = Comments::parse(Path::new("<dummy>"), s).unwrap();
    let config = config();
    let messages = vec![
        vec![], vec![], vec![], vec![], vec![],
        vec![
            Message {
                message: "Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                level: Level::Error,
            }
        ]
    ];
    let mut errors = vec![];
    check_annotations(messages, vec![], Path::new("moobar"), &mut errors, &config, "", &comments);
    match &errors[..] {
        [Error::PatternNotFound { definition_line: 6, .. }] => {}
        _ => panic!("{:#?}", errors),
    }
}

#[test]
fn missing_pattern() {
    let s = r"
use std::mem;

fn main() {
    let _x: &i32 = unsafe { mem::transmute(16usize) }; //~ ERROR: encountered a dangling reference (address 0x10 is unallocated)
}
    ";
    let comments = Comments::parse(Path::new("<dummy>"), s).unwrap();
    let config = config();
    let messages = vec![
        vec![], vec![], vec![], vec![], vec![],
        vec![
            Message {
                message: "Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                level: Level::Error,
            },
            Message {
                message: "Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                level: Level::Error,
            }
        ]
    ];
    let mut errors = vec![];
    check_annotations(messages, vec![], Path::new("moobar"), &mut errors, &config, "", &comments);
    match &errors[..] {
        [Error::ErrorsWithoutPattern { path: Some((_, 5)), .. }] => {}
        _ => panic!("{:#?}", errors),
    }
}

#[test]
fn missing_warn_pattern() {
    let s = r"
use std::mem;

fn main() {
    let _x: &i32 = unsafe { mem::transmute(16usize) }; //~ ERROR: encountered a dangling reference (address 0x10 is unallocated)
    //~^ WARN: cake
}
    ";
    let comments = Comments::parse(Path::new("<dummy>"), s).unwrap();
    let config = config();
    let messages= vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![
            Message {
                message: "Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                level: Level::Error,
            },
            Message {
                message: "kaboom".to_string(),
                level: Level::Warn,
            },
            Message {
                message: "cake".to_string(),
                level: Level::Warn,
            },
        ],
    ];
    let mut errors = vec![];
    check_annotations(messages, vec![], Path::new("moobar"), &mut errors, &config, "", &comments);
    match &errors[..] {
        [Error::ErrorsWithoutPattern { path: Some((_, 5)), msgs, .. }] =>
            match &msgs[..] {
                [Message { message, level: Level::Warn }] if message == "kaboom" => {}
                _ => panic!("{:#?}", msgs),
            },
        _ => panic!("{:#?}", errors),
    }
}

#[test]
fn missing_implicit_warn_pattern() {
    let s = r"
use std::mem;
//@require-annotations-for-level: ERROR
fn main() {
    let _x: &i32 = unsafe { mem::transmute(16usize) }; //~ ERROR: encountered a dangling reference (address 0x10 is unallocated)
    //~^ WARN: cake
}
    ";
    let comments = Comments::parse(Path::new("<dummy>"), s).unwrap();
    let config = config();
    let messages = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![
            Message {
                message: "Undefined Behavior: type validation failed: encountered a dangling reference (address 0x10 is unallocated)".to_string(),
                level: Level::Error,
            },
            Message {
                message: "kaboom".to_string(),
                level: Level::Warn,
            },
            Message {
                message: "cake".to_string(),
                level: Level::Warn,
            },
        ],
    ];
    let mut errors = vec![];
    check_annotations(messages, vec![], Path::new("moobar"), &mut errors, &config, "", &comments);
    match &errors[..] {
        [] => {}
        _ => panic!("{:#?}", errors),
    }
}
