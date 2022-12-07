use bit_vec::BitVec;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let packets = Packet::parse();
    println!("{}", packets.iter().map(|p| p.version_sum()).sum::<u64>());
}

fn part2() {
}

const TYPE_ID_LITERAL: u8 = 4;

const LENGTH_TYPE_BITS: u8 = 0;
const LENGTH_TYPE_PACKETS: u8 = 1;

#[derive(Debug)]
struct Operator {
    length_type: u8,
    length: u16,
    sub_packets: Vec<Packet>,
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    literal: Option<i64>,
    operator: Option<Operator>,
}

impl Packet {
    fn version_sum(&self) -> u64 {
       self.version as u64 + match &self.operator {
           None => 0,
           Some(operator) => operator.sub_packets.iter().map(|p| p.version_sum()).sum(),
       }
    }

    fn parse() -> Vec<Self> {
        let i = input().trim();
        let b: Vec<u8> = (0..i.len()).step_by(2).map(|x| u8::from_str_radix(&i[x..x+2], 16).unwrap()).collect();
        let bits = BitVec::from_bytes(&b);
        let mut offset = 0;

        let mut v = Vec::new();

        while offset < bits.len() - 7 {
            let p = Self::make_packet(&bits, &mut offset);
            v.push(p);
        }

        v
    }

    fn make_packet(bits: &BitVec<u32>, offset: &mut usize) -> Self {
        let version = Self::extract_val(&bits, 3, offset) as u8;
        let type_id = Self::extract_val(&bits, 3, offset) as u8;
        let literal = match type_id {
            TYPE_ID_LITERAL => Some(Self::extract_literal(&bits, offset)),
            _ => None,
        };
        let operator = match type_id {
            TYPE_ID_LITERAL => None,
            _ => Some(Self::extract_operator(&bits, offset)),
        };

        Self { version, type_id, literal, operator }
    }

    fn extract_val(bits: &BitVec<u32>, bytes: usize, offset: &mut usize) -> usize {
        let mut t = 0;

        for i in 0..bytes {
            t *= 2;
            if bits.get(*offset + i).expect("ran off the end") {
                t += 1;
            }
        }
        *offset += bytes;

        t
    }

    fn extract_literal(bits: &BitVec<u32>, offset: &mut usize) -> i64 {
        let mut literal: i64 = 0;
        let mut cont: u8 = 1;

        while cont == 1 {
            cont = Self::extract_val(bits, 1, offset) as u8;
            literal *= 16;
            literal += Self::extract_val(bits, 4, offset) as i64;
        }

        literal
    }

    fn extract_operator(bits: &BitVec<u32>, offset: &mut usize) -> Operator {
        let length_type = Self::extract_val(bits, 1, offset) as u8;
        let mut length = Self::extract_val(bits, if length_type == LENGTH_TYPE_BITS { 15 } else { 11 }, offset) as u16;
        let mut sub_packets = Vec::new();

        while length > 0 {
            let start = *offset;
            sub_packets.push(Self::make_packet(bits, offset));

            length -= match length_type {
                LENGTH_TYPE_BITS => (*offset - start) as u16,
                _ => 1,
            }
        }

        Operator { length_type, length, sub_packets }
    }
}

fn input() -> &'static str {
    r###"
C20D718021600ACDC372CD8DE7A057252A49C940239D68978F7970194EA7CCB310088760088803304A0AC1B100721EC298D3307440041CD8B8005D12DFD27CBEEF27D94A4E9B033006A45FE71D665ACC0259C689B1F99679F717003225900465800804E39CE38CE161007E52F1AEF5EE6EC33600BCC29CFFA3D8291006A92CA7E00B4A8F497E16A675EFB6B0058F2D0BD7AE1371DA34E730F66009443C00A566BFDBE643135FEDF321D000C6269EA66545899739ADEAF0EB6C3A200B6F40179DE31CB7B277392FA1C0A95F6E3983A100993801B800021B0722243D00042E0DC7383D332443004E463295176801F29EDDAA853DBB5508802859F2E9D2A9308924F9F31700AA4F39F720C733A669EC7356AC7D8E85C95E123799D4C44C0109C0AF00427E3CC678873F1E633C4020085E60D340109E3196023006040188C910A3A80021B1763FC620004321B4138E52D75A20096E4718D3E50016B19E0BA802325E858762D1802B28AD401A9880310E61041400043E2AC7E8A4800434DB24A384A4019401C92C154B43595B830002BC497ED9CC27CE686A6A43925B8A9CFFE3A9616E5793447004A4BBB749841500B26C5E6E306899C5B4C70924B77EF254B48688041CD004A726ED3FAECBDB2295AEBD984E08E0065C101812E006380126005A80124048CB010D4C03DC900E16A007200B98E00580091EE004B006902004B00410000AF00015933223100688010985116A311803D05E3CC4B300660BC7283C00081CF26491049F3D690E9802739661E00D400010A8B91F2118803310A2F43396699D533005E37E8023311A4BB9961524A4E2C027EC8C6F5952C2528B333FA4AD386C0A56F39C7DB77200C92801019E799E7B96EC6F8B7558C014977BD00480010D89D106240803518E31C4230052C01786F272FF354C8D4D437DF52BC2C300567066550A2A900427E0084C254739FB8E080111E0
    "###.trim()
}
