use std::{
    fs::{File, metadata},
    io::{Read, Write, BufReader, BufWriter, BufRead},
    env,
    time::Instant,
    path::Path,
};

// Convenience functions for buffered I/O ---------------------------
#[derive(PartialEq, Eq)]
enum BufferState {
    NotEmpty,
    Empty,
}

trait BufferedRead {
    fn read_byte(&mut self, input: &mut [u8; 1]) -> usize;
    fn fill_buffer(&mut self) -> BufferState;
}
impl BufferedRead for BufReader<File> {
    fn read_byte(&mut self, input: &mut [u8; 1]) -> usize {
        let bytes_read = self.read(input).unwrap();
        if self.buffer().is_empty() { 
            self.consume(self.capacity()); 
            self.fill_buf().unwrap();
        }
        bytes_read
    }
    fn fill_buffer(&mut self) -> BufferState {
        self.consume(self.capacity());
        match self.fill_buf() {
            Ok(_)  => {},
            Err(e) => { 
                println!("Function fill_buffer failed."); 
                println!("Error: {}", e);
            },
        }
        if self.buffer().is_empty() { 
            return BufferState::Empty; 
        }
        BufferState::NotEmpty
    }
}
trait BufferedWrite {
    fn write_byte(&mut self, output: u8);
    fn flush_buffer(&mut self);
}
impl BufferedWrite for BufWriter<File> {
    fn write_byte(&mut self, output: u8) {
        match self.write(&[output]) {
            Ok(_)  => {},
            Err(e) => { 
                println!("Function write_byte failed."); 
                println!("Error: {}", e);
            },
        }
        if self.buffer().len() >= self.capacity() { 
            match self.flush() {
                Ok(_)  => {},
                Err(e) => { 
                    println!("Function write_byte failed."); 
                    println!("Error: {}", e);
                },
            } 
        }
    }
    fn flush_buffer(&mut self) {
        match self.flush() {
            Ok(_)  => {},
            Err(e) => { 
                println!("Function flush_buffer failed."); 
                println!("Error: {}", e);
            },
        }    
    }
}
fn new_input_file(capacity: usize, file_name: &str) -> BufReader<File> {
    BufReader::with_capacity(capacity, File::open(file_name).unwrap())
}
fn new_output_file(capacity: usize, file_name: &str) -> BufWriter<File> {
    BufWriter::with_capacity(capacity, File::create(file_name).unwrap())
}
// ------------------------------------------------------------------


const STATE_TABLE: [[u8; 2]; 256] = [
[  1,  2],[  3,  5],[  4,  6],[  7, 10],[  8, 12],[  9, 13],[ 11, 14], // 0
[ 15, 19],[ 16, 23],[ 17, 24],[ 18, 25],[ 20, 27],[ 21, 28],[ 22, 29], // 7
[ 26, 30],[ 31, 33],[ 32, 35],[ 32, 35],[ 32, 35],[ 32, 35],[ 34, 37], // 14
[ 34, 37],[ 34, 37],[ 34, 37],[ 34, 37],[ 34, 37],[ 36, 39],[ 36, 39], // 21
[ 36, 39],[ 36, 39],[ 38, 40],[ 41, 43],[ 42, 45],[ 42, 45],[ 44, 47], // 28
[ 44, 47],[ 46, 49],[ 46, 49],[ 48, 51],[ 48, 51],[ 50, 52],[ 53, 43], // 35
[ 54, 57],[ 54, 57],[ 56, 59],[ 56, 59],[ 58, 61],[ 58, 61],[ 60, 63], // 42
[ 60, 63],[ 62, 65],[ 62, 65],[ 50, 66],[ 67, 55],[ 68, 57],[ 68, 57], // 49
[ 70, 73],[ 70, 73],[ 72, 75],[ 72, 75],[ 74, 77],[ 74, 77],[ 76, 79], // 56
[ 76, 79],[ 62, 81],[ 62, 81],[ 64, 82],[ 83, 69],[ 84, 71],[ 84, 71], // 63
[ 86, 73],[ 86, 73],[ 44, 59],[ 44, 59],[ 58, 61],[ 58, 61],[ 60, 49], // 70
[ 60, 49],[ 76, 89],[ 76, 89],[ 78, 91],[ 78, 91],[ 80, 92],[ 93, 69], // 77
[ 94, 87],[ 94, 87],[ 96, 45],[ 96, 45],[ 48, 99],[ 48, 99],[ 88,101], // 84
[ 88,101],[ 80,102],[103, 69],[104, 87],[104, 87],[106, 57],[106, 57], // 91
[ 62,109],[ 62,109],[ 88,111],[ 88,111],[ 80,112],[113, 85],[114, 87], // 98
[114, 87],[116, 57],[116, 57],[ 62,119],[ 62,119],[ 88,121],[ 88,121], // 105
[ 90,122],[123, 85],[124, 97],[124, 97],[126, 57],[126, 57],[ 62,129], // 112
[ 62,129],[ 98,131],[ 98,131],[ 90,132],[133, 85],[134, 97],[134, 97], // 119
[136, 57],[136, 57],[ 62,139],[ 62,139],[ 98,141],[ 98,141],[ 90,142], // 126
[143, 95],[144, 97],[144, 97],[ 68, 57],[ 68, 57],[ 62, 81],[ 62, 81], // 133
[ 98,147],[ 98,147],[100,148],[149, 95],[150,107],[150,107],[108,151], // 140
[108,151],[100,152],[153, 95],[154,107],[108,155],[100,156],[157, 95], // 147
[158,107],[108,159],[100,160],[161,105],[162,107],[108,163],[110,164], // 154
[165,105],[166,117],[118,167],[110,168],[169,105],[170,117],[118,171], // 161
[110,172],[173,105],[174,117],[118,175],[110,176],[177,105],[178,117], // 168
[118,179],[110,180],[181,115],[182,117],[118,183],[120,184],[185,115], // 175
[186,127],[128,187],[120,188],[189,115],[190,127],[128,191],[120,192], // 182
[193,115],[194,127],[128,195],[120,196],[197,115],[198,127],[128,199], // 189
[120,200],[201,115],[202,127],[128,203],[120,204],[205,115],[206,127], // 196
[128,207],[120,208],[209,125],[210,127],[128,211],[130,212],[213,125], // 203
[214,137],[138,215],[130,216],[217,125],[218,137],[138,219],[130,220], // 210
[221,125],[222,137],[138,223],[130,224],[225,125],[226,137],[138,227], // 217
[130,228],[229,125],[230,137],[138,231],[130,232],[233,125],[234,137], // 224
[138,235],[130,236],[237,125],[238,137],[138,239],[130,240],[241,125], // 231
[242,137],[138,243],[130,244],[245,135],[246,137],[138,247],[140,248], // 238
[249,135],[250, 69],[ 80,251],[140,252],[249,135],[250, 69],[ 80,251], // 245
[140,252],[  0,  0],[  0,  0],[  0,  0]];                              // 252

fn next_state(state: u8, bit: i32) -> u8 {
    STATE_TABLE[state as usize][bit as usize]
}

#[allow(overflowing_literals)]
const HI_22_MSK: i32 = 0xFFFFFC00; // High 22 bit mask
const LIMIT: usize = 127; // Controls rate of adaptation (higher = slower) (0..512)

struct StateMap {
    cxt:      usize,       // Context of last prediction
    cxt_map:  Vec<u32>,    // Maps a context to a prediction and a count 
    rec_t:    [i32; 512],  // Reciprocal table: controls the size of each adjustment to cxt_map
}
impl StateMap {
    fn new(n: usize) -> StateMap {
        let mut sm = StateMap { 
            cxt:      0,
            cxt_map:  vec![1 << 31; n],
            rec_t:    [0; 512],
        };
        for i in 0..512 { 
            sm.rec_t[i] = (16_384 / (i + i + 3)) as i32; 
        }
        sm
    }
    fn p(&mut self, cxt: usize) -> i32 {                   
        self.cxt = cxt;
        (self.cxt_map[self.cxt] >> 20) as i32  
    }
    fn update(&mut self, bit: i32) {
        assert!(bit == 0 || bit == 1);  
        let count = (self.cxt_map[self.cxt] & 1023) as usize; // Low 10 bits
        let pr    = (self.cxt_map[self.cxt] >> 10 ) as i32;   // High 22 bits

        if count < LIMIT { self.cxt_map[self.cxt] += 1; }

        // Update cxt_map based on prediction error
        self.cxt_map[self.cxt] = self.cxt_map[self.cxt].wrapping_add(
        ((((bit << 22) - pr) >> 3) * self.rec_t[count] & HI_22_MSK) as u32); 
    }
}
// -----------------------------------------------------------------

// Predictor -------------------------------------------------------
struct Predictor {
    cxt:    usize,
    sm:     StateMap,
    state:  [u8; 256],
}
impl Predictor {
    fn new() -> Predictor {
        Predictor {
            cxt:    0,
            sm:     StateMap::new(65536),
            state:  [0; 256],
        }
    }
    fn p(&mut self) -> i32 { 
        self.sm.p(self.cxt * 256 + self.state[self.cxt] as usize) 
    } 
    fn update(&mut self, bit: i32) {
        self.sm.update(bit);

        self.state[self.cxt] = next_state(self.state[self.cxt], bit);

        self.cxt += self.cxt + bit as usize;
        if self.cxt >= 256 { self.cxt = 0; }
    }
}
// -----------------------------------------------------------------

// Encoder ----------------------------------------------------------
struct Encoder {
    predictor:  Predictor,
    high:       u32,
    low:        u32,
    file_out:   BufWriter<File>,
}
impl Encoder {
    fn new(file_out: BufWriter<File>) -> Encoder {
        Encoder {
            predictor: Predictor::new(), 
            high: 0xFFFFFFFF, 
            low: 0,  
            file_out,
        }
    }
    fn encode(&mut self, bit: i32) {
        let p = self.predictor.p() as u32;
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * p 
                       + ((self.high - self.low & 0x0FFF) * p >> 12);
        if bit == 1 { 
            self.high = mid;    
        } 
        else {        
            self.low = mid + 1; 
        }
        self.predictor.update(bit);

        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.file_out.write_byte((self.high >> 24) as u8);
            self.high = (self.high << 8) + 255;
            self.low <<= 8;  
        }
    }
    fn flush(&mut self) {
        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.file_out.write_byte((self.high >> 24) as u8);
            self.high = (self.high << 8) + 255;
            self.low <<= 8; 
        }
        self.file_out.write_byte((self.high >> 24) as u8);
        self.file_out.flush_buffer();
    }
}
// ------------------------------------------------------------------


// Decoder ----------------------------------------------------------
struct Decoder {
    predictor:  Predictor,
    high:       u32,
    low:        u32,
    x:          u32,
    file_in:    BufReader<File>,   
}
impl Decoder {
    fn new(file_in: BufReader<File>) -> Decoder {
        let mut dec = Decoder {
            predictor: Predictor::new(), 
            high: 0xFFFFFFFF, 
            low: 0, 
            x: 0, 
            file_in, 
        };
        for _ in 0..4 {
            let mut byte = [0; 1];
            dec.file_in.read_byte(&mut byte);
            dec.x = (dec.x << 8) + byte[0] as u32;
        }
        dec
    }
    fn decode(&mut self) -> i32 {
        let mut byte = [0; 1];
        let mut bit: i32 = 0;

        let p = self.predictor.p() as u32;
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * p 
                       + ((self.high - self.low & 0x0FFF) * p >> 12);

        if self.x <= mid {
            bit = 1;
            self.high = mid;
        } 
        else {
            self.low = mid + 1;
        }
        self.predictor.update(bit);
        
        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.high = (self.high << 8) + 255;
            self.low <<= 8;
            self.file_in.read_byte(&mut byte); 
            self.x = (self.x << 8) + byte[0] as u32; 
        }
        bit
    }
}
// ------------------------------------------------------------------

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    
    let mut file_in  = new_input_file(4096, &args[2]);
    let mut file_out = new_output_file(4096, &args[3]);
    match (&args[1]).as_str() {
        "c" => {  
            let mut enc = Encoder::new(file_out);
            let mut byte = [0; 1];

            while file_in.read_byte(&mut byte) != 0 { 
                enc.encode(1);
                for i in (0..=7).rev() {
                    enc.encode(((byte[0] >> i) & 1) as i32);
                } 
            }   
            enc.encode(0);
            enc.flush(); 
            println!("Finished Compressing.");     
        }
        "d" => {
            let mut dec = Decoder::new(file_in);
            
            while dec.decode() != 0 {   
                let mut dec_byte: i32 = 1;
                while dec_byte < 256 {
                    dec_byte += dec_byte + dec.decode();
                }
                dec_byte -= 256;
                file_out.write_byte((dec_byte & 0xFF) as u8);
            }
            file_out.flush_buffer();
            println!("Finished Decompressing.");   
        }
        _ => {  
            println!("Enter 'c input output' to compress");
            println!("Enter 'd input output' to decompress"); 
        } 
    } 
    let file_in_size = metadata(Path::new(&args[2])).unwrap().len();
    let file_out_size = metadata(Path::new(&args[3])).unwrap().len();
    println!("{} bytes -> {} bytes in {:.2?}", 
    file_in_size, file_out_size, start_time.elapsed());  
}
