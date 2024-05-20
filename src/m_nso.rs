#[derive(Debug)]
struct Segment_header{
    file_offset : u32 ,
    memory_offset : u32,
    size : u32 ,
}
#[derive(Debug)]
struct Section_header{

    offset : u32,
    size : u32,
}
#[derive(Debug)]
struct NSO_0_header{

    magic : [u8 ; 4],
    format_version : u32,
    reserved : [u8 ; 4],
    flags : [u8 ; 4],
    
    text_segment_header : Segment_header,
    module_name_offset : u32,
    
    ro_data_segment_header : Segment_header,
    module_name_size : u32 ,
    
    data_segment_header : Segment_header,
    bss_size : u32 ,
    
    module_id : [u8 ; 32],
    
    text_file_size : u32 ,
    ro_file_size : u32 ,
    data_file_size : u32 ,
    
    reserved_2 : [u8 ; 28],
    
    embedded : Section_header,
    dyn_str : Section_header,
    dyn_sym : Section_header,
    
    text_hash : [u8 ; 32],
    ro_hash : [u8 ; 32],
    data_hash : [u8 ; 32]

}

pub fn handle_nso_0(file_header : &[u8]){

    let mut header = NSO_0_header{

	magic : [file_header[0],file_header[1],file_header[2],file_header[3]],
	format_version : u32::from_le_bytes([file_header[4],file_header[5],file_header[6],file_header[7]]),
	reserved : [file_header[8],file_header[9],file_header[10],file_header[11]],
	flags : [file_header[12],file_header[13],file_header[14],file_header[15]],

	
	text_segment_header : Segment_header{
	    file_offset : u32::from_le_bytes([file_header[16],file_header[17],file_header[18],file_header[19]]),
	    memory_offset : u32::from_le_bytes([file_header[20],file_header[21],file_header[22],file_header[23]]),
	    size : u32::from_le_bytes([file_header[24],file_header[25],file_header[26],file_header[27]]),
	},

	module_name_offset : u32::from_le_bytes([file_header[28],file_header[29],file_header[30],file_header[31]]),

	
	ro_data_segment_header : Segment_header{
	    file_offset : u32::from_le_bytes([file_header[32],file_header[33],file_header[34],file_header[35]]),
	    memory_offset : u32::from_le_bytes([file_header[36],file_header[37],file_header[38],file_header[39]]),
	    size : u32::from_le_bytes([file_header[40],file_header[41],file_header[42],file_header[43]]),
	},
	
	module_name_size : u32::from_le_bytes([file_header[44],file_header[45],file_header[46],file_header[47]]),

	data_segment_header : Segment_header{
	    file_offset : u32::from_le_bytes([file_header[48],file_header[49],file_header[50],file_header[51]]),
	    memory_offset : u32::from_le_bytes([file_header[52],file_header[53],file_header[54],file_header[55]]),
	    size : u32::from_le_bytes([file_header[56],file_header[57],file_header[58],file_header[59]]),
	},

	bss_size : u32::from_le_bytes([file_header[60],file_header[61],file_header[62],file_header[63]]),

	module_id : {
	    let mut ret : [u8 ; 32] = [0 ; 32];
	    let mut counter = 64;
	    let mut index = 0;
	    loop{
		if counter == 96 {
		    break;
		}
		ret[index] = file_header[counter];
		index += 1;
		counter += 1;
	    }

	    ret
	    
	},
	
	text_file_size : u32::from_le_bytes([file_header[96],file_header[97],file_header[98],file_header[99]]),
	ro_file_size : u32::from_le_bytes([file_header[100],file_header[101],file_header[102],file_header[103]]),
	data_file_size : u32::from_le_bytes([file_header[104],file_header[105],file_header[106],file_header[107]]),

	reserved_2 :  {
	    let mut ret : [u8 ; 28] = [0 ; 28];
	    let mut counter = 108;
	    let mut index = 0;
	    loop{
		if counter == 136 {
		    break;
		}
		ret[index] = file_header[counter];
		index += 1;
		counter += 1;
	    }

	    ret
	    
	},


	embedded : Section_header{
	    offset : u32::from_le_bytes([file_header[136],file_header[137],file_header[138],file_header[139]]),
	    size : u32::from_le_bytes([file_header[140],file_header[141],file_header[142],file_header[143]]),
	},

	
	dyn_str : Section_header{
	    offset : u32::from_le_bytes([file_header[144],file_header[145],file_header[146],file_header[147]]),
	    size : u32::from_le_bytes([file_header[148],file_header[149],file_header[150],file_header[151]]),
	},

	
	dyn_sym : Section_header{
	    offset : u32::from_le_bytes([file_header[152],file_header[153],file_header[154],file_header[155]]),
	    size : u32::from_le_bytes([file_header[156],file_header[157],file_header[158],file_header[159]]),
	},

	text_hash : {
	    let mut ret : [u8 ; 32] = [0 ; 32];
	    let mut counter = 160;
	    let mut index = 0;
	    loop{
		if counter == 192 {
		    break;
		}
		ret[index] = file_header[counter];
		index += 1;
		counter += 1;
	    }

	    ret
	    
	},
	
	ro_hash :  {
	    let mut ret : [u8 ; 32] = [0 ; 32];
	    let mut counter = 192;
	    let mut index = 0;
	    loop{
		if counter == 224 {
		    break;
		}
		ret[index] = file_header[counter];
		index += 1;
		counter += 1;
	    }

	    ret
	    
	},

	
	data_hash :  {
	    let mut ret : [u8 ; 32] = [0 ; 32];
	    let mut counter = 224;
	    let mut index = 0;
	    loop{
		if counter == 256 {
		    break;
		}
		ret[index] = file_header[counter];
		index += 1;
		counter += 1;
	    }

	    ret
	    
	},
	
    };

    println!("{:#?}",header);
}
