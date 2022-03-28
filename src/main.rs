
mod vm;

fn main() {

    let program = include_str!("main.bf");
    
    let mut prog_vm = vm::vm::new();

    prog_vm.load_program(&String::from(program));

    prog_vm.run();
    
}
