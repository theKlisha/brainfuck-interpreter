#![allow(dead_code)]
#![allow(unused)]

use std::fmt::{Display, Pointer};
use std::io;
use std::io::stdin;
use std::{env, fs};

struct Interpreter {
    state: State,
    input: Box<dyn io::Read>,
    output: Box<dyn io::Write>,
}

impl Interpreter {
    pub fn tick(&mut self) -> Result<(), InterpteterError> {
        let ins = self
            .state
            .ins
            .get(self.state.ins_ptr)
            .ok_or(InterpteterError::EndOfProgram)?;

        self.exec()?;
        self.state.ins_ptr += 1;

        Ok(())
    }

    fn exec(&mut self) -> Result<(), InterpteterError> {
        let s = &mut self.state;
        match s.ins[s.ins_ptr] {
            b'>' => {
                s.data_ptr += 1;
                if s.data_ptr >= s.data.len() {
                    return Err(InterpteterError::DataPointerBounds);
                } else {
                    Ok(())
                }
            }
            b'<' => {
                s.data_ptr -= 1;
                if s.data_ptr >= s.data.len() {
                    return Err(InterpteterError::DataPointerBounds);
                } else {
                    Ok(())
                }
            }
            b'+' => {
                s.data[s.data_ptr] += 1;
                Ok(())
            }
            b'-' => {
                s.data[s.data_ptr] -= 1;
                Ok(())
            }
            b'.' => {
                self.output.write_all(&[s.data[s.data_ptr]]).unwrap();
                self.output.flush().unwrap();
                Ok(())
            }
            b',' => {
                let mut buf = [0; 1];
                self.input.read_exact(&mut buf).unwrap();
                s.data[s.data_ptr] = buf[0];
                Ok(())
            }
            b'[' => {
                if s.data[s.data_ptr] == 0 {
                    let mut count = 1;
                    while count != 0 {
                        s.ins_ptr += 1;
                        match s.ins[s.ins_ptr] {
                            b'[' => count += 1,
                            b']' => count -= 1,
                            _ => (),
                        }
                    }
                }
                Ok(())
            }
            b']' => {
                if s.data[s.data_ptr] != 0 {
                    let mut count = 1;
                    while count != 0 {
                        s.ins_ptr -= 1;
                        match s.ins[s.ins_ptr] {
                            b'[' => count -= 1,
                            b']' => count += 1,
                            _ => (),
                        }
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
struct State {
    data: Vec<u8>,
    data_ptr: usize,
    ins: Vec<u8>,
    ins_ptr: usize,
}

impl State {
    pub fn new() -> Self {
        Self::new_with_size(30_000)
    }

    pub fn new_with_size(size: usize) -> Self {
        let ins = Vec::new();
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);

        Self {
            data,
            data_ptr: 0,
            ins,
            ins_ptr: 0,
        }
    }

    pub fn push(&mut self, prog: &[u8]) {
        self.ins.extend_from_slice(prog)
    }
}

enum InterpteterError {
    EndOfProgram,
    DataPointerBounds,
}

impl Display for InterpteterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpteterError::EndOfProgram => write!(f, "end of program"),
            InterpteterError::DataPointerBounds => write!(f, "data pointer out of bounds"),
        }
    }
}

fn interpret_file(path: &str) {
    let mut i = Interpreter {
        state: State::new(),
        input: Box::new(io::stdin()),
        output: Box::new(io::stdout()),
    };

    let prog = fs::read_to_string(path).unwrap();
    i.state.push(prog.as_bytes());

    loop {
        match i.tick() {
            Ok(_) => (),
            Err(InterpteterError::EndOfProgram) => break,
            Err(e) => panic!("{}", e),
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 => todo!("implement interactive shell"),
        2 => interpret_file(&args[1]),
        _ => panic!("too many arguments"),
    }
}
