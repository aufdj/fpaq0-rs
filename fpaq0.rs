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

// Predictor --------------------------------------------------------
struct Predictor {
    cxt:   usize,
    cxts:  [[u32; 2]; 512],
}
impl Predictor {
    fn new() -> Predictor {
        Predictor {
            cxt:   1, 
            cxts:  [[0; 2]; 512],
        }
    }
    fn p(&mut self) -> u32 { 
        4096 * (self.cxts[self.cxt][1] + 1) / 
        (self.cxts[self.cxt][0] + self.cxts[self.cxt][1] + 2) 
    } 
    fn update(&mut self, bit: usize) {
        self.cxts[self.cxt][bit] += 1;
        let bit_count: u32 = self.cxts[self.cxt][bit]; 
        if bit_count > 65534 {
            self.cxts[self.cxt][0] >>= 1;
            self.cxts[self.cxt][1] >>= 1;   
        } 
        self.cxt += self.cxt + bit;
        if self.cxt >= 512 {
            self.cxt = 1;
        } 
    }
}
// ------------------------------------------------------------------

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
    fn encode(&mut self, bit: usize) {
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * self.predictor.p();
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
    fn decode(&mut self) -> usize {
        let mut byte = [0; 1];
        let mut bit: usize = 0;
        let mid: u32 = self.low + ((self.high - self.low) >> 12) * self.predictor.p();
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
                enc.encode(0);
                for i in (0..=7).rev() {
                    enc.encode(((byte[0] >> i) & 1).into());
                } 
            }   
            enc.encode(1);
            enc.flush(); 
            println!("Finished Compressing.");     
        }
        "d" => {
            let mut dec = Decoder::new(file_in);
            
            while dec.decode() != 1 {   
                let mut dec_byte: usize = 1;
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


