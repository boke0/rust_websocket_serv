extern crate rand;

use std::mem;
use rand::{thread_rng, prelude::*};

pub struct DataFrame {
    pub base: Vec<u8>,
    pub opcode: u8,
    pub data: Vec<u8>
}
impl DataFrame {
    pub fn new(base: Vec<u8>, opcode: u8, data: Vec<u8>) -> DataFrame {
        DataFrame {
            base,
            opcode,
            data
        }
    }
    pub fn from(buf: &[u8]) -> DataFrame {
        let buf = buf.to_vec();
        let opcode = buf[0] & 0b00001111;
        let masked = buf[1] & 0b10000000 == 128;
        let mut len: usize = (buf[1] & 0b01111111) as usize;
        let mut i=2;
        let key: [u8; 4] = if masked {
            i+=4;
            let mut arr=[0; 4];
            arr.copy_from_slice(&buf[2..6]);
            arr
        } else {
            [0; 4]
        };
        if len == 126 {
            unsafe {
                let mut arr=[0; 2];
                arr.copy_from_slice(&buf[i..(i+2)]);
                len = mem::transmute::<[u8; 2], u16>(arr) as usize;
            }
            i+=2;
        } else if len == 127 {
            unsafe {
                let mut arr=[0; 8];
                arr.copy_from_slice(&buf[i..(i+8)]);
                len = mem::transmute::<[u8; 8], u64>(arr) as usize;
            }
            i+=8;
        }
        let mut data: Vec<u8> = buf[i..(len+i)].to_vec();
        if masked {
            for j in 0..len {
                data[j] ^= key[j%4];
            }
        }
        DataFrame::new(buf,opcode,data)
    }
    pub fn as_slice(&self) -> &[u8] {
        return self.data.as_slice();
    }
    pub fn as_raw_data(&mut self) -> &[u8] {
        self.base=Vec::new();
        self.base.push(0b10000000+self.opcode);
        if self.data.len() < 126 {
            self.base.push(0b10000000+(self.base.len() as u8));
        } else if self.data.len() < 65536 {
            let l = self.base.len() as u16;
            self.base.push(0b11111110);
            unsafe {
                let mut m = mem::transmute::<u16, [u8; 2]>(l).to_vec();
                self.base.append(&mut m);
            }
        } else {
            let l = self.base.len() as u64;
            self.base.push(0b11111111);
            unsafe {
                let mut m = mem::transmute::<u64, [u8; 8]>(l).to_vec();
                self.base.append(&mut m);
            }
        }
        let mut rng = thread_rng();
        let mut mask: Vec<u8> = vec![0; 4].iter().map(|x| rng.gen_range(0, 255)).rev().collect();
        self.base.append(&mut mask);
        for (i,byte) in self.data.iter().enumerate() {
            self.base.push(byte ^ mask[i%4]);
        }
        return self.base.as_slice();
    }
}
