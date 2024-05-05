#![allow(dead_code,unused)]

pub struct RC4 {
    i: u8,
    j: u8,
    state: [u8; 256]
}

impl RC4 {
    pub fn new(key: &[u8]) -> RC4 {
        assert!(
            key.len() >= 1 && key.len() <= 256,
            "key's length must be between 1 and 256"
        );

        let mut rc4 = RC4 {
            i: 0,
            j: 0,
            state: [0; 256]
        };
        
        // key-scheduling algorithm
        rc4.ksa(key);
        rc4
    }

    fn ksa(&mut self, key: &[u8]) {
        // initialize state
        for (i, s) in self.state.iter_mut().enumerate() {
            *s = i as u8;
        }

        // shuffle state
        let mut j = 0u8;
        for i in 0..256 {
            j = j.wrapping_add(self.state[i]).wrapping_add(key[i % key.len()]);
            self.state.swap(i, j as usize);
        }
    }

    pub fn prga(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        self.state.swap(self.i as usize, self.j as usize);

        let out  = self.state[
            self.state[self.i as usize].wrapping_add(self.state[self.j as usize]) as usize
        ];
        out
    }
}