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
    
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter, BufRead};
use std::env;
use std::fs::metadata;
use std::time::Instant;
use std::path::Path;

// Convenience functions for buffered IO ---------------------------
fn write(buf_out: &mut BufWriter<File>, output: &[u8]) {
    buf_out.write(output).unwrap();
    if buf_out.buffer().len() >= buf_out.capacity() { 
        buf_out.flush().unwrap(); 
    }
}
fn read(buf_in: &mut BufReader<File>, input: &mut [u8; 1]) -> usize {
    let bytes_read = buf_in.read(input).unwrap();
    if buf_in.buffer().len() <= 0 { 
        buf_in.consume(buf_in.capacity()); 
        buf_in.fill_buf().unwrap();
    }
    bytes_read
}
// -----------------------------------------------------------------

// Logistic Functions ----------------------------------------------
fn squash(mut d: i32) -> i32 {
    const SQUASH_TABLE: [i32; 33] = [
    1,2,3,6,10,16,27,45,73,120,194,310,488,747,1101,
    1546,2047,2549,2994,3348,3607,3785,3901,3975,4022,
    4050,4068,4079,4085,4089,4092,4093,4094];
    if d > 2047 { return 4095; }
    if d < -2047 { return 0; }
    let w = d & 127;
    d = (d >> 7) + 16;
    (SQUASH_TABLE[d as usize] * (128 - w) + SQUASH_TABLE[(d + 1) as usize] * w + 64) >> 7
}
struct Stretch {
    stretch_table: [i16; 4096],
}
impl Stretch {
    fn new() -> Stretch {
        let mut s = Stretch {
            stretch_table: [0; 4096],
        };
        let mut pi = 0;
        for x in -2047..=2047 {
            let i = squash(x);
            for j in pi..=i {
                s.stretch_table[j as usize] = x as i16;
            }
            pi = i + 1;
        }
        s.stretch_table[4095] = 2047;
        s
    }
    fn stretch(&self, p: i32) -> i32 {
        assert!(p < 4096);
        self.stretch_table[p as usize] as i32
    }
}
// -----------------------------------------------------------------
/*
The output of the StateMap is passed through a series of 5 more adaptive tables, 
(Adaptive Probability Maps, or Apm) each of which maps a context and the input 
probability to an output probability.  The input probability is interpolated between
33 bins on a nonlinear scale with smaller bins near 0 and 1.  After each prediction,
the corresponding table entries on both sides of p are adjusted to improve the
last prediction.

apm1 and apm2 both take cxt (the preceding bits of the current byte) as additional 
context, but one is fast adapting and the other is slow adapting. Their 
outputs are averaged.

apm3 is an order 1 context (previous byte and current partial byte).

apm4 takes the current byte and the low 5 bits of the second byte back.

apm5 takes a 14 bit hash of an order 3 context (last 3 bytes plus current partial
byte) and is averaged with 1/2 weight to the apm4 output.
*/
// Adaptive Probability Map ----------------------------------------
struct Apm {
    stretcher:  Stretch,
    bin:        usize,    
    num_cxts:   usize, 
    bin_map:    Vec<u16>, // maps each bin to a squashed value
}
impl Apm {
    fn new(n: usize) -> Apm {
        let mut apm = Apm {  
            stretcher:  Stretch::new(), 
            bin:        0, // last pr, context
            num_cxts:   n,
            bin_map:    Vec::with_capacity(n * 33),
        };
        apm.bin_map.resize(n * 33, 0);

        for i in 0..apm.num_cxts {
            for j in 0usize..33 {
                apm.bin_map[(i * 33) + j] = if i == 0 {
                    (squash(((j as i32) - 16) * 128) * 16) as u16
                } else {
                    apm.bin_map[j]
                }
            }
        }
        apm
    }
    fn p(&mut self, bit: i32, mut pr: i32, cxt: usize, rate: i32) -> i32 {
        assert!(bit == 0 || bit == 1 && pr >= 0 && pr < 4096 && cxt < self.num_cxts);
        self.update(bit, rate);
        
        pr = self.stretcher.stretch(pr); // -2047 to 2047
        

        let interp_wght = pr & 127; // interpolation weight (33 points)
        
        // each context has a corresponding set of 33 bins, and bin is 
        // a specific bin within the set corresponding to the current context
        self.bin = (((pr + 2048) >> 7) + ((cxt as i32) * 33)) as usize;

        (((self.bin_map[self.bin]     as i32) * (128 - interp_wght) ) + 
        ( (self.bin_map[self.bin + 1] as i32) *        interp_wght) ) >> 11

    }
    fn update(&mut self, bit: i32, rate: i32) {
        assert!(bit == 0 || bit == 1 && rate > 0 && rate < 32);
        // g controls direction of update (bit = 1 - increase, bit = 0 - decrease)
        let g: i32 = (bit << 16) + (bit << rate) - bit - bit;
        self.bin_map[self.bin] = ((self.bin_map[self.bin] as i32) + 
                            ((g - (self.bin_map[self.bin] as i32)) >> rate)) as u16;

        self.bin_map[self.bin + 1] = ((self.bin_map[self.bin + 1] as i32) + 
                                ((g - (self.bin_map[self.bin + 1] as i32)) >> rate)) as u16;
    }
}
// -----------------------------------------------------------------

const N: usize = 65_536;  // number of contexts
const LIMIT: usize = 127; // controls rate of adaptation (higher = slower) (0..512)

// StateMap --------------------------------------------------------
struct StateMap {
    cxt:           usize,         // context of last prediction
    cxt_map:       Box<[u32; N]>, // maps a context to a prediction and a count (allocate on heap to avoid stack overflow)
    recipr_table:  [i32; 512],    // controls the size of each adjustment to cxt_map
}
impl StateMap {
    fn new() -> StateMap {
        let mut statemap = StateMap { 
            cxt:           0,
            cxt_map:       Box::new([0; N]),
            recipr_table:  [0; 512],
        };

        for i in 0..N {
            statemap.cxt_map[i] = 1 << 31;
        }

        if statemap.recipr_table[0] == 0 {
            for i in 0..512 { 
                statemap.recipr_table[i] = (32_768 / (i + i + 5)) as i32; 
            }
        }
        statemap
    }
    fn p(&mut self, bit: i32, cx: usize) -> i32 {
        assert!(bit == 0 || bit == 1);
        self.update(bit);                      // update prediction for previous context
        self.cxt = cx;
        (self.cxt_map[self.cxt] >> 20) as i32  // output prediction for new context
    }
    fn update(&mut self, bit: i32) {
        let count: usize = (self.cxt_map[self.cxt] & 511) as usize;  // low 9 bits
        let prediction: i32 = (self.cxt_map[self.cxt] >> 14) as i32; // high 18 bits

        if count < LIMIT { self.cxt_map[self.cxt] += 1; }

        // updates cxt_map based on the difference between the predicted and actual bit
        #[allow(overflowing_literals)]
        let high_23_bit_mask: i32 = 0xFFFFFE00;
        self.cxt_map[self.cxt] = self.cxt_map[self.cxt].wrapping_add(
        (((bit << 18) - prediction) * self.recipr_table[count] & high_23_bit_mask) as u32); 
    }
}
// -----------------------------------------------------------------

// Predictor -------------------------------------------------------
struct Predictor {
    cxt:        usize,      apm1: Apm,   
    cxt4:       usize,      apm2: Apm,   
    pr:         i32,        apm3: Apm,   
    state:      [u8; 256],  apm4: Apm,
    statemap:   StateMap,   apm5: Apm,     
}
impl Predictor {
    fn new() -> Predictor {
        Predictor {
            cxt:       0,                apm1: Apm::new(256),
            cxt4:      0,                apm2: Apm::new(256),
            pr:        2048,             apm3: Apm::new(65536),
            state:     [0; 256],         apm4: Apm::new(8192),
            statemap:  StateMap::new(),  apm5: Apm::new(16384),         
        }
    }
    fn p(&mut self) -> i32 { 
        assert!(self.pr >= 0 && self.pr < 4096);
        self.pr 
    } 
    fn update(&mut self, bit: i32) {
        assert!(bit == 0 || bit == 1);
        self.state[self.cxt] = STATE_TABLE[self.state[self.cxt] as usize][bit as usize];

        self.cxt += self.cxt + bit as usize;
        if self.cxt >= 256 {
            self.cxt4 = (self.cxt4 << 8) | (self.cxt - 256);  // shift new byte into cxt4
            self.cxt = 0;
        }

        self.pr = self.statemap.p(bit, self.cxt * 256 + self.state[self.cxt] as usize);
        
        self.pr = self.apm1.p(bit, self.pr,    self.cxt,  5 ) +
                  self.apm2.p(bit, self.pr,    self.cxt,  9 ) + 1 >> 1;
        
        self.pr = self.apm3.p(bit, self.pr,    self.cxt | (self.cxt4 << 8) & 0xFF00,  7 );
        
        self.pr = self.apm4.p(bit, self.pr,    self.cxt | (self.cxt4 & 0x1F00),  7 ) * 3 + self.pr + 2 >> 2;
        self.pr = self.apm5.p(bit, self.pr,  ( (self.cxt as u32) ^ (
                                             ( (self.cxt4 as u32) & 0xFFFFFF ).wrapping_mul(123456791)
                                                ) >> 18) as usize,               7 ) + self.pr + 1 >> 1;    
    }
}
// -----------------------------------------------------------------

// Encoder ---------------------------------------------------------
#[allow(dead_code)]
struct Encoder {
    high:       u32,
    low:        u32,
    predictor:  Predictor,
    file_in:    BufReader<File>,
    file_out:   BufWriter<File>,
    x:          u32,
    compress:   bool,
}
impl Encoder {
    fn new(file_in: BufReader<File>, file_out: BufWriter<File>, compress: bool) -> Encoder {
        let mut encoder = Encoder {
            high: 0xFFFFFFFF, 
            low: 0, 
            x: 0, 
            predictor: Predictor::new(), 
            file_in, file_out, compress
        };
        if !compress {
            for _i in 0..4 {
                let mut byte = [0; 1];
                read(&mut encoder.file_in, &mut byte);
                encoder.x = (encoder.x << 8) + byte[0] as u32;
            }
        }
        encoder
    }
    fn encode(&mut self, bit: i32) {
        let mut p = self.predictor.p() as u32;
        if p < 2048 { p += 1; } else {}
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * p + ((self.high - self.low & 0x0FFF) * p >> 12);
        if bit == 1 {
            self.high = mid;
        } else {
            self.low = mid + 1;
        }
        self.predictor.update(bit);

        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            write(&mut self.file_out, &self.high.to_le_bytes()[3..4]);
            self.high = (self.high << 8) + 255;
            self.low <<= 8;  
        }
    }
    fn decode(&mut self) -> i32 {
        let mut byte = [0; 1];
        let mut p = self.predictor.p() as u32;
        if p < 2048 { p += 1; } else {}
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * p + ((self.high - self.low & 0x0FFF) * p >> 12);
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
            read(&mut self.file_in, &mut byte); 
            self.x = (self.x << 8) + byte[0] as u32; 
        }
        bit
    }
    fn flush(&mut self) {
        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            write(&mut self.file_out, &self.high.to_le_bytes()[3..4]);
            self.high = (self.high << 8) + 255;
            self.low <<= 8; 
        }
        write(&mut self.file_out, &self.high.to_le_bytes()[3..4]);
        self.file_out.flush().unwrap();
    }
}
// -----------------------------------------------------------------

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    // main() buffers
    let mut file_in  = BufReader::with_capacity(4096, File::open(&args[2]).unwrap());
    let mut file_out = BufWriter::with_capacity(4096, File::create(&args[3]).unwrap());
    // Encoder buffers
    let e_file_in  =   BufReader::with_capacity(4096, File::open(&args[2]).unwrap());
    let e_file_out =   BufWriter::with_capacity(4096, File::create(&args[3]).unwrap());
    match (&args[1]).as_str() {
        "c" => {  
            let file_in_size = metadata(Path::new(&args[2])).unwrap().len();
            let mut e = Encoder::new(e_file_in, e_file_out, true);
            let mut byte = [0; 1];

            while read(&mut file_in, &mut byte) != 0 {
                e.encode(1);
                for i in (0..=7).rev() {
                    e.encode(((byte[0] >> i) & 1).into());
                } 
            }   
            e.encode(0);
            e.flush(); 
            
            let file_out_size = metadata(Path::new(&args[3])).unwrap().len();
            println!("Finished Compressing.");   
            println!("{} bytes -> {} bytes in {:.2?}", file_in_size, file_out_size, start_time.elapsed());    
        }
        "d" => {
            let file_in_size = metadata(Path::new(&args[2])).unwrap().len();
            let mut e = Encoder::new(e_file_in, e_file_out, false);
            
            while e.decode() != 0 {   
                let mut decoded_byte: i32 = 1;
                while decoded_byte < 256 {
                    decoded_byte += decoded_byte + e.decode();
                }
                decoded_byte -= 256;
                write(&mut file_out, &decoded_byte.to_le_bytes()[0..1]);
            }
            file_out.flush().unwrap();

            let file_out_size = metadata(Path::new(&args[3])).unwrap().len();
            println!("Finished Decompressing.");  
            println!("{} bytes -> {} bytes in {:.2?}", file_in_size, file_out_size, start_time.elapsed());   
        }
        _ => {
        println!("To compress: c input output");
        println!("To decompress: d input output");
        }
    } 
}





