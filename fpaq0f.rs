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
[140,252],[  0,  0],[  0,  0],[  0,  0]];  // 252

const N: usize = 65_536;  // number of contexts
const LIMIT: usize = 127; // controls rate of adaptation (higher = slower) (0..512)
    
use std::fs::File;
use std::io::prelude::*;
use std::env;

// StateMap --------------------------------------------------------

#[derive(Debug)]
struct StateMap {
    context:        usize,         // context of last prediction
    context_map:    Box<[u32; N]>, // maps a context to a prediction and a count (allocate on heap to avoid stack overflow)
    recipr_table:  [i32; 512],     // controls the size of each adjustment to context_map
}
impl StateMap {
    fn new() -> StateMap {
        let mut statemap = StateMap { 
            context:        0,
            context_map:    Box::new([0; N]),
            recipr_table:  [0; 512],
        };

        for i in 0..N { 
            statemap.context_map[i] = 1 << 31; 
        }
        if statemap.recipr_table[0] == 0 {
            for i in 0..512 { 
                statemap.recipr_table[i] = (32_768 / (i + i + 5)) as i32; 
            }
        }
        statemap
    }
    fn p(&mut self, cx: usize) -> u32 {
        self.context = cx;
        self.context_map[self.context] >> 16
    }
    fn update(&mut self, bit: i32) {
        let count: usize = (self.context_map[self.context] & 511) as usize;  // low 9 bits
        let prediction: i32 = (self.context_map[self.context] >> 14) as i32; // high 18 bits

        if count < LIMIT { self.context_map[self.context] += 1; }

        // updates context_map based on the difference between the predicted and actual bit
        #[allow(overflowing_literals)]
        let high_23_bit_mask: i32 = 0xFFFFFE00;
        self.context_map[self.context] = self.context_map[self.context].wrapping_add(
        (((bit << 18) - prediction) * self.recipr_table[count] & high_23_bit_mask) as u32);
        
    }
}

// -----------------------------------------------------------------

// Predictor -------------------------------------------------------
#[derive(Debug)]
struct Predictor {
    context:   usize,
    statemap:  StateMap,
    state:     [u8; 256],
}
impl Predictor {
    fn new() -> Predictor {
        Predictor {
            context:   0,
            statemap:  StateMap::new(),
            state:     [0; 256],
        }
    }
    fn p(&mut self) -> u32 { 
        self.statemap.p(self.context * 256 + self.state[self.context] as usize)
    } 
    fn update(&mut self, bit: i32) {
        self.statemap.update(bit);

        self.state[self.context] = STATE_TABLE[self.context][bit as usize];

        self.context += self.context + bit as usize;
        if self.context >= 256 { self.context = 0; }
    }
}
// -----------------------------------------------------------------

// Encoder ---------------------------------------------------------
#[derive(Debug)]
struct Encoder {
    high:       u32,
    low:        u32,
    predictor:  Predictor,
    file:       File,
    x:          u32,
    compress:   bool,
}
impl Encoder {
    fn new(predictor: Predictor, file: File, compress: bool) -> Encoder {
        let mut encoder = Encoder {
            high:  0xFFFFFFFF, 
            low:   0, 
            x:     0, 
            predictor, file, compress
        };
        if compress == false {
            for _i in 0..4 {
                let mut byte = [0; 1];
                encoder.file.read(&mut byte).unwrap();
                encoder.x = (encoder.x << 8) + (byte[0] & 0xFF) as u32;
            }
        }
        encoder
    }
    fn encode(&mut self, bit: i32) {
        let p: u32 = self.predictor.p();
        let mid: u32 = self.low + ((self.high - self.low) >> 16) * p + ((self.high - self.low & 0xFFFF) * p >> 16);
        if bit == 1 {
            self.high = mid;
        } else {
            self.low = mid + 1;
        }
        self.predictor.update(bit);

        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.file.write(&self.high.to_le_bytes()[3..4]).expect("Couldn't write to file.");
            self.high = (self.high << 8) + 255;
            self.low <<= 8;  
        }
    }
    fn decode(&mut self) -> i32 {
        let mut byte = [0; 1];
        let p: u32 = self.predictor.p();
        let mid: u32 = self.low + ((self.high - self.low) >> 16) * p + ((self.high - self.low & 0xFFFF) * p >> 16);
        let mut bit: i32 = 0;
        if self.x <= mid {
            bit = 1;
            self.high = mid;
        } else {
            self.low = mid + 1;
        }
        self.predictor.update(bit);
        
        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.high = (self.high << 8) + 255;
            self.low <<= 8;
            self.file.read(&mut byte).unwrap(); 
            self.x = (self.x << 8) + byte[0] as u32; 
        }
        bit
    }
    fn flush(&mut self) {
        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.file.write(&self.high.to_le_bytes()[3..4]).expect("Couldn't write to file.");
            self.high = (self.high << 8) + 255;
            self.low <<= 8; 
        }
        self.file.write(&self.high.to_le_bytes()[3..4]).expect("Couldn't write to file.");
    }
}
// -----------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_in = File::open(&args[2]).expect("Couldn't open input file.");
    let mut file_out = File::create(&args[3]).expect("Couldn't open output file.");
    match (&args[1]).as_str() {
        "c" => {  
            let predictor = Predictor::new();
            let mut encoder = Encoder::new(predictor, file_out, true);
            let mut byte = [0; 1];

            while file_in.read(&mut byte).unwrap() != 0 {
                encoder.encode(1);
                for i in (0..=7).rev() {
                    encoder.encode(((byte[0] >> i) & 1).into());
                } 
                 
            }   
            encoder.encode(0);
            encoder.flush(); 
            println!("Finished Compressing.");       
        }
        "d" => {
            let predictor = Predictor::new();
            let mut encoder = Encoder::new(predictor, file_in, false);
            
            while encoder.decode() != 0 {   
                let mut decoded_byte: i32 = 1;
                while decoded_byte < 256 {
                    decoded_byte += decoded_byte + encoder.decode();
                }
                decoded_byte -= 256;
                file_out.write(&decoded_byte.to_le_bytes()[0..1]).expect("Couldn't write to file.");
            }
            println!("Finished Decompressing.");   
        }
        _ => {
            println!("Enter 'c' to compress and 'd' to decompress");
        }
    } 
}

