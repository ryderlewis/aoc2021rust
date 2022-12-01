use std::collections::HashSet;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let g = Grid::from_input(input());
    println!("{}", g.risk_level());
}

fn part2() {
    let g = Grid::from_input(input());
    println!("{}", g.basins());
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    val: u8,
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Point>>,
}

impl Grid {
    fn from_input(input: &str) -> Grid {
        let mut grid = Grid{
            rows: Vec::new(),
        };

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();

            for (x, c) in line.trim().chars().enumerate() {
                row.push(Point{
                    val: c as u8 - '0' as u8,
                    x: x as u8,
                    y: y as u8,
                })
            }

            grid.rows.push(row);
        }

        grid
    }

    fn low_points(&self) -> Vec<&Point> {
        let height = self.rows.len();
        let width = self.rows[0].len();
        let mut v: Vec<&Point> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if self.neighbors(&self.rows[y][x]).iter().all(|p| p.val > self.rows[y][x].val) {
                    v.push(&self.rows[y][x]);
                }
            }
        }

        v
    }

    fn risk_level(&self) -> i32 {
        let mut risk = 0;
        for p in self.low_points() {
            risk += 1 + p.val as i32;
        }
        risk
    }

    fn neighbors(&self, p: &Point) -> Vec<&Point> {
        let height = self.rows.len();
        let width = self.rows[0].len();
        let mut v: Vec<&Point> = Vec::new();

        // try visiting all neighbors
        let x = p.x as usize;
        let y = p.y as usize;

        if y > 0 {
            v.push(&self.rows[y-1][x]);
        }
        if y < height-1 {
            v.push(&self.rows[y+1][x]);
        }
        if x > 0 {
            v.push(&self.rows[y][x-1]);
        }
        if x < width-1 {
            v.push(&self.rows[y][x+1]);
        }

        v
    }

    fn basins(&self) -> i32 {
        let mut basin_sizes: Vec<i32> = Vec::new();
        let mut visited: HashSet<Point> = HashSet::new();

        for p in self.low_points() {
            let mut basin_size = 0;
            let mut to_visit: Vec<&Point> = vec![p];
            visited.insert(*p);

            while to_visit.len() > 0 {
                let v = to_visit.pop().unwrap();
                basin_size += 1;

                for n in self.neighbors(v) {
                    if !visited.contains(n) {
                        visited.insert(*n);
                        if n.val < 9 {
                            to_visit.push(n);
                        }
                    }
                }
            }

            basin_sizes.push(basin_size);
        }

        basin_sizes.sort();
        basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()
    }
}

fn input() -> &'static str {
    r###"
5434456789899876598943212349988734679896554212468998765634987654567895454569876745567898765987654678
4421345997789997987894103458976521398765432101457897664529876543988987353478965434456799974398766789
3210237896698999876789214567895320129878543212346987543212989432499976212389876521234589867219878999
8732456934567987655678925679976543234999657323657898952101296521276895423467987420145678954323989987
7545678923979996544367899899997665379898775454767899843212398730145789439999876521266789765439896895
8656989019899985431257912978989876456789896569898998754324579821234679598787987632348999976598765434
9767893198769876542349943969878987567891987678959799867465679932348789987656899973467898899989754325
9899954599356987653567899858767898998910198789345689879876898655459892199767898765989987678978965434
0949765983245898974567986745656789429963239891256998989987998798767891019898999899899876569867896645
1239899874123798765678965432345678998754356999969897896998919999898942129919899989798765432956789896
2456999763235679976789876521012899569896567987898776645789423998989763298901698978679654321987899987
3567998654345789697894997863246789345989679976897655434678999887678954397992497761598767210198989999
4568998767658994598932987654959896559878998965689543212789989756545895986789986550349989321239878934
5679109978767893459891098799898998698766987654599652101299876542136789975498765431256999932349867995
6789299989878932398789129988797789797654598543688943212389987651012898996329765432367899893998656789
7898989899989321239679939878646678976543987654567894345678999954123456789319876783456987679899543454
8987674789996510398567898965434567895432398769878997656789298893256567892101989865597976545788932123
9876543668897932987678987654325478999421239878989989897892196799347878943212399876789876434567893015
6976542456789893498789798843412345678910129989095678998943985978958989954323498989899854323479992134
5988764567898789999895699752103456989321298792134578949959874567899699895434597898998769436589789645
3299985998976679891934987653212367895433397654236689939898765878934598789545986567899898998995679756
4399876789765456790949998654354567976564579864346789795789876989423987678959876456789987999754598967
5989987898874345689898998765667678987678679875467997674898997893219876579898965345699876899643457999
9874399987654234889767249879799899398788799986578998543456789979101985498787976458798765678952348688
9965456798982124678954123998989921299899899897789019432349892356919876987656897569987654569861234567
9878767899971012567893299897678930987943945798899929656478901237895988995434789978999543498990195688
4989879998753223456789987684567899876542134689967898767567892348954599986523678989898921987989989789
3292998987654345567899876543454979987698745798756789888678943456795678987434567898797990196578878990
5101997699865456878999987952123567898987656987645999999799764789989799398646678987686789987458767891
3219876439876767989789854321018798909998789398739898998999875699978989109798789999575679654345656789
4923987421988878994698765432134689212999899219898787887689987898967878919899899998434598743212345699
9894996210199989543569876987656789329896998998997645986567898987657567894902999896565679754103456999
8789765434567896532979987899769895498785987987987432345457899996543456793219899789678998765214667898
7679987545698987649899998959878999597654596576796541012345678987532345789398798698989439886356778987
6567899756789498998787899543999788999869989435987782134456989997431234678987654587892129987487899876
1456789987891349987656987682101677896998764324598943245667892986520345679976743476789098987598989988
2367891098942998876435699899323456795109875455799876368998901975421234567895432125678987798679769899
3458932989959897652123799998754789894298989569899965467999219876532345689984321014789976689789543767
6567899876798789943254889879875678989997898698999876567894334998765456796975432423899865579899632156
7678999865987656798765679765996899569876679987987987678999549769976789895496676534998784465978943234
8789489954698747899898789894597943456965569876756798989998998756987899932398987545797653234567894965
9896567893497656789989899989698952349893457954348899999867897645698998651239998676789543123456789896
4989678912399788997678969878999876599789569943235978987656798434569679962345999987898654234567898797
3479989101989899234579953967899997987679979754101567896546795320197599843469899998939764346678999689
2567893219876920123498842545798798996589898765243489998657896431989987654598788999323976487989898567
1457954998985431245987721234989659987456799876756678998798986549879999867998687896534987578998766456
0345699877896545679876510129878943298589890987987899989899598698768899979896556789649898789789654233
1258789565987656798765431298967892129678992999398987667965498797658789998787445679998769897698763101
3879895434598767899876532987656789098789979893219898548994349988546698987674324567898756989549874222
4989999323569899987989749876545699989898767689456798656789239975234567896532014687996545678934965434
5699888912479959876599898765434598878989954597997959897891098754123458976432123456789436789029896945
6789767893569542987456999876523987654567893456789542998943987653012369897643234797897647892198789896
7996556789698959898968998765219876543478932345698951019995698432123456797654365698998758943987668689
8954345678987998759979897654327989657899321234567892129889997643264567899895776789019869959876547567
9543287567896789543499789876456798798999872345678963498769898965459678999976798997998999898954323458
8762123456795678932987678976578979899698765456789954989656789878678989689987899756987889767895444767
6571012387894239321998489987678967989459889578997899876545995989789996577998987645986563456976655678
8432123456943145939876569998989345678967998789756798765634894399898895456879995429876432356897779899
7543234579432019899998678989492235678979679897645987654329795210956789327467896539994321237998989931
8654345678973298799659989976320136789989536965534598965418689632345893214357987698789435398949995432
9765566899954989578945698765431246789997645984323459876324596543456789301236798797698945459539876543
9878987899869876457936789987543657894987659876775767988765797786569895212545679987567896579321997654
9989398998998765346897894697654769923498767987899878999896789897678954323598789498678987678932398895
9899459987679976456789923498767878939999989998910989999987894998789765499989892349989698989993979976
8798967943568989769893212459878989998791299879322398989998943769999876987878991234596439498789765987
7667899652456899878976493967989399876689399965443987878999542356789989776456789949697321334599654598
6545678943597956989987989898996598985545989987559876769897669467894597654345699898989210123498963239
9436789894989439997899876789987987694234567897698765458789798978932987543248998767678933234987892134
8997898789878998876799985689998975543123458998789987345678997989321997654367999856577894549876789045
6889997679767897545989654567899984321012567899898753234789986899439898765456897845456789698765692156
5679986545656989439876543678920995443227679998999864565699875778999789996567986434345698799754589267
4798765434745778924987762399439876654535789587998765699789564567987678987698954323236789988643678979
3987654321434567995699843489597987765678996476899876789895423459876569299899875210127999876512889989
2398776410123456789987654578976598978789875345789987899964312346975489101999986321239899974323690198
1459898531234587898798767678965439989999954234598798979975423769876594312398765432349798765434569297
2699876542346678997649878789432123499999876345987659567897569878987694324569898543498659896565678956
9989987843656789989432989896543034578989985456798741456898689989299789455689987654987545987876789245
8978998654567897678921299987652145989979876787899432346789799892109896596798998965696534698987992123
7767899765678976567932349998543267898765998998989753456899896789512987989987899878987623499698993434
6456789878789995478945678997654379987654989999978999567899945695423499879876789989998834988549889645
2348995999998989599656789998765459998769877898765688979998956789534598766545679199876549876534678956
1256893212987876989767899869876567899898756987654567999986899899976987655326989298987698765423459867
2367999323986765878998987656987878998999967898323457899875798999897898543201399987598789879210678978
3478998939765654569999896545698989667989899999212345899964567898799939954912568898449899984321289989
4989997898754343456789765434349893459879798987463456789643456789688929899893456789234989965434567896
5678976798773212345899984321256789599965667896554569896532567894567998789789598894345679876565778965
6899995987654301236789876532367899987654356987789678997321013789679876545699679965956789987876789954
8987989998765312447897987688456789998543234598898789398432123678989999433488989989897999998998899895
9766767989876493558965499786567896987652105679999891298743454569999878521367893198769899899899966789
7654555679997989967894329897898945698543212589899999987655677678998765310456789976556789765732455894
6553334899989879898965469989999334987654323456789678998766788889429896321367899896345678954621234932
5432124578978956789876598976989219999768454579894599659877899995434975433698998765234579543210129891
6543023569864345699997986745878998789879767699923589547998998989565986844589987654357897654521656790
9632123479983234567898965434567899667989888789013478936669987879976987865679999866468999865632347891
8743244569874347689999874323456965456795999993123569425459986567899898979789899998567899876774458942
8654456798765456790198765201597896568954987654256989312398765458968799989897789219778945988765667899
9987587899976569891239877332387899878943298965345993201239954349355689997956678909899434199876789978
1098788998988789932545976545456956989652129876556789713498765693234598766434567899974321019998893459
3129999987699896543489988758969139898761034987679899654569876789345679654323489998765432198779902345
4534567898799987654578999767878956789432156798789998775678987895468798765434591249976545698654323456
    "###.trim()
}