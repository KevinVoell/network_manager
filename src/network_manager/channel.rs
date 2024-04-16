pub struct Channel {
    pub channel: u32,
    pub frequency: u32,
}

impl Channel {
    pub fn a_frequencies() -> [Channel; 46] {
        [
            Channel {
                channel: 7,
                frequency: 5035,
            },
            Channel {
                channel: 8,
                frequency: 5040,
            },
            Channel {
                channel: 9,
                frequency: 5045,
            },
            Channel {
                channel: 11,
                frequency: 5055,
            },
            Channel {
                channel: 12,
                frequency: 5060,
            },
            Channel {
                channel: 16,
                frequency: 5080,
            },
            Channel {
                channel: 34,
                frequency: 5170,
            },
            Channel {
                channel: 36,
                frequency: 5180,
            },
            Channel {
                channel: 38,
                frequency: 5190,
            },
            Channel {
                channel: 40,
                frequency: 5200,
            },
            Channel {
                channel: 42,
                frequency: 5210,
            },
            Channel {
                channel: 44,
                frequency: 5220,
            },
            Channel {
                channel: 46,
                frequency: 5230,
            },
            Channel {
                channel: 48,
                frequency: 5240,
            },
            Channel {
                channel: 50,
                frequency: 5250,
            },
            Channel {
                channel: 52,
                frequency: 5260,
            },
            Channel {
                channel: 56,
                frequency: 5280,
            },
            Channel {
                channel: 58,
                frequency: 5290,
            },
            Channel {
                channel: 60,
                frequency: 5300,
            },
            Channel {
                channel: 64,
                frequency: 5320,
            },
            Channel {
                channel: 100,
                frequency: 5500,
            },
            Channel {
                channel: 104,
                frequency: 5520,
            },
            Channel {
                channel: 108,
                frequency: 5540,
            },
            Channel {
                channel: 112,
                frequency: 5560,
            },
            Channel {
                channel: 116,
                frequency: 5580,
            },
            Channel {
                channel: 120,
                frequency: 5600,
            },
            Channel {
                channel: 124,
                frequency: 5620,
            },
            Channel {
                channel: 128,
                frequency: 5640,
            },
            Channel {
                channel: 132,
                frequency: 5660,
            },
            Channel {
                channel: 136,
                frequency: 5680,
            },
            Channel {
                channel: 140,
                frequency: 5700,
            },
            Channel {
                channel: 149,
                frequency: 5745,
            },
            Channel {
                channel: 152,
                frequency: 5760,
            },
            Channel {
                channel: 153,
                frequency: 5765,
            },
            Channel {
                channel: 157,
                frequency: 5785,
            },
            Channel {
                channel: 160,
                frequency: 5800,
            },
            Channel {
                channel: 161,
                frequency: 5805,
            },
            Channel {
                channel: 165,
                frequency: 5825,
            },
            Channel {
                channel: 183,
                frequency: 4915,
            },
            Channel {
                channel: 184,
                frequency: 4920,
            },
            Channel {
                channel: 185,
                frequency: 4925,
            },
            Channel {
                channel: 187,
                frequency: 4935,
            },
            Channel {
                channel: 188,
                frequency: 4945,
            },
            Channel {
                channel: 192,
                frequency: 4960,
            },
            Channel {
                channel: 196,
                frequency: 4980,
            },
            Channel {
                channel: 0,
                frequency: 0,
            },
        ]
    }

    pub fn gb_frequencies() -> [Channel; 15] {
        [
            Channel {
                channel: 1,
                frequency: 2412,
            },
            Channel {
                channel: 2,
                frequency: 2417,
            },
            Channel {
                channel: 3,
                frequency: 2422,
            },
            Channel {
                channel: 4,
                frequency: 2427,
            },
            Channel {
                channel: 5,
                frequency: 2432,
            },
            Channel {
                channel: 6,
                frequency: 2437,
            },
            Channel {
                channel: 7,
                frequency: 2442,
            },
            Channel {
                channel: 8,
                frequency: 2447,
            },
            Channel {
                channel: 9,
                frequency: 2452,
            },
            Channel {
                channel: 10,
                frequency: 2457,
            },
            Channel {
                channel: 11,
                frequency: 2462,
            },
            Channel {
                channel: 12,
                frequency: 2467,
            },
            Channel {
                channel: 13,
                frequency: 2472,
            },
            Channel {
                channel: 14,
                frequency: 2484,
            },
            Channel {
                channel: 0,
                frequency: 0,
            },
        ]
    }
}
