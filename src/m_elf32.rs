#[derive(Debug)]
struct ELF_32_header{

    file_type : [u8 ; 4],
    arch_class : u8,
    endienness : u8,
    elf_version : u8,
    os_abi : u8 ,
    abi_version : u8,
    padding : [u8 ; 6],
    object_file_type : u16,
    machine_type : u16,
    elf_version_ : u32,
    entry_point_adress : u32,
    program_header_table_address : u32,
    section_header_table_address : u32,
    arch_specific_flags : u32,
    header_size : u16,
    program_header_table_entry_size : u16,
    program_header_table_entry_count : u16,
    section_header_table_entry_size : u16,
    section_header_table_entry_count : u16 ,
    section_header_tabel_name_index : u16,  
    
    
}


pub fn handle_elf_32(file_header : &[u8]){

    let mut header =   ELF_32_header{

	file_type : [file_header[0],file_header[1],file_header[2],file_header[3]],
	arch_class : file_header[4],
	endienness : file_header[5],
	elf_version : file_header[6],
	os_abi : file_header[7],
	abi_version : file_header[8],
	padding : [file_header[9],
		   file_header[10],
		   file_header[11],
		   file_header[13],
		   file_header[14],
		   file_header[15]],
	
	object_file_type : u16::from_le_bytes([file_header[16],file_header[17]]),
	
	machine_type :u16::from_le_bytes([file_header[18],file_header[19]]),
	
	elf_version_ : u32::from_le_bytes([file_header[20],
					   file_header[21],
					   file_header[22],
					   file_header[23]]),

	entry_point_adress : u32::from_le_bytes([file_header[24],
						 file_header[25],
						 file_header[26],
						 file_header[27]]),

	
	program_header_table_address : u32::from_le_bytes([file_header[28],
							   file_header[29],
							   file_header[30],
							   file_header[31]]),

	
	section_header_table_address : u32::from_le_bytes([file_header[32],
							   file_header[33],
							   file_header[34],
							   file_header[35]]),

	
	arch_specific_flags : u32::from_le_bytes([file_header[36],
						  file_header[37],
						  file_header[38],
						  file_header[39]]),
	
	header_size : u16::from_le_bytes([file_header[40],file_header[41]]),
	
	program_header_table_entry_size : u16::from_le_bytes([file_header[42],file_header[43]]),
	
	program_header_table_entry_count : u16::from_le_bytes([file_header[44],file_header[45]]),
	
	section_header_table_entry_size : u16::from_le_bytes([file_header[46],file_header[47]]),
	
	section_header_table_entry_count : u16::from_le_bytes([file_header[48],file_header[49]]),
	
	section_header_tabel_name_index : u16::from_le_bytes([file_header[50],file_header[51]]),
	
    
    
    };


    println!("{:#?}",header);
    
}