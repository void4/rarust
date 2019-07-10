#![allow(while_true)]
#![feature(nll)]

extern crate num_derive;
extern crate num_traits;
use num_traits::FromPrimitive;

#[derive(Debug, num_derive::FromPrimitive)]
enum Stati {
    NOR, //Normal
    HLT, //Halt
    RET, //Return
    YLD, //Yield
    OOC, //OutOfCode
    OOA, //OutOfArguments
    OOS, //OutOfStack
    OOM, //OutOfMemory
    UOC, //UnknownCode
}

#[derive(Debug, PartialEq, num_derive::FromPrimitive)]
enum IS {
    HALT,
    RETURN,
    YIELD,
    RUN,
    JUMP,
    JZ,
    PUSH,
    POP,
    DUP,
    FLIP,
    KEYSET,
    KEYHAS,
    KEYGET,
    KEYDEL,
    STACKLEN,
    MEMORYLEN,
    AREALEN,
    READ,
    WRITE,
    AREA,
    DEAREA,
    ALLOC,
    DEALLOC,
    ADD,
    SUB,
    NOT,
    MUL,
    DIV,
    MOD,
    SHA256,
    ECVERIFY,
    ROT,
    ROT2,
}

#[derive(Clone)]
struct Requirement {
    length: i32,
    stack_req: i32,
    addtl_mem: i32,
    gas_cost: i32,
}

//Instruction length, stack reqs, additional memory, gas cost
fn requirement(is: IS) -> Requirement {
    match is {
        IS::HALT => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 0,
            gas_cost: 1,
        },
        IS::RETURN => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 0,
            gas_cost: 1,
        },
        IS::YIELD => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 0,
            gas_cost: 1,
        },
        IS::RUN => Requirement {
            length: 1,
            stack_req: 3,
            addtl_mem: -3,
            gas_cost: 0,
        },
        IS::JUMP => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: -1,
            gas_cost: 1,
        },
        IS::JZ => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -2,
            gas_cost: 1,
        },
        IS::PUSH => Requirement {
            length: 2,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 2,
        },
        IS::POP => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: -1,
            gas_cost: 2,
        }, //XXX changed stack effect
        IS::DUP => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 4,
        },
        IS::FLIP => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: 0,
            gas_cost: 4,
        },
        IS::KEYSET => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -2,
            gas_cost: 10,
        }, //keys
        IS::KEYHAS => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 4,
        },
        IS::KEYGET => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 6,
        },
        IS::KEYDEL => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: -1,
            gas_cost: 4,
        },
        IS::STACKLEN => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 2,
        }, //lens
        IS::MEMORYLEN => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 2,
        },
        IS::AREALEN => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 2,
        },
        IS::READ => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 2,
        }, //r/w
        IS::WRITE => Requirement {
            length: 1,
            stack_req: 3,
            addtl_mem: -3,
            gas_cost: 2,
        },
        IS::AREA => Requirement {
            length: 1,
            stack_req: 0,
            addtl_mem: 1,
            gas_cost: 10,
        }, //a/d
        IS::DEAREA => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: -1,
            gas_cost: 10,
        }, //use after free!
        IS::ALLOC => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -2,
            gas_cost: 10,
        }, //alloc/dealloc
        IS::DEALLOC => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -2,
            gas_cost: 10,
        },
        IS::ADD => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 6,
        },
        IS::SUB => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 6,
        },
        IS::NOT => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 4,
        },
        IS::MUL => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 8,
        },
        IS::DIV => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 10,
        },
        IS::MOD => Requirement {
            length: 1,
            stack_req: 2,
            addtl_mem: -1,
            gas_cost: 10,
        },
        IS::SHA256 => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 100,
        },
        IS::ECVERIFY => Requirement {
            length: 1,
            stack_req: 1,
            addtl_mem: 0,
            gas_cost: 100,
        },
        IS::ROT => Requirement {
            length: 1,
            stack_req: 3,
            addtl_mem: 0,
            gas_cost: 10,
        },
        IS::ROT2 => Requirement {
            length: 1,
            stack_req: 3,
            addtl_mem: 0,
            gas_cost: 10,
        },
    }
}

#[derive(Debug, Clone)]
struct Header {
    status: u64,
    rec: u64,
    gas: u64,
    mem: u64,
    ip: u64,
}

#[derive(Debug, Clone)]
struct Process {
    header: Header,
    code: Vec<u64>,
    stack: Vec<u64>,
    map: Vec<u64>,
    memory: Vec<Vec<u64>>,
}

/**
Deserialize the standard process snapshot format to the internal representation
*/
fn d(flat: &Vec<u64>) -> Process {
    let header: Header = unsafe { std::ptr::read(flat.as_ptr() as *const _) };

    const CODELEN: usize = 5;
    let codelen: usize = flat[CODELEN] as usize;
    let stacklen: usize = flat[CODELEN + 1] as usize;
    let maplen: usize = flat[CODELEN + 2] as usize;
    let memorylen: usize = flat[CODELEN + 3] as usize;

    let mut start: usize = CODELEN + 4;
    let mut end: usize = start + codelen;
    let code: Vec<u64> = flat[start..end].to_vec();

    start = end;
    end = start + stacklen;
    let mut stack: Vec<u64> = flat[start..end].to_vec();
    stack.reserve(1024);

    start = end;
    end = start + maplen;
    let mut map: Vec<u64> = flat[start..end].to_vec();
    map.reserve(1024);

    end = end;
    let mut memory: Vec<Vec<u64>> = Vec::new();
    for _area in 0..memorylen {
        let arealen: usize = flat[end] as usize;
        start = end + 1;
        end = start + arealen;
        let mut areavec = flat[start..end].to_vec();
        areavec.reserve(4096);
        memory.push(areavec);
    }

    return Process {
        header: header,
        code: code,
        stack: stack,
        map: map,
        memory: memory,
    };
}

/**
Serialize the internal representation to the standard process snapshot format
*/
fn s(sharp: &Process) -> Vec<u64> {
    let mut flat: Vec<u64> = Vec::new();
    flat.push(sharp.header.status);
    flat.push(sharp.header.rec);
    flat.push(sharp.header.gas);
    flat.push(sharp.header.mem);
    flat.push(sharp.header.ip);

    flat.push(sharp.code.len() as u64);
    flat.push(sharp.stack.len() as u64);
    flat.push(sharp.map.len() as u64);
    flat.push(sharp.memory.len() as u64);

    flat.extend(&sharp.code);
    flat.extend(&sharp.stack);
    flat.extend(&sharp.map);

    for area in sharp.memory.iter() {
        flat.push(area.len() as u64);
        flat.extend(area.clone());
    }
    return flat;
}

/**
Run a snapshot until an exit or error occurs
*/
fn run(sharp: Process, gas: u64, mem: u64, debug: bool) -> Process {
    //println!("Length of binary: {0}", flat.len());
    // Process, previously serialized length, rec index
    let mut edges: Vec<u64> = vec![0];
    let mut sizes: Vec<u64> = vec![s(&sharp).len() as u64];
    let mut states: Vec<(Process)> = vec![sharp]; //d(&flat)

    let statelen = states.len() - 1 as usize;
    states[statelen].header.status = Stati::NOR as u64;
    states[statelen].header.gas = gas;
    states[statelen].header.mem = mem;

    loop {
        let mut jump_back: i64 = -2;
        let blockret = {
            let instr: u64 = 0;
            let ref mut state = states[statelen];
            //println!("{:?} {:?}", state.header.gas, state.header.ip);
            if debug {
                println!("{:?}", state.stack);
            }

            if state.header.status != Stati::NOR as u64 {
                //&& state.header.status != REC
                jump_back = (statelen as i64) - 1;
                (
                    instr.clone(),
                    Requirement {
                        length: 0,
                        stack_req: 0,
                        addtl_mem: 0,
                        gas_cost: 0,
                    },
                )
            } else if state.header.ip >= state.code.len() as u64 {
                state.header.status = Stati::OOC as u64;
                jump_back = (statelen as i64) - 1;
                (
                    instr.clone(),
                    Requirement {
                        length: 0,
                        stack_req: 0,
                        addtl_mem: 0,
                        gas_cost: 0,
                    },
                )
            } else {
                let instr = state.code[state.header.ip as usize];
                let decoded: Option<IS> = IS::from_u64(instr);
                match decoded {
                    None => {
                        state.header.status = Stati::UOC as u64;
                        jump_back = (statelen as i64) - 1;
                        (
                            instr.clone(),
                            Requirement {
                                length: 0,
                                stack_req: 0,
                                addtl_mem: 0,
                                gas_cost: 0,
                            },
                        )
                    }
                    Some(i) => {
                        let reqs = requirement(i);
                        if state.header.ip + (reqs.length as u64) - 1 >= state.code.len() as u64 {
                            state.header.status = Stati::OOA as u64;
                            jump_back = (statelen as i64) - 1;
                        }

                        if reqs.stack_req as u64 > state.stack.len() as u64 {
                            state.header.status = Stati::OOS as u64;
                            jump_back = (statelen as i64) - 1;
                        }
                        (instr, reqs.clone())
                    }
                }
            }
        };

        let instr: u64 = blockret.0;
        let reqs = blockret.1;

        fn valid_area(index: u64, process: &Process) -> bool {
            return index < process.memory.len() as u64;
        }

        if jump_back == -2 {
            for psi in (0..states.len()).rev() {
                let gascost = reqs.gas_cost as u64;
                let memcost: u64 = gascost; //XXX(ps.1 + (reqs[2] as u64))*

                if states[psi].header.mem < memcost {
                    states[psi].header.status = Stati::OOM as u64;
                    jump_back = (psi as i64) - 1;
                }

                //p.header.mem -= memcost;
                //p.header.gas -= gascost;
            }
        }

        if jump_back > -2 {
            let mut serialized: Vec<u64> = Vec::new();
            for psi in (0..states.len()).rev() {
                if psi != 0 {
                    states[statelen - psi].memory[edges[edges.len() - psi] as usize] = serialized;
                    serialized = s(&states[states.len() - psi - 1]);
                }

                if psi == 0 {
                    return states[0].clone();
                }

                if psi as i64 == jump_back - 1 {
                    break;
                }
            }
        }

        let ie: IS = unsafe { std::mem::transmute(instr as i8) };
        let ref mut state = states[statelen];
        let stacklen: usize = state.stack.len() as usize;
        if debug {
            println!(
                "INSTR: {:?} {:?}",
                IS::from_u8(instr as u8),
                state.header.ip
            );
        }
        if jump_back > -2 {
            //pass
        } else if ie == IS::RUN {
            let area = state.stack[stacklen - 3];
            let gas = state.stack[stacklen - 2];
            let mem = state.stack[stacklen - 1];

            if valid_area(area, &states[statelen]) {
                {
                    if states[statelen].header.rec == 0 {
                        let ref mut child = states[statelen];
                        child.header.rec = area + 1;

                        child.memory[area as usize][0] = Stati::NOR as u64;
                        child.memory[area as usize][2] = gas;
                        child.memory[area as usize][3] = mem;
                    }
                }
                if states[statelen].header.rec > 0
                    && states[statelen].memory[area as usize][0] == Stati::NOR as u64
                {
                    states.push(d(&states[statelen].memory[area as usize]));
                    edges.push(states[statelen].header.rec);
                    sizes.push(states[statelen].memory[area as usize].len() as u64);
                } else {
                    states[statelen].header.rec = 0;
                }
            }
        } else {
            let ref mut state = states[statelen];
            let mut jump: bool = false;
            if ie == IS::HALT {
                println!("HALT");
                state.header.status = Stati::HLT as u64;
            } else if ie == IS::RETURN {
                state.header.status = Stati::RET as u64;
                state.header.ip = 0;
                jump = true;
            } else if ie == IS::YIELD {
                state.header.status = Stati::YLD as u64;
            } else if ie == IS::JUMP {
                state.header.ip = state.stack[state.stack.len() - 1];
                state.stack.pop();
                jump = true;
            } else if ie == IS::JZ {
                if state.stack.len() >= 2 {
                    if state.stack[state.stack.len() - 2] == 0 {
                        state.header.ip = state.stack[state.stack.len() - 1];
                        jump = true;
                    }
                    state.stack.pop();
                    state.stack.pop();
                }
            } else if ie == IS::DUP {
                state.stack.push(state.stack[state.stack.len() - 1]);
            //memory effect has to be applied to all parent states!
            //check resources after decision?
            } else if ie == IS::PUSH {
                let value = state.code[(state.header.ip + 1) as usize];
                state.stack.push(value);
            } else if ie == IS::POP {
                if state.stack.len() > 0 {
                    state.stack.pop();
                }
            } else if ie == IS::DUP {
                let last = state.stack.last();
                match last {
                    Some(last) => {
                        let truelast = last.clone();
                        state.stack.push(truelast);
                    }
                    None => {}
                }
            } else if ie == IS::ADD {
                if state.stack.len() >= 2 {
                    let b = state.stack.pop().unwrap();
                    let a = state.stack.pop().unwrap();
                    state.stack.push(a.wrapping_add(b));
                }
            } else if ie == IS::SUB {
                if state.stack.len() >= 2 {
                    let b = state.stack.pop().unwrap();
                    let a = state.stack.pop().unwrap();
                    state.stack.push(a.wrapping_sub(b));
                }
            } else if ie == IS::NOT {
                if state.stack.len() >= 1 {
                    let value = state.stack.pop().unwrap();
                    if value == 0 {
                        state.stack.push(1);
                    } else {
                        state.stack.push(0);
                    }
                }
            } else if ie == IS::MEMORYLEN {
                state.stack.push(state.memory.len() as u64);
            } else if ie == IS::AREALEN {
                let area = state.stack[stacklen - 1] as usize;
                state.stack[stacklen - 1 as usize] = state.memory[area].len() as u64;
            } else if ie == IS::READ {
                if state.stack.len() >= 2 {
                    let offset = state.stack.pop().unwrap() as usize;
                    let area = state.stack.pop().unwrap() as usize;
                    if area < state.memory.len() && offset < state.memory[area].len() {
                        state.stack.push(state.memory[area][offset]);
                    }
                }
            } else if ie == IS::WRITE {
                if state.stack.len() >= 3 {
                    let value = state.stack.pop().unwrap();
                    let offset = state.stack.pop().unwrap() as usize;
                    let area = state.stack.pop().unwrap() as usize;
                    if area < state.memory.len() && offset < state.memory[area].len() {
                        state.memory[area][offset] = value;
                    }
                }
            } else if ie == IS::AREA {
                state.memory.push(Vec::with_capacity(1024)); //XXX Vec::new()
            } else if ie == IS::ALLOC {
                let size = state.stack.pop().unwrap();
                let area = state.stack.pop().unwrap() as usize;
                for _i in 0..size {
                    state.memory[area].push(0); //XXX should use resize
                }
            } else if ie == IS::DEALLOC {
                let size = state.stack.pop().unwrap();
                let area = state.stack.pop().unwrap() as usize;
                for _i in 0..size {
                    state.memory[area].pop(); //XXX should use resize
                }
            } else if ie == IS::ROT2 {
                //stack length should already be checked!
                let first = state.stack[stacklen - 1];
                let second = state.stack[stacklen - 2];
                let third = state.stack[stacklen - 3];
                state.stack[stacklen - 1] = third;
                state.stack[stacklen - 2] = first;
                state.stack[stacklen - 3] = second;
            } else {
                state.header.status = Stati::UOC as u64;
                println!("UOC{:?}", instr);
                //break;
            }

            if !jump {
                state.header.ip += reqs.length as u64;
            }

            if reqs.addtl_mem < 0 {
                /*
                let range = -reqs[2];
                for i in 0..range {
                    state.stack.pop();
                }
                */
            }
            let _stackdiff: i64 = ((states[(states.len() - 1) as usize].stack.len() as i64)
                - (stacklen as i64)) as i64;
            let stateslen = states.len();
            for i in 0..states.len() {
                states[stateslen - i - 1].header.gas -= 1; //reqs[2] as u64;
                let memcost: u64 = (sizes[states.len() - i - 1]) * (reqs.gas_cost as u64); //stackdiff
                states[stateslen - i - 1].header.mem -= memcost;
            }
        }
    }
}

use std::collections::HashMap;

struct Container {
    sharp: Process,
    table: HashMap<u64, fn(&mut Process)>,
}

use std::io;
use std::io::prelude::*;
use std::time::Instant;
impl Container {
    fn new(sharp: Process) -> Container {
        let tab = HashMap::new();//<u64, fn(Process)>
        Container {
            sharp: sharp.clone(),
            table: tab,
        }
    }

    fn get(&self, key: u64) -> fn(&mut Process) {
        self.table[&key]
    }


    fn add_func(&mut self, key: u64, fun: fn(&mut Process)) {
        self.table.insert(key, fun);
    }

    fn run_io(&mut self) {//TODO mut sharp: Process

        let debug = false;
        const NSPERS: u32 = 1000000000;
        const START_GAS: u64 = 100000000000000;
        let loopstart = Instant::now();

        loop {
            let now = Instant::now();
            // TODO why clone()?
            self.sharp = run(self.sharp.clone(), START_GAS, 10000000000000000, debug);
            let elapsed = now.elapsed();

            if false {
                let gasdelta = (START_GAS - self.sharp.header.gas) as u32;
                let ips = NSPERS / (elapsed.subsec_nanos() / gasdelta);
                println!("{:?}", ips);
                //println!("{}", Stati::string_from_int(sharp.header.status as u8));
                //println!("{}", sharp.header.ip);
            }

            let stacklen = self.sharp.stack.len();

            //print!("{}", stacklen);
            if self.sharp.header.status == (Stati::YLD as u64)
                && stacklen >= 2
            {

                let funid = self.sharp.stack[stacklen - 2];
                let func = self.table.get(&funid).map(|x| *x);

                match func {
                    Some(func) => func(&mut self.sharp),
                    None => {
                        println!("invalid op");
                    }
                };

                self.sharp.stack.pop();
            } else if self.sharp.header.status != Stati::NOR as u64 {
                break;
            }

            // std::thread::sleep(std::time::Duration::from_millis(200));
        }

        let looptime = loopstart.elapsed();
        if true {
            let gasdelta = (START_GAS - self.sharp.header.gas) as u32;
            let ips = NSPERS / (looptime.subsec_nanos() / gasdelta);
            println!("{:?}", ips);
            //println!("{}", Stati::string_from_int(sharp.header.status as u8));
            //println!("{}", sharp.header.ip);
        }
    }
}

fn print42(sharp: &mut Process) {
    let ci: u32 = sharp.stack.pop().unwrap() as u32;
    let c = std::char::from_u32(ci).unwrap();
    print!("{}", c); //
    io::stdout().flush().ok().expect("Could not flush stdout");
}

use std::fs;
extern crate byteorder;
use std::io::Cursor;
use byteorder::{ByteOrder, BigEndian, WriteBytesExt, ReadBytesExt};

fn writeHelloBin() {
    let hello : Vec<u64> = vec![
        0, 0, 10000000, 100000000, 0, 713, 0, 0, 0, 6, 0, 19, 6, 0, 6, 1, 21, 6, 425, 4, 6, 0, 6,
        2, 21, 6, 0, 8, 16, 6, 2, 24, 6, 0, 8, 17, 18, 6, 0, 8, 16, 6, 1, 24, 32, 18, 6, 0, 8, 8,
        16, 6, 2, 24, 18, 6, 42, 6, 0, 8, 8, 17, 6, 1, 24, 17, 2, 6, 0, 8, 16, 6, 1, 24, 17, 6, 0,
        8, 16, 6, 2, 24, 17, 6, 0, 8, 16, 6, 0, 8, 17, 24, 22, 6, 0, 8, 32, 18, 4, 6, 0, 6, 3, 21,
        6, 0, 8, 16, 6, 2, 24, 6, 0, 8, 17, 18, 6, 0, 8, 16, 6, 1, 24, 32, 18, 6, 0, 8, 8, 16, 6,
        3, 24, 18, 6, 0, 8, 8, 8, 17, 32, 18, 6, 0, 8, 8, 17, 17, 6, 0, 8, 8, 17, 6, 2, 24, 6, 1,
        23, 17, 24, 6, 216, 5, 6, 0, 8, 8, 8, 17, 6, 2, 24, 17, 6, 0, 8, 8, 17, 17, 23, 17, 6, 0,
        6, 1, 21, 6, 0, 8, 16, 6, 1, 24, 32, 18, 6, 192, 6, 11, 4, 6, 0, 6, 1, 22, 6, 0, 8, 8, 17,
        17, 6, 1, 23, 6, 0, 8, 8, 17, 32, 18, 6, 133, 4, 6, 0, 8, 16, 6, 1, 24, 17, 6, 0, 8, 16, 6,
        2, 24, 17, 6, 0, 8, 16, 6, 0, 8, 17, 24, 22, 6, 0, 8, 32, 18, 4, 6, 0, 6, 2, 21, 6, 0, 8,
        16, 6, 2, 24, 6, 0, 8, 17, 18, 6, 0, 8, 16, 6, 1, 24, 32, 18, 6, 0, 8, 8, 16, 6, 2, 24, 18,
        6, 0, 8, 8, 17, 6, 2, 24, 17, 6, 0, 8, 8, 17, 6, 2, 24, 6, 1, 23, 17, 6, 0, 6, 2, 21, 6, 0,
        8, 16, 6, 1, 24, 32, 18, 6, 0, 8, 16, 6, 2, 24, 32, 18, 6, 332, 6, 90, 4, 6, 0, 6, 2, 22,
        6, 0, 6, 1, 21, 6, 0, 8, 16, 6, 1, 24, 6, 10, 18, 6, 0, 16, 6, 1, 24, 6, 1, 6, 0, 6, 2, 21,
        6, 0, 8, 16, 6, 1, 24, 32, 18, 6, 0, 8, 16, 6, 2, 24, 32, 18, 6, 388, 6, 90, 4, 6, 0, 6, 2,
        22, 6, 0, 8, 16, 6, 1, 24, 17, 6, 0, 8, 16, 6, 2, 24, 17, 6, 0, 8, 16, 6, 0, 8, 17, 24, 22,
        6, 0, 8, 32, 18, 4, 6, 0, 6, 5, 21, 6, 0, 8, 16, 6, 2, 24, 6, 0, 8, 17, 18, 6, 0, 8, 16, 6,
        1, 24, 32, 18, 6, 0, 8, 8, 16, 6, 5, 24, 18, 6, 0, 6, 13, 21, 6, 0, 8, 16, 6, 13, 24, 6,
        104, 18, 6, 0, 8, 16, 6, 12, 24, 6, 101, 18, 6, 0, 8, 16, 6, 11, 24, 6, 108, 18, 6, 0, 8,
        16, 6, 10, 24, 6, 108, 18, 6, 0, 8, 16, 6, 9, 24, 6, 111, 18, 6, 0, 8, 16, 6, 8, 24, 6, 44,
        18, 6, 0, 8, 16, 6, 7, 24, 6, 32, 18, 6, 0, 8, 16, 6, 6, 24, 6, 119, 18, 6, 0, 8, 16, 6, 5,
        24, 6, 111, 18, 6, 0, 8, 16, 6, 4, 24, 6, 114, 18, 6, 0, 8, 16, 6, 3, 24, 6, 108, 18, 6, 0,
        8, 16, 6, 2, 24, 6, 100, 18, 6, 0, 8, 16, 6, 1, 24, 6, 33, 18, 6, 0, 16, 6, 13, 24, 6, 13,
        6, 0, 8, 8, 17, 6, 1, 23, 6, 1, 23, 32, 18, 6, 0, 8, 8, 17, 6, 1, 23, 32, 18, 6, 0, 8, 8,
        17, 6, 1, 23, 17, 6, 0, 8, 8, 17, 6, 1, 23, 6, 1, 23, 17, 6, 0, 6, 2, 21, 6, 0, 8, 16, 6,
        1, 24, 32, 18, 6, 0, 8, 16, 6, 2, 24, 32, 18, 6, 675, 6, 248, 4, 6, 0, 6, 2, 22, 6, 0, 8,
        8, 16, 6, 1, 24, 17, 6, 0, 8, 16, 6, 2, 24, 17, 6, 0, 8, 16, 6, 0, 8, 17, 24, 22, 6, 0, 8,
        32, 18, 0,
    ];

    let mut vec8: Vec<u8> = vec![0;hello.len()*8];
    BigEndian::write_u64_into(&hello, &mut vec8);

    println!("Writing hello.bin: {} bytes", vec8.len());
    fs::write("hello.bin", vec8).expect("Writing hello.bin failed");
}

fn main() {

    writeHelloBin();

    let data: Vec<u8> = fs::read("hello.bin").expect("Unable to read file");
    println!("Read: {} bytes", data.len());
    let mut flat: Vec<u64> = vec![0;data.len()/8];
    BigEndian::read_u64_into(&data, &mut flat);
    //let mut rdr = Cursor::new(data);
    //println!("{} {:?}", data.len(), flat);

    //println!("Start");
    let mut sharp: Process = d(&flat);

    //TODO from_sharp

    let mut instance = Container::new(sharp);
    instance.add_func(42, print42);
    instance.run_io();

    //println!("Done.");
}
