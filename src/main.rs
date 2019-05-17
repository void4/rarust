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

//Instruction length, stack reqs, additional memory, gas cost
const REQS: [[i64; 4]; 33] = [
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [1, 3, -3, 0],
    [1, 1, -1, 1],
    [1, 2, -2, 1],
    [2, 0, 1, 2],
    [1, 0, -1, 2], //XXX changed stack effect
    [1, 0, 1, 4],
    [1, 2, 0, 4],
    [1, 2, -2, 10], //keys
    [1, 1, 0, 4],
    [1, 1, 0, 6],
    [1, 1, -1, 4],
    [1, 0, 1, 2], //lens
    [1, 0, 1, 2],
    [1, 1, 0, 2],
    [1, 2, -1, 2], //r/w
    [1, 3, -3, 2],
    [1, 0, 1, 10],  //a/d
    [1, 1, -1, 10], //use after free!
    [1, 2, -2, 10], //alloc/dealloc
    [1, 2, -2, 10],
    [1, 2, -1, 6],
    [1, 2, -1, 6],
    [1, 1, 0, 4],
    [1, 2, -1, 8],
    [1, 2, -1, 10],
    [1, 2, -1, 10],
    [1, 1, 0, 100],
    [1, 1, 0, 100],
    [1, 3, 0, 10],
    [1, 3, 0, 10],
];

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

fn run(sharp: Process, gas: u64, mem: u64, debug: bool) -> Process {
    //println!("Length of binary: {0}", flat.len());
    // Process, previously serialized length, rec index
    let mut edges: Vec<u64> = vec![0];
    let mut sizes: Vec<u64> = vec![s(&sharp).len() as u64];
    let mut states: Vec<(Process)> = vec![sharp]; //d(&flat)

    let mut reqs: [i64; 4] = [0, 0, 0, 0];
    let statelen = states.len() - 1 as usize;
    states[statelen].header.status = Stati::NOR as u64;
    states[statelen].header.gas = gas;
    states[statelen].header.mem = mem;

    loop {
        let mut jump_back: i64 = -2;
        let blockret = {
            let mut instr: u64 = 0;
            let ref mut state = states[statelen];
            //println!("{:?} {:?}", state.header.gas, state.header.ip);
            if debug {
                println!("{:?}", state.stack);
            }

            if state.header.status != Stati::NOR as u64 {
                //&& state.header.status != REC
                jump_back = (statelen as i64) - 1;
            } else if state.header.ip >= state.code.len() as u64 {
                state.header.status = Stati::OOC as u64;
                jump_back = (statelen as i64) - 1;
            } else {
                instr = state.code[state.header.ip as usize];
                if instr >= REQS.len() as u64 {
                    state.header.status = Stati::UOC as u64;
                    jump_back = (statelen as i64) - 1;
                } else {
                    reqs = REQS[instr as usize];

                    if state.header.ip + (reqs[0] as u64) - 1 >= state.code.len() as u64 {
                        state.header.status = Stati::OOA as u64;
                        jump_back = (statelen as i64) - 1;
                    }

                    if reqs[1] as u64 > state.stack.len() as u64 {
                        state.header.status = Stati::OOS as u64;
                        jump_back = (statelen as i64) - 1;
                    }
                }
            }
            (instr.clone(), reqs.clone())
        };

        let instr: u64 = blockret.0;
        reqs = blockret.1;

        fn valid_area(index: u64, process: &Process) -> bool {
            return index < process.memory.len() as u64;
        }

        if jump_back == -2 {
            for psi in (0..states.len()).rev() {
                let gascost = reqs[3] as u64;
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
                state.header.ip += reqs[0] as u64;
            }

            if reqs[2] < 0 {
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
                let memcost: u64 = (sizes[states.len() - i - 1]) * (reqs[3] as u64); //stackdiff
                states[stateslen - i - 1].header.mem -= memcost;
            }
        }
    }
}
use std::io;
use std::io::prelude::*;
use std::time::Instant;
fn main() {
    let flat = vec![
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
    //println!("Start");
    let debug = false;
    const NSPERS: u32 = 1000000000;
    const START_GAS: u64 = 100000000000000;
    let loopstart = Instant::now();
    let mut sharp: Process = d(&flat);
    loop {
        let now = Instant::now();

        sharp = run(sharp, START_GAS, 10000000000000000, debug);
        let elapsed = now.elapsed();

        if false {
            let gasdelta = (START_GAS - sharp.header.gas) as u32;
            let ips = NSPERS / (elapsed.subsec_nanos() / gasdelta);
            println!("{:?}", ips);
            //println!("{}", Stati::string_from_int(sharp.header.status as u8));
            //println!("{}", sharp.header.ip);
        }

        let stacklen = sharp.stack.len();

        //print!("{}", stacklen);
        if sharp.header.status == (Stati::YLD as u64)
            && stacklen >= 2
            && sharp.stack[stacklen - 2] == 42
        {
            let ci: u32 = sharp.stack.pop().unwrap() as u32;
            let c = std::char::from_u32(ci).unwrap();
            print!("{}", c); //
            io::stdout().flush().ok().expect("Could not flush stdout");
            sharp.stack.pop();
        } else if sharp.header.status != Stati::NOR as u64 {
            break;
        }

        // std::thread::sleep(std::time::Duration::from_millis(200));
    }
    let looptime = loopstart.elapsed();
    if true {
        let gasdelta = (START_GAS - sharp.header.gas) as u32;
        let ips = NSPERS / (looptime.subsec_nanos() / gasdelta);
        println!("{:?}", ips);
        //println!("{}", Stati::string_from_int(sharp.header.status as u8));
        //println!("{}", sharp.header.ip);
    }

    //println!("Done.");
}
