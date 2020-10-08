use vm::*;
use formats::*;

use std::collections::HashMap;
use std::time::Instant;

pub struct Container {
    sharp: Process,
    table: HashMap<u64, fn(&mut Process)>,
}

#[allow(dead_code)]
impl Container {
    pub fn new(sharp: Process) -> Container {
        let tab = HashMap::new();
        Container {
            sharp: sharp.clone(),
            table: tab,
        }
    }

    pub fn get(&self, key: u64) -> fn(&mut Process) {
        self.table[&key]
    }

    pub fn add_func(&mut self, key: u64, fun: fn(&mut Process)) {
        self.table.insert(key, fun);
    }

    pub fn run_io(&mut self) {//TODO mut sharp: Process

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
