pub struct Processor<'a> {
    name: &'a str
}

impl<'a> Processor<'a> {
    pub fn new(name: &'a str) -> Processor<'a> {
        Processor {
            name
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn start(process: &Process) {
        
    }

}

pub struct Process<'a> {
    name: &'a str
}

impl<'a> Process<'a> {
    pub fn new(name: &'a str) -> Process<'a> {
        Process {
            name
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
}