#[derive(Debug, num_derive::FromPrimitive)]
pub enum Stati {
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

#[derive(Debug, Clone)]
pub struct Header {
    pub status: u64,
    pub rec: u64,
    pub gas: u64,
    pub mem: u64,
    pub ip: u64,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub header: Header,
    pub code: Vec<u64>,
    pub stack: Vec<u64>,
    pub map: Vec<u64>,
    pub memory: Vec<Vec<u64>>,
}

/**
Deserialize the standard process snapshot format to the internal representation
*/
pub fn d(flat: &Vec<u64>) -> Process {
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
pub fn s(sharp: &Process) -> Vec<u64> {
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
