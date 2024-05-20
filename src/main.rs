#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod m_elf64;
pub mod m_elf32;
pub mod m_nso;

use crate::m_elf64::handle_elf_64;
use crate::m_elf32::handle_elf_32;
use crate::m_nso::handle_nso_0;

use std::str;
use std::env;
use std::fs::File;
use std::io::Read;


enum EXEC_T{
    ELF32 ,
    ELF64 ,
    NSO0 ,
    INVALID,
}



fn get_file_from_argv() -> Vec<u8> {

    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1]).unwrap();

    let file_len = file.metadata().unwrap().len() as usize;

    let mut buffer = vec![ 0u8 ; file_len];
    
    let _ = file.read(&mut buffer).unwrap();

    return buffer;

}


fn get_exe_type(e_ident :  &[u8] , e_class : u8) -> EXEC_T{

    let file_name : &str = str::from_utf8(e_ident).unwrap();

    let elf_id : [u8 ; 4] = [0x7F , 0x45 , 0x4C , 0x46];
    let nso_id : [u8 ; 4] = [0x4E , 0x53 , 0x4F , 0x30];

    let elf_name : &str = str::from_utf8(&elf_id).unwrap();
    let nso_name : &str = str::from_utf8(&nso_id).unwrap();

    if file_name == elf_name {
	if e_class == 1 {
	    return EXEC_T::ELF32;
	}
	if e_class == 2 {
	    return EXEC_T::ELF64;
	}
    }

    if file_name == nso_name{
	return EXEC_T::NSO0;
    }

    return EXEC_T::INVALID;
    
}

fn main() {

    let file = get_file_from_argv();

    match get_exe_type(&file[0..4] , file[4]){

	EXEC_T::ELF32 => handle_elf_32(&file[0..52]),

	EXEC_T::ELF64 => handle_elf_64(&file[0..64]),

	EXEC_T::NSO0 => handle_nso_0(&file[0..256]),

	EXEC_T::INVALID => println!("File is not an shared object or an executable"),

    };
    
}
