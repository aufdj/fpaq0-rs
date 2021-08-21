use std::fs::File;
use std::fs::metadata;
use std::io::{Read, Write, BufReader, BufWriter, BufRead};
use std::env;
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

// Predictor -------------------------------------------------------
struct Predictor {
    context:     usize,
    context_map: [u32; 512],
}
impl Predictor {
    fn new() -> Predictor {
        let mut predictor = Predictor {
            context: 1, 
            context_map: [0; 512], // maps context to probability
        };
        for i in 0..512 {
            predictor.context_map[i] = 32768;
        }
        predictor
    }
    fn p(&mut self) -> u32 { 
        self.context_map[self.context] >> 4
    } 
    fn update(&mut self, bit: &usize) {
        if *bit == 1 { 
            self.context_map[self.context] += 65536 - self.context_map[self.context] >> 5; 
        } else { 
            self.context_map[self.context] -= self.context_map[self.context] >> 5; 
        }
        self.context += self.context + *bit;
        if self.context >= 512 { self.context = 1; }
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
    fn encode(&mut self, bit: usize) {
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * self.predictor.p();
        if bit == 1 { 
            self.high = mid;    
        } else {        
            self.low = mid + 1; 
        }
        self.predictor.update(&bit);

        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            write(&mut self.file_out, &self.high.to_le_bytes()[3..4]);
            self.high = (self.high << 8) + 255;
            self.low <<= 8;  
        }
    }
    fn decode(&mut self) -> usize {
        let mut byte = [0; 1];
        
        let mut bit: usize = 0;
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * self.predictor.p();
        if self.x <= mid {
            bit = 1;
            self.high = mid;
        } else {
            self.low = mid + 1;
        }
        self.predictor.update(&bit);
        
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
                e.encode(0);
                for i in (0..=7).rev() {
                    e.encode(((byte[0] >> i) & 1).into());
                } 
            }   
            e.encode(1);
            e.flush(); 
            
            let file_out_size = metadata(Path::new(&args[3])).unwrap().len();
            println!("Finished Compressing.");   
            println!("{} bytes -> {} bytes in {:.2?}", file_in_size, file_out_size, start_time.elapsed());    
        }
        "d" => {
            let file_in_size = metadata(Path::new(&args[2])).unwrap().len();
            let mut e = Encoder::new(e_file_in, e_file_out, false);
            
            while e.decode() != 1 {   
                let mut decoded_byte: usize = 1;
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
        _ => { println!("Enter 'c input output' to compress and 'd input output' to decompress") }
    } 
}


