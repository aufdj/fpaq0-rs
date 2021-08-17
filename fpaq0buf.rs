use std::fs::File;
use std::fs::metadata;
use std::io::{Read, Write, BufReader, BufWriter, BufRead};
use std::env;
use std::time::Instant;
use std::path::Path;

// Predictor -------------------------------------------------------
struct Predictor {
    context:  usize,
    contexts: [[u32; 2]; 512],
}
impl Predictor {
    fn new() -> Predictor {
        Predictor {
            context: 1, 
            contexts: [[0; 2]; 512],
        }
    }
    fn p(&mut self) -> u32 { 
        4096*(self.contexts[self.context][1]+1) / 
        (self.contexts[self.context][0]+self.contexts[self.context][1]+2) 
    } 
    fn update(&mut self, bit: &usize) {
        self.contexts[self.context][*bit]+=1;
        let bit_count: u32 = self.contexts[self.context][*bit]; 
        if bit_count > 65534 {
            self.contexts[self.context][0] >>= 1;
            self.contexts[self.context][1] >>= 1;   
        } 
        self.context += self.context + bit;
        if self.context >= 512 {
            self.context = 1;
        } 
    }
}
// -----------------------------------------------------------------

// Encoder ---------------------------------------------------------
#[allow(dead_code)]
struct Encoder {
    high:       u32,
    low:        u32,
    predictor:  Predictor,
    buf_in:     BufReader<File>,
    buf_out:    BufWriter<File>,
    x:          u32,
    compress:   bool,
}
impl Encoder {
    fn new(predictor: Predictor, buf_in: BufReader<File>, buf_out: BufWriter<File>, compress: bool) -> Encoder {
        let mut encoder = Encoder {
            high: 0xFFFFFFFF, 
            low: 0, 
            x: 0, 
            predictor, buf_in, buf_out, compress
        };
        if !compress {
            for _i in 0..4 {
                let mut byte = [0; 1];
                encoder.buf_in.read(&mut byte).unwrap();
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
            self.buf_out.write(&self.high.to_le_bytes()[3..4]).unwrap();
            if self.buf_out.buffer().len() >= self.buf_out.capacity() { self.buf_out.flush().unwrap(); }

            self.high = (self.high << 8) + 255;
            self.low <<= 8;  
        }
    }
    fn decode(&mut self) -> usize {
        let mut byte = [0; 1];
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * self.predictor.p();
        let mut bit: usize = 0;
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
            self.buf_in.read(&mut byte).unwrap(); 
            self.x = (self.x << 8) + byte[0] as u32; 

            if self.buf_in.buffer().len() <= 0 { 
                self.buf_in.consume(self.buf_in.capacity()); 
                self.buf_in.fill_buf().unwrap();
            }
        }
        bit
    }
    fn flush(&mut self) {
        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.buf_out.write(&self.high.to_le_bytes()[3..4]).unwrap();
            if self.buf_out.buffer().len() >= self.buf_out.capacity() { self.buf_out.flush().unwrap(); }

            self.high = (self.high << 8) + 255;
            self.low <<= 8; 
        }
        self.buf_out.write(&self.high.to_le_bytes()[3..4]).unwrap();
        self.buf_out.flush().unwrap();
    }
}
// -----------------------------------------------------------------

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    // main() buffers
    let file_in = File::open(&args[2]).unwrap();
    let file_out = File::create(&args[3]).unwrap();
    let mut buf_in = BufReader::with_capacity(4096, file_in);
    let mut buf_out = BufWriter::with_capacity(4096, file_out);
    // Encoder buffers
    let e_file_in = File::open(&args[2]).unwrap();
    let e_file_out = File::create(&args[3]).unwrap();
    let e_buf_in = BufReader::with_capacity(4096, e_file_in);
    let e_buf_out = BufWriter::with_capacity(4096, e_file_out);
    match (&args[1]).as_str() {
        "c" => {  
            let file_in_size = metadata(Path::new(&args[2])).unwrap().len();
            let predictor = Predictor::new();
            let mut e = Encoder::new(predictor, e_buf_in, e_buf_out, true);
            let mut byte = [0; 1];

            while buf_in.read(&mut byte).unwrap() != 0 {
                e.encode(0);
                for i in (0..=7).rev() {
                    e.encode(((byte[0] >> i) & 1).into());
                } 

                if buf_in.buffer().len() <= 0 { 
                    buf_in.consume(buf_in.capacity()); 
                    buf_in.fill_buf().unwrap(); 
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
            let predictor = Predictor::new();
            let mut e = Encoder::new(predictor, e_buf_in, e_buf_out, false);
            
            while e.decode() != 1 {   
                let mut c: usize = 1;
                while c < 256 {
                    c += c + e.decode();
                }
                c -= 256;
                buf_out.write(&c.to_le_bytes()[0..1]).unwrap();
                if buf_out.buffer().len() >= buf_out.capacity() { buf_out.flush().unwrap(); }
            }
            buf_out.flush().unwrap();

            let file_out_size = metadata(Path::new(&args[3])).unwrap().len();
            println!("Finished Decompressing.");  
            println!("{} bytes -> {} bytes in {:.2?}", file_in_size, file_out_size, start_time.elapsed());   
        }
        _ => {
        println!("Enter 'c' to compress and 'd' to decompress")
        }
    } 
}

