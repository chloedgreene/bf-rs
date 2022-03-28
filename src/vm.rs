pub struct vm{

    pub program_counter:i16,
    pub program:Vec<char>,
    pub memory: [i8; 30000]

}

impl vm {
    
    pub fn new() -> vm{
        vm { 
            program_counter: 0,
            program: String::from("").chars().collect(),
            memory: [0; 30000] 
        }
    }

    pub fn load_program(&mut self, prog: &String){

        self.program = prog.clone().chars().collect();

    }

    pub fn run(&mut self){
     
        loop{

            if self.program_counter == self.program.len() as i16
            {
                break;
            }

            let prog = self.program[self.program_counter as usize];
        
            print!("{}",prog);
            self.program_counter += 1;
        }
        println!("\n")
        
    }

}