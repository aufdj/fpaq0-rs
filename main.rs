use std::fs::{File, metadata};
use std::io::prelude::*;
use std::env;

// Predictor -------------------------------------------------------
#[derive(Debug)]
struct Predictor {
    context:  usize,
    contexts: [[u32; 2]; 512],
}
impl Predictor {
    fn new(context: usize, contexts: [[u32; 2]; 512]) -> Predictor {
        Predictor {
            context, contexts,
        }
    }
    fn p(&mut self) -> u32 { 
        return 4096*(self.contexts[self.context][1]+1) / 
        (self.contexts[self.context][0]+self.contexts[self.context][1]+2); 
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
    fn new(high: u32, low: u32, predictor: Predictor, mut file: File, mut x: u32, compress: bool) -> Encoder {
        let mut encoder = Encoder {
            high, low, predictor, file, x, compress
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
    fn encode(&mut self, bit: usize) {
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * self.predictor.p();
        if bit == 1 {
            self.high = mid;
        } else {
            self.low = mid + 1;
        }
        self.predictor.update(&bit);

        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.file.write(&self.high.to_le_bytes()[3..4]).expect("Couldn't write to file.");
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
            let predictor = Predictor::new(1, [[0; 2]; 512]);
            let mut e = Encoder::new(0xFFFFFFFF, 0, predictor, file_out, 0, true);
            let mut byte = [0; 1];

            while file_in.read(&mut byte).unwrap() != 0 {
                e.encode(0);
                for i in (0..=7).rev() {
                    e.encode(((byte[0] >> i) & 1).into());
                } 
                 
            }   
            e.encode(1);
            e.flush(); 
            println!("Finished Compressing.");       
        }
        "d" => {
            let predictor = Predictor::new(1, [[0; 2]; 512]);
            let mut e = Encoder::new(0xFFFFFFFF, 0, predictor, file_in, 0, false);
            
            while e.decode() != 1 {   
                let mut c: usize = 1;
                while c < 256 {
                    c += c + e.decode();
                }
                c -= 256;
                file_out.write(&c.to_le_bytes()[0..1]).expect("Couldn't write to file.");
            }
            println!("Finished Decompressing.");   
        }
        _ => {
            println!("Enter 'c' to compress and 'd' to decompress");
        }
    } 
}
