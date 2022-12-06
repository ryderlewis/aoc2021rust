use std::collections::HashMap;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    println!("{}", find_pos(4));
}

fn part2() {
    println!("{}", find_pos(14));
}

fn find_pos(consecutive: usize) -> usize {
    let mut last_pos: HashMap<char, usize> = HashMap::new();
    let mut answer = 0;
    let mut good_count = 0;

    for (idx, c) in input().chars().enumerate() {
        match last_pos.get(&c) {
            Some(x) => {
                if idx - *x >= good_count + 1 {
                    good_count += 1;
                } else {
                    good_count = idx - *x;
                }
            },
            None => good_count += 1,
        }

        if good_count >= consecutive {
            answer = idx+1;
            break;
        }

        last_pos.insert(c, idx);
    }

    answer
}

fn input() -> &'static str {
    r###"
qnjjqgjqgglqqwrrvvcwwtjjzpjjfwwtwzwmzmrmttrvrpppbtbllhrrwtwzwjwzznfzfvvvngnrrhzrhhsmsrrsrrtqrrpzpnpfphfhllwlclfcfrrqffhvhsvswvswwngnzzcmmnrnzrnnwhwgwvggpcggnrntrrgmgdgmmgzmzssprrmddjndnsnrsrddnmdmqddrzznwznndfnndjjmgjmjmmnpmnmrmzmfzmmwhwzwrwqqqhshhncnvvhfvhfhzzvttrnrdnnwddptphpthhbgbgmbbvppnhphmmncnwcwqwppgddsqqjqppcrcncvnccjsjzztczttjzzwwhshnsnttrctchthnnswnnfzfhhsfhhdccgwcggbrblrlwlrlnntsscffpnnwffqgqdqhqhphhvtvmvhmmnssqbqfqfwwbfwbfwwzwgwgpgwwchcshslhssqsppqdqhhnssrzzvbbhjjqdjdgjgttvzzmvzvvnmvvfbbncbcqcpqpbbcjcncjnccjfjpjdjvdddbzzfrfwfpwpfwppvjvfjjhfhvvqttfhffscsvvbqbzqqjssbmsbshsrshsphhwhvwhwcwwnjjlzzfgfrgrwggfddpqpqqcmmtwtcwttgjjnfnnjvvmfmccvhchjcjmccmmwgghccnffjccqczqqbdbrdrsdsffmzzndnncbnbgbsshbhvbvsbvsbbvjbvbdvvqpqnpngpnggbbndbbdzdhzdhhjcjjmbbqgqbbfrrvzvcvffhggcbcjbbqtbbszbbvvcnvnccldddlrrtqrttpjpnjpnpznnqmnnrrmmsdmdbbffcwwllnffssspvvrjjwhwmmqhmmhjjtnjtjgttnrtnnsgsbssffstssjvjwwwwnjngjnnnlndndqdqqwdwcdcsccsgssgrrqwqqqhgqgfgffmnffzjjssqsfqqgddgcgtgrrvgrrdhdcdgdbdsswgwwpwggdfggnjnnvffzhhcqclqclcdlldbldlffdrrzttnhthdthhrthhgcgllzqlzzfhhmshssdvvtnnpngpprqrrrbnbssnhnnqdndsnshhzjjrnjnmjjwfffpbpnbbwddpwptwwpbpffvggmjmgghrrnqqhnhsnhsnnnlddmjddrdrhrchhgmgfmfdfcfpprsrzzzmhzhmhrmmrwmmpdmpmcclmlvvlffhlhhczccsvvptvpvwvdvqqdvqdddvnvsnswwspwpqqpddpvvsqsjjwnwrrjbjtttdhhhwbhbzzqrqfrrzlrzrqrbqrqzzfczfzrrqffnrnhrrcrtrvrffvflfsllcjcvvmqmlqqwjwgglhlvldltlqqdcddtssltsllsnndvdnddfqdffprrbssgccqmqrrnsrnnlzlhhqrhqhcqcjqcqpcqqmvqqgbgpbggnfggdpgpvpgvvsrrhqhshmsmwsmwssvnvtnttphpdpdspsvslshhlplmplmmljjvnnsrslrrvttqqprrptrptpnnmmflltntrrnjjcljcllwcllddbgbjbrrtwrtrftfctftsslsggggzqqhhrvvdqqwqbqtthdhrrpwpqpmmqcmqmllsmllhvfwbfvgzqgbhlfhqrqtzfpplgjtgngzrdfltnqlwsbhmwdfvrdjlgwftjvmdsgdgnswhrtmzgfqfsfnczjnmgqcfzvwlbgzsrpcbwwtrmbqtqhdmhmscqgjgpqdqcrvdvwplpdzsjqbvgpqcvfspqrcsjlhrqpjmdszjpqhmdwtddwqhbwsrlcjpzwsjjvbzcllqfwbhfvjqbqfbsrbgvgchdmgqjnvdrzlmrnlpzrljgjvtrdtqnzbnhpgpgjvwttmnfbpvrtmpstbmtwdwfzvznrwmspftgvrmdfwqltzzmlgrvvwjgdblnnbjzjfqpldsqbhrstnhrjqfcmzcrtqcpqmmfqzndgjwtcgnwrgdznzdgzbvmrlvrjvjgmfcrmrjbpjwqvhprbphnqsbpcpnflpgnpgggqpgrwghfpfvdljjqnvqgbvcpjbjlqghjppfhzfzczmcwnhrjzzrwlfqhvdwqrcbvprclnmqmqwdrhgtswwqhqtcjhndrmcrzdjnvwsstzplhcdzwzqbjjthsmjrmpfbqljlmnvmfddqflnhrfhchzgnlbcpvppnspsdtnqmfhrztznhltmqlrwzgczqmjggvnphwpvrwcnqhsdfglblvvdlqfnghtzgngtjnrzsjgwcglsrczlfqpmmzsrqwcclslzjpvpfgcwzwsfmwwswdsvnvtmwzmhwzvnnnldnjljbwlglpjvrggbmvhctsggtjgrmjrglnhtjzwhrvtmfmphgmpdmvlhzrhrtjcwswlnnjphsschrdvstmflzqsbqmqggjjdsdtjhcvjlzclbpvnbjgngnzvtdgszdtrwpnjmgwpcjpbsvrsmdjdjdbhqqvplbmfhmbgmtggvtltqjphlggscdbzncqhqqlmflmjbpnhbnqjcbfmggnftjjbzbrqwcgvfhsddrqgrvnztzzqzwjrjprmqctlttmgltdgrcvnbrbzrsbbqnsrslrswvmrqlctvsdfmcrdddcdwvfmwwcbqsdhmmgplzhnsfvhtctmvrzntvptpnbhhvjlphmtcctlhdvtvvqbmbczjqfvnqwzbgjzlgnqhvbjjnvrgfprvllltqdwvvmchrdqfczmmbmcjwtwgqbfjzgpcfzhhnwcghqltrlhlntjcrthpsbmcdzzqljdrdhfctwrzlvhdjvgmpscssmhdggrnhfbtfpqtsbqnqjfjwlbdfcpzfjtspnrjzshjcrrzwclddqqbplwghrcjhfqgblfrlrdhmdqmrzdnqthvvzlcjmqzqtgbcwcprpnjtbhsqmzlprhmssltfvvgqzzpnjtsmplhfpsnznjwrrgvmbbvjzzwctmwcwcwjwvmlpcmlmrfqbdjpclsqjnsnndzqfcghqsmhqcjwjjbbwmsttwnththhhgrlhrtrcjppvmjlqtqhpbhpsdhzcqpmqqbvwrlvjrnmlfvtntwghtphwzqmbmmpgvfrqsjwffdbcjjbwrrntrhnjwjfprzjcpnwvtwjcdhppspbdbwnzdbhvpqljbrfnpnbqdtqzdjlbvgbvvhcjrqwlqmhnhjfppllrsnsvnmttvrsdrqjfrdvhplwbhhnvglvgcvtzblfswzrrbwpjnslthjmgsdgwqrfjfplmbgsdcltzmdhrbpqggnjrcddltvdbrlhtzwfghbpnlmghwqdtpctmqbggfzflfzjgcbvfqtztsmgdvdlrrdrtmmjbtdtjvvjvlwwlwnzhlfqtrmdgvlflmbffnzlvtccnmcddrqfhgzlhngrfmsqdrnshshfhhtnwmnqmjtzggdfmpgcsrhjzhhvlppfnqmjsvtcwwjdvdrgrldbdnrqtvjjnddgmfdtqrncdlfznfpbqldztzzgpvwhvnqrrzdwsrdvcnslmwcdztvqzgbshpqrdbnmlfltpbsfrnbbcwvwvjnrdspwrwzpljqwwvqhbhhqqjnlrgwqvvsjrscbfqvhbwtjvwzmwzvftlwmtbmglvqvlnwrzvwclvzjcvgfstjpwdbtbhmnqzfgpqmthpchrrqfpqfflwhwrhmdzvgbzlmfgzvhrldprlgrvjcvjhslghqddfvqzgplmtfqjzhpctvtgrzjhwrjzbsqqrcztrstbvdjbtszrbvvzvwzfclsnrftcqtqdghdrnhglzptzwtqtgnfsqhmvnqqwtnwszgtvpnndbzcwgwhpcdlsdpptvlcqtmfhfjzbvfsvnjvqrrjqbggnjqfrrrvqmzscvlltfwzdlzqstlrrmqqmbmbtgbqjsvbncblddgzvlstmrpmrlfcqpvwthgbhlvfcfcwvslcjnztfwgdgwjfrwrzbbszmtvzchvstcfrgqllfrdccrjmtspshgqbzcldddqgjnpdsctmjphcwqvjtvmqlvvpzdgjdlpwdrjbshhrgjtglwnzlzsqngspbzgbwpgmfcv
    "###.trim()
}
