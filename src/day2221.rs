use std::collections::HashMap;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut monkeys = Monkeys::parse();
    println!("{}", monkeys.calculate("root".to_string()));
}

fn part2() {
}

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
enum Shout {
    Num(i64),
    Op(String, Operation, String),
}

#[derive(Debug)]
struct Monkey {
    name: String,
    shout: Shout,
}

impl Monkey {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() == 2 {
            let answer = parts[1].parse::<i64>().unwrap();

            Self {
                name: parts[0][..4].to_string(),
                shout: Shout::Num(answer),
            }
        } else {
            let operation = match parts[2] {
                "+" => Operation::Plus,
                "-" => Operation::Minus,
                "*" => Operation::Times,
                "/" => Operation::Divide,
                other => panic!("{other} is not a valid operation"),
            };

            Self {
                name: parts[0][..4].to_string(),
                shout: Shout::Op(parts[1].to_string(), operation, parts[3].to_string()),
            }
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: HashMap<String, Monkey>,
    answers: HashMap<String, i64>,
}

impl Monkeys {
    fn parse() -> Self {
        let mut m = HashMap::new();

        for line in input().lines() {
            let monkey = Monkey::parse(line);
            m.insert(monkey.name.clone(), monkey);
        }

        Self {
            monkeys: m,
            answers: HashMap::new(),
        }
    }

    fn calculate(&mut self, name: String) -> i64 {
        if let Some(answer) = self.answers.get(&name) {
            return *answer;
        }

        let monkey = self.monkeys.get(&name).unwrap();
        let val = match &monkey.shout {
            Shout::Num(x) => *x,
            Shout::Op(name_a, op, name_b) => {
                let name_a = name_a.clone();
                let name_b = name_b.clone();
                let op = op.clone();

                let val_a = self.calculate(name_a);
                let val_b = self.calculate(name_b);

                match op {
                    Operation::Plus => val_a + val_b,
                    Operation::Minus => val_a - val_b,
                    Operation::Times => val_a * val_b,
                    Operation::Divide => val_a / val_b,
                }
            },
        };

        self.answers.insert(name, val);
        val
    }
}

fn input_test() -> &'static str {
    r###"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
    "###.trim()
}

fn input() -> &'static str {
    r###"
scsc: qzgn * dvqf
vbgg: wtsg * hghv
dhgc: qqtp * lfpt
zzjt: 2
djvt: 4
cwdj: 10
sqmp: 5
jmbp: 10
snwp: 2
pgth: 3
jrmp: trtl + dtgw
zpmz: jhnv + rjzt
lldv: 5
qczl: 2
hthd: 7
svpv: ndcp / sfjz
crch: 20
tbmd: fllg * fccb
czbt: 3
jzdf: 2
lhhr: 4
clrz: dnpw * mzdj
qlcc: 9
gdld: 7
plsb: jpfm * dnfd
cpdl: vmnt * mdpb
vggz: 3
jcgs: 11
fzmw: 2
vqcw: 3
qqtp: 5
qwrs: 8
cbmm: sgll + vhnf
fpbj: bqwd * clrv
qfrj: 2
zlpj: pfrc * lmvf
znqj: 2
qfvv: swvt / tltw
bwmz: 9
pvjd: 5
fgvc: 3
qwfb: 3
srfh: vbvz * gdmw
jbzz: jfnm + llmt
fggt: 2
mhlr: ppsz + prwp
sfvd: thvn / lzbm
jlcj: vjdp * fpjm
bfgl: 3
nqgp: 2
twlb: 1
wfrs: wpnm / dtjl
dnqt: vnlq + lndt
mdfp: 5
mhch: ltrp - djrh
rhvw: pbst + twzw
wfcm: 13
bjth: mmtm * zgqr
jmld: mzsc + hhhv
nbpf: 5
cwfc: mwvl + jhdv
rvpz: 4
cvnq: 3
hjwl: rrch * zlpp
qzvv: cmrs * czcp
rcsd: jpcg * znrr
tcsr: 2
ztbt: wthb * pnqh
pndq: 9
wmpn: ztcb + fssj
qvlj: gbpn * rgnh
brzm: 1
hrnt: 11
rnlt: twnz + qrfz
rfcn: bcqr + wzcg
qphg: trnr + pfjr
ctgs: wngv * tpcr
hfwz: 2
fwhw: gfcq * jgws
qjmr: 3
rrlr: 1
btbj: wmdz + bgpl
pgjs: shzj + cfqb
fzdp: dtvj * jchm
sqfv: lhtl / slsn
mrhq: 2
lfgj: dtqz + mgsf
zgls: 2
hzld: ltbz + hthd
gpdl: jrsd * gbln
tcnv: 2
lbbj: 5
tgbp: lmvl * qwdv
nthm: tcsr * wjzm
fmsh: vtqs * bjwg
trwj: wbbg + wtzt
tqcb: 13
twtq: 4
cpzw: cpbm * gszz
dfrn: zfjq * fdtt
crst: qhrt * jglz
bfgf: 4
fpfn: jvcm * qdnz
zlpp: 4
dhqv: cldg + mjpq
ldnf: gcpg + vmsd
rpnw: dbph + rqpt
trtl: vtzd / qfdb
sdcw: 1
hdfd: nzbl * vtgq
lrmt: 3
wfzt: rnlt + plsb
ljpz: gmgt / rzhm
fmnf: crnw + vtnv
wgdv: cgbc - ttjn
gffz: lsfd + bqnl
qqqr: 4
jqcw: 10
mqgb: gjcv + llhr
qhbs: fvcm * hbnz
hvsg: pjst * jrsb
mljv: 4
cptz: nwjq + gjzj
hjbp: 3
qjzr: mwch + hqms
sgsg: 1
ndcp: djsp + jpwl
wzsf: 3
jsng: 1
fgvg: 5
zggm: 3
jgcd: 2
jvpb: mhfd * npdn
rvzl: 2
rmqr: smwb * tspc
cbnd: dpbl * sftv
mwch: 3
mzpm: 3
hvtc: wvdv + nmlw
ftvq: 4
qllp: hghz + qvlj
fhjj: 2
jpfm: 7
sljg: 4
wmdn: 7
pnpz: fcwp + wwhf
mgsf: qqlt * cqrm
hflj: mslf * lpmd
cjnl: fcsq * vzrf
qrfn: 2
gzfz: 4
ngvm: 3
rmmg: 2
fbpn: 2
pdqn: lfgj * fmbm
fbnr: 3
dmrm: 3
bgvr: qllp + qlsz
bjwg: 6
dlrz: cpdl + frqj
mpvz: lptz * twtq
qhpb: hmpq * wqcn
ldnm: 3
dtjl: 2
vmjc: 2
mzwq: 2
rcvb: jcvz + pcdm
ncfz: 3
crql: 11
hwsm: 2
tnrp: 3
dcww: 2
cpmw: 12
nqst: wnjn * nffc
dhvm: 9
hhvn: 2
llhr: 5
dnhz: hdqw * hzld
mttg: 6
svmv: 11
pfrc: 4
vmqw: 15
qrhb: bhln / hrhc
bmgz: qrbb * grcp
jglz: 11
dgdm: 3
dtrb: qldv * mhhc
pvch: 2
tpct: 4
ztrc: 2
jrsn: rvjh + plfl
mmbq: htdj + brcj
qwdv: vcjp - fgnz
mdpl: 20
dlzv: nmjs + ptll
fllg: wmfr + qbjm
ccwz: sbdp * dwnn
crbz: 7
glqb: mvbs * pqvp
vmqv: 14
mcjv: 5
gjtt: vqrc + qdgw
fvcm: 4
gblj: ljlt * nftm
pscp: pnwn - rfhj
rzft: 9
tbhm: qtjr * fvsg
lfpt: 5
gvfz: czth / pmfg
vnjj: 13
jgdd: 2
qccj: rcvb + gqwh
ncvm: 2
dfdv: 8
cczf: 2
wttm: zfpg + blmz
rprp: 4
dzwl: 2
vbdr: 4
vdfq: jzhp + jghr
tlwj: nfbc * ssbv
vwsq: sztw + zqqq
bdgn: rwlm * cpzr
fjvt: 4
zqdh: nmhj + nzjc
wgcf: zspb * vggz
bzhw: czzh + fwgr
qwsv: bpgz / btlc
clsj: nlrn + jbzg
hzjc: 15
qfvm: fbfl * whpc
ljlt: ccpm / rvzl
qdmj: 17
nqmp: bvnb - ngpm
rqww: 2
qpgt: ftmv * lchz
tpbg: 5
vmsd: 2
wrqr: 5
bsss: qwfb * wcpw
jhjv: gmnz * nvzn
lsjp: 5
vpgn: 2
hzwt: 6
nsmh: 5
pnzn: 5
mdbc: fljj * nshm
nrdh: wdss + dhqv
fmwh: 3
nmjt: 19
wzcg: qmtt * vjjr
fdcr: fmwm * qwjp
smcl: rzmp * gbfq
tdsz: dgsc * rmgd
pfqc: 5
zcqt: gqgt * plts
wfbr: nhbr + lvfs
qrrh: tpbs + hstz
wthb: fgvc * nglb
brdv: 5
sglr: 6
psqg: qngr + mdfp
gpvf: 5
wnnd: 2
zzvr: qfnl + rcsd
gqnm: nqrm * jmld
ndcb: gzfz * dsgj
vqrc: nmww * lbbc
ptpc: 3
dqtn: jcqq / qzgh
lnrb: 4
rzhm: 4
mqzc: pbft + lbqg
gqdl: 2
ntbd: bhwc + crwz
cvqn: nbjh + jlmh
tztq: wmpz + dbgh
gndm: qhpb / ghws
mzsc: 20
whrt: 2
vqvl: zbnw * zbnl
pppq: 2
glzh: ffld - rzft
jpwl: qqjs * rtcb
nszn: jzhs + mngv
wvsr: 6
rpmw: 5
smbr: 1
lvbd: 5
zgqr: 13
nrld: 2
rdml: 2
dfwt: ldpw * zlwv
zftq: pnfl * hdvr
smjq: ctgs + ptdh
vjph: wffh * hzfn
grss: jbpl + shnh
zpvm: hbpb + gdnv
dsrc: zvzf * cqff
mjmg: 10
vnsn: 3
wpgg: slhn + tjvn
qmtt: ntvs * qgzw
brmt: znhz - hwdb
bfhd: rvpz + ddmp
lcst: jwbp + bqvg
sqhh: 4
spvd: ccwz / tlqv
gbpn: 5
dbqn: 4
wdss: vdcg * cznl
qgvz: 4
mspm: ztsn - mdwv
qbqw: vdmw * nsbb
nngg: tdlv / ghpm
lsnz: wdsq * gnfl
svpd: tcps - tgmf
nbjg: 1
dqqm: 3
ljpl: 3
mpql: 2
zlnn: ljzg + ddfj
zzsd: ltpn - mndf
nlvz: cjnl / nqgp
cwdw: vvdb * nrld
drrd: 1
nmhz: 3
vvvt: jfrp * nwdt
whpc: 5
bhgl: wddc + sljw
wssp: 2
ntnp: 3
gmsq: qrnl + znfz
clch: lhzd * hdfd
vfjb: 7
wzjg: bdpv * lhvg
hmqz: tgzp * nchz
lwnl: 2
gtnd: fcpw * zzzm
pnvb: lvbd * hvqp
wcpw: qhbs + jvpb
jqnq: 2
gtlb: tnfq + jzpn
nlcs: 3
dbcq: 14
rsfh: 3
fcsq: mmzw + wlst
dqgn: tmjb * tdzs
sqsq: nmdh + crgh
frlw: vwqt * hplq
jwbp: vhcc * fqlj
lnvm: ztqf * wdtn
lpql: wcdr * jhjv
grqr: jbld * dtjc
dwtj: zzjt * sbdl
hgqq: llzc * lmlj
cqff: fjgg - hcvc
lszb: 19
rzzz: sgqt * nsrs
fcpw: 2
gjzg: 9
dbnv: 3
mngr: 2
lmpw: tgbd + zzdb
pvhf: 9
hjrf: fzwn + mnqw
pdgq: dvlj * rwqs
bqnl: 13
nsnf: 16
cldg: ptvg * hrnt
wgtv: 2
zgtl: 2
hdvr: 2
qnqn: ldwb * tfzn
qlqn: 9
mcmc: 4
hcnr: lpzl * jjjw
qwhm: scqp * rjsj
ttst: dtsv + grqr
zfbw: 2
zrlp: bwcd * znqj
ggcp: 15
pvtc: hzch * slzv
nlrn: ljpl * bjcn
mfbc: zlld + tqbg
root: ztbt + jzqh
dqbf: drlb * cgvt
spgv: 2
vvtv: 4
ndrl: 3
ppws: lbrb * spdv
ldcw: 2
fmrj: 2
zfbg: 16
zspb: 2
dhmq: 3
mdwg: pqql * pnpz
fsdh: 4
trsz: 12
qrzp: 7
grcp: 3
tlhd: 13
vzvj: 3
wjql: dsvz * ddbs
vsvv: gbhf + vwsq
zhwc: 9
vtjs: jfmh * nlcs
gjzj: bpzs * vhdh
gwzg: zrjv / blrb
chfq: jbtr * mvpm
mhfd: 5
srlv: wnlr - btbj
lzvl: zhqd * pvch
qzvd: 2
fvnt: dmvn * wfwg
hstz: fsqq + cbmc
fprv: 3
zjvf: 5
vhdh: ltwr * lqsm
sctr: 4
pndd: cfhm + fvct
fqhn: cvrv * qdlq
mjlb: rbql / clcc
brnl: vdcn + pbbb
fljj: dqgn * twgb
pnwn: jbbh + zfzs
stds: psms * bzth
chgv: jnrd * nqtg
wflj: 2
wjpb: 5
cbpm: 3
nqqc: 2
jlpz: 3
pntn: 5
dtgw: qwsv * ncvm
bdrf: 3
tjpn: mftl * vfst
wcdr: 18
cqfd: 2
tzvg: wssp + jnnr
vtgq: rhng * mdgz
shnh: bntn * dzft
nchz: bwmz + jgcd
mscm: dlmb * rcns
zhzn: mfbc * ldvg
jchg: 3
hjzv: 3
dzdc: ltml + hvtv
tntw: 5
rdfd: 4
zlwv: 3
zfvw: zftq + dgrp
hbnz: 9
rpgd: lmnl * jnmn
bpgj: pvjd + vhpj
ptpp: jdpf * cgrr
qzgh: 2
wgrl: 7
dbts: sjfb + hzwt
djrh: pzmz * rmmd
lgfh: 3
vmcr: 1
cbqt: zvwb - dwch
slrw: 2
hfzc: 5
qwfd: 2
ffjh: 18
qcjz: 5
qvmc: glww + nwgj
slmr: zbtn + crch
fbpf: stpj + pscc
qths: 20
bbft: cwdj * bjfp
vbwj: qtcv * tdjm
vgcs: gbmh + nhtg
jsvb: 2
nftm: trsz + fsvf
llmt: mzwc + csgh
hzcg: 2
ztqf: 5
fpnh: 4
lbzg: 1
lszh: 4
sjdr: 6
tjrw: lhhr * tsjl
rjqq: fpcp - vggg
hrjb: 2
cwlb: djrm * dgpd
tpbs: lbbj * mghv
cpms: 4
cqbc: jfqh * nnhl
ghlp: gjsq * cwdw
pqvp: 3
dmmf: 7
dtvj: 7
spnp: 5
bfns: rmsr + zwwj
vllw: 5
bggz: hjws + gggn
dmtj: 3
rhtp: 4
mvnf: 6
gqrs: 2
drlb: 2
hlml: rwgh * glqb
dwrh: 7
sjhr: vgbn * gwgj
gccv: qzgc * bszn
lfhm: 1
fvtv: slmr * vtrj
hjqv: 2
ppsz: 2
scch: 3
mdwv: 3
hhzd: bcch + hwbj
mmrz: 6
dbpj: dfwl + brdv
cbmc: rvhg * sqmp
fjmb: 2
njvl: lrbb * dgcg
vfst: njvl * dsfr
vrzb: nrdc * djzr
lmrn: tshf + rbzj
jthl: nghz + chfq
rpff: 2
bpzs: rrln + dblg
npdn: 7
lfnf: 2
djnm: qncp * mqpp
nwgj: 2
vggg: zjfq * hnlc
vztl: 2
rzsg: tpwp + vbdj
sfjz: 4
fcbj: 2
tsds: blht - tmcv
dqnj: 15
vsdz: tqsf * sqpl
mqsc: fssn * dbpj
ncsd: 2
fjvv: 8
wtsg: vbwj * zvqb
wfwg: 5
qlsb: 4
cpwq: pjlj + nvzp
gmnz: 3
ldbt: chdd + wlmj
mlsf: 2
hvwq: prng - rpnw
nfbc: 2
shlz: bcdq / qlmr
wlmj: 1
swrw: cnjm / mdjv
vvdb: qlbz + sqsq
vwtm: tlqw * dfrn
hhhv: fvcb + nftw
djfb: qdbj * qsrg
mrwc: 5
gdbh: 4
lpzl: 3
dbgh: 1
czfq: 13
ctsn: 4
btvb: fscw * zpfj
bdzw: 7
vpmg: jqbw + ccdj
vqpw: psjt * fvvr
nlzh: 17
ndwm: 2
lmwc: rzzz + qdwl
bgqh: 2
tsrz: pfpm * rwzc
pnqs: 3
dpbl: rqlg * cvnq
fjll: 5
blqf: 3
fvsg: 3
chrr: 3
cpjh: 5
vpvg: 19
llns: 14
fvvw: 5
gfcq: rnst * vpmd
qqjs: 18
qsrg: 13
ncdw: wjgg * wqsc
fzmt: 16
msfb: zmnq * lbsl
drzt: hhrr * fmgv
pggt: bwmp + mmmc
pvnb: 5
bdjs: 3
ddcb: 4
djgt: 5
hghz: 15
wpff: wzcs * zvnp
ljhn: 3
wgnl: 3
pdwv: mscm / whwq
mnqw: 4
ftmv: gpdl + bcpl
gtdt: 2
jzbr: 2
mzcf: zgvs - vbdr
ggzq: gqtv + bhgl
fcvp: vbjs * hszb
dttt: fwtg + pndd
ttsb: whhr + bsjf
sgll: 5
gnfl: 2
dhtw: fbpn + ljvt
spvb: 2
wvwq: nbsf / cnsq
jrsb: mzwq + cnrt
ptll: bdgn + hrrc
drmd: 4
ppbs: gbpt + jnnv
mhqw: rdqp + wttp
dgsc: djvt + blqf
qwnc: 2
ldfd: vmqv + ggcp
jngs: hwww - hlfn
pcvm: 6
zgvs: zqgj + mrvd
hvpt: gwpf * ntnp
jfmh: 2
ncqt: zqzw + jthl
clqt: lqwh * vjgh
thtw: 5
sbfs: 19
nshm: jhvv + phps
pssl: mffs + dhcj
rjpv: lrjd / pbcr
tclc: 3
gwgj: rnhq / nqwg
pftg: mcrg * mmrm
dgdq: hnjf - dvzq
pstm: 3
nsjw: dgtw + dcrf
jgdl: 5
wfpg: 3
gcdr: 2
vpdn: ndgv * qnqn
jhvv: 5
mhjz: 6
vcpb: jfqq * mlfn
nzph: trdf + hwdd
zwff: ttst + gddm
mmrm: gqlp * cvtw
sjlz: 7
fsqq: zhzn * tlnl
tmzc: 12
cmrs: bhcm / hnpn
jbdg: djfq * ctjm
vpmd: gblj + dhqb
srnv: dnfw * smrj
hpzl: vvwm - hznl
tqzb: qmpr / mdjr
ftlj: ftrr * fbnr
qzsd: 8
rzsr: mhqw + lbdt
drsc: 3
wpwl: 19
wvwv: 2
wwnt: bpmf + wttm
fzvr: 7
ddmp: 13
mdjv: 2
jnmz: hgrl + fqhn
wmmr: pmdq * gjsh
ffld: hqpw * qptz
bdpv: dpnq + qhtw
tmcv: 8
cpzr: gjtt - rszj
wtnr: nrdh * ptpc
npzw: nrrw - clch
dgpd: zwtg + dbnv
rmnj: 2
zdfh: 12
hwdb: rmqr * lldv
jqqr: 4
jnrd: 3
gspz: wvlm * tsds
wbbg: zhjc + wjpb
wcsp: jpcf / mvnf
nghz: vgrp * zrzl
cnjm: dsrc - lhpz
fdlg: 4
qqsz: 3
sttq: qrzp + jzdf
zvwz: pbrv + nwgg
nqrm: 5
bftt: 3
whwq: 2
zpvv: dhmq * zfqh
bjrw: 1
rnlm: 2
sfpl: bgqb * pnrz
psnr: 3
ssbv: 11
jfrp: vsdz + tcwm
zmzv: 1
hplw: 5
psfc: dfdv - jwrz
mwcq: wpwl * grbj
gdzg: 3
fzzh: drcq * btll
bttr: 9
wjrq: psnr * lcmn
ssvc: tllf / lrnf
cczh: hlhv + rdlg
qlph: zpjq / tcnv
rcsc: 1
dzhv: 2
rnlg: 2
psqp: 4
zlbf: qdmj + ljnh
cjnv: 2
hwwn: wbgq + twlb
plwl: 2
dcnd: 2
mdlr: gqqf * hzqh
lvpz: 7
wbgq: 8
zzzm: cczh * wfpg
cgft: 7
rgcz: 2
zwwj: scpj * twst
wfcp: 15
ddfj: pgjs * qcpw
tvfv: 10
frjt: 4
tspc: 4
spdv: 12
gbpt: 3
qrjj: jrmp + smdg
gfcr: tlwc * lsjp
ljnh: fhhg + jsng
frpf: 4
wmfr: bmpm * sdtj
rrln: lsww * wlqj
vtnv: mttd * btdv
dhcm: djmb + cpmw
vtqs: bgls + bjrw
pqdh: 2
vhqb: 3
ngjz: wgsb * gphb
hrlw: lzcj + qdvw
tsfc: qhbj * hmbs
hndr: mpss + msfb
sqfh: 4
lbbc: ngvm * jcsh
twsh: 17
ldwb: 2
jfnm: drqb * smfd
tsjl: lltd + qclr
nsfg: bfbp - lmpw
tdlv: rnlg * vqpw
wbdw: zlnn / hphj
fcwp: hchd + dsct
zpfj: 3
jjjw: 2
bmrj: 2
qsng: 2
zgvr: dtrb + hptr
gszz: 4
ldvg: 3
mjpq: chmr * tztd
vhnf: rscm * bgqh
cmhs: 2
vdmw: 5
rmzh: 10
zscb: llns / swnc
rvpj: 7
dnfw: 3
wflv: 5
gbln: bhrh / cqfd
szbz: 3
vmnt: jchv + fwbm
wwpm: lcnr + mjpt
rpwl: rgjl * dwlj
phps: 1
fwgm: 13
tlwc: 2
slzc: 1
jwrz: 1
djhm: 4
fmpj: 15
pzqg: bbtz * rvmf
mrdz: stjs * bzrb
pbrv: jgdd * fzwb
gbrs: srsz * vqcw
rsdj: 3
ztsn: 9
lhql: 2
vwbq: wfcm * szbz
fjgg: stbd / dchf
qncp: 17
fvpl: wzsf + dnwv
gcjj: slrw * rhjw
npds: 8
tnfq: spvb * qrrh
zvwb: szmd * vjpg
tsms: lfhm + smnj
llgb: fvmp + mqqd
vmls: tnrp + hwhh
btfm: 10
cbhs: nrpn * tchc
hdsm: 1
nrtc: 17
dwsm: 2
slnj: 2
mcrg: 4
gsjf: djtd / jwsp
ndgv: 2
djzr: 7
crzh: dfwt * wgsg
ggjd: rnlm + tntw
hzqs: pcpp - zjzz
lsfd: 4
pqql: 2
rfjg: tqzb + bjrz
jnnr: 20
jzhr: zzvr - dfsv
wbdj: 1
hdsj: tdsz + ldfd
cgbc: dfrs + jlcj
htms: 16
crtt: rhtp * mrqc
brjh: clsj / bqcm
twnz: dcnf + zmqd
qpwz: 2
hzwj: wcsq + pjhn
wgsg: 3
mhsv: gsjf * drsc
bqmn: wdww + zmts
sbnr: 16
wvwt: lrcl - lwdw
ftrr: pwmf * dzdc
dsgj: 7
rwns: wqnt * pntn
rwgh: 2
mhsh: 2
wrtl: gspz / zcqv
rnmv: 5
lhzd: 2
zjzz: wfcp * tvfv
hghv: 4
rvbp: 5
tbtm: gfzr + dqtn
bwlh: 3
nwph: 4
gjmh: pdwv * pbts
bhrh: tnnj * lzjt
dvnp: spml * zscb
mwww: 2
glww: ddwm / cjnv
qqsn: 4
nrrw: srlv * bwlh
lcnr: mrdz + smbr
qghp: tcfs - cfng
drtq: hvsg + vcpb
jpcf: zdvj * lgww
llzc: bswr / nrts
bnjb: qwds * hpzl
zzdb: slmq + lcst
lwgf: qhww + glvc
gddm: 14
dbhh: zdfh + pssl
vnzp: 14
crnw: rdfd + zrjn
pvvs: nhfv + brcc
cnvp: 3
bpgz: sjlz * vlhj
qngr: 6
bsmh: 3
tcfs: fczd * pvlf
mjpt: 8
jndt: 6
jnwq: nmhs * nttv
ppbt: 4
smrj: 3
wmnl: rcsc + cjrv
qtcv: 4
rmmd: 3
vtrj: 5
qbtl: 5
ldvl: wqjt + qwdm
ffvr: 17
fzwn: lszb * dcww
zqfb: 2
bcqr: cbmm * lcfn
vdtg: 5
csrr: 3
tgjn: 3
swlr: 2
ncrd: 2
cwtc: 2
mfjh: trdh * gqrs
jgtr: 2
lfvc: 7
bhlp: tqfl / htpr
smbb: 10
jjpv: fqhd * wpgg
mmtm: 2
bsjf: tgsl * zbrh
dtts: 4
nqcc: nsfg * jqnq
phdv: 3
qvdz: 2
vzhr: 3
qzgn: 2
jljb: jtvf * gvwp
tmjb: 2
dzgz: trwj + mmrz
jvcm: sjmh + crtt
tfzn: hlpl * vrfh
jhhs: 3
fqhd: mdwm - nbjg
hmpq: ghpq + gmsq
gqlp: 3
zbnw: 7
blht: nrjm * slwp
cdjh: 2
jgws: 2
tllf: hqlm + cpwq
trdh: 11
smfd: 2
vrfh: 2
bcch: 4
ghpm: 2
nmtv: 3
dgtw: 4
rbvw: jjqv + swnr
ddmn: 1
zvzf: fggt * qfhf
fsfh: 19
dcrf: 9
jnfc: psnb * jzfh
thvn: tmzm * dzgz
hlpl: 5
nhbb: 2
qcdq: mdpl + fvpl
nvzn: 3
lcws: qpll * brbw
qhjc: rfcn * czpp
hzqh: pppq * sslm
htdj: qwwf * pwsq
grmc: fhwr + lnlw
tgll: bvmd * lmjh
lrjd: hrgn * ctph
lngb: mqgb * lgrc
mtfq: tnmv + vwtr
hvtv: 2
cbnb: 2
pcpp: nqcc / ctzc
vjjq: zpdq * gndm
rpdz: 2
tlbt: cpms + mdwb
jbtr: bttr + spgv
pqsm: cmpp * rqww
lbqg: 5
wqhs: qbbm - pstm
grbj: 5
zbrh: mngr * zfbg
nlhg: dtts * tplv
bqqm: 5
dbqv: bvtg + nwph
nvzp: qths + ntbn
dlpq: gvbl * hrjb
twst: 3
zljq: bftt * vsgp
rznn: 4
djqm: 15
zgvm: 4
zjbv: hzqs * cjlm
qsfz: 3
drmr: clrz + nszn
pnrl: ndcb + pggt
hchd: plsr * qtjw
qfhf: 14
jjqf: sftc * sbfs
jwqw: pgrw - zsrt
hzch: 11
tdzs: 4
gbmh: 1
zmnq: 2
ljvt: 12
fpcp: rzwt * pqdh
rqhb: pchr / cwfl
cqnl: rnct * shlz
ddmb: snct - mljv
ptdh: mwqs * qghp
nrhp: 19
wddc: tsrz + zpvm
ddrj: 11
zdfm: llgb * llnr
rmrl: frlw + fpnh
jvpd: 4
znrr: gwsc + hmwq
hhtw: 17
zztv: 5
brcc: 3
fjqz: lqhg * hnvl
tltw: 6
zmph: 2
vjpg: 2
ddbp: 5
ddwm: wmgz * ntdb
fjmm: 2
szsb: wqnd + qmbp
flzn: cptz / wrqr
dmzg: fzrw * nsnf
jcqq: gbrs - qzsd
fcsp: tgjn * tqcb
btll: hdgt + dmzg
jndp: gcmh + jqcw
jchm: 2
nrrr: 5
fmbm: cbnb + pnzn
hmmp: 2
jfqq: qqqr * rlgz
mqpp: 3
dwsw: 2
nqtg: ppbs + tgll
czth: ldnf * nsmh
gcql: hmqz + stgw
tzjh: 9
qldv: jvpj + tfzp
lnlw: rbls + cpzw
bbcg: hzcg * thtw
dlgz: 3
rgnh: 2
tchd: hvgv / rpff
pdwt: 3
zjhd: sqhh * rzqj
jghn: qsbb / ncsd
ctdv: 14
mqdr: wcqm + slrg
zccw: 5
zfcq: lfgc * qrfn
pwll: 2
dfwl: pmjs * dmrm
lmjh: 3
dzvd: mtfq - jbdg
vbdj: gqdl * jfgw
wcrc: mfmd + zpvv
vjgh: tnbd * dfbd
htdb: 2
vnlq: 8
vrdz: wbjl * jsvb
wvlm: zjhd * cbpm
blsl: jfjb - nlhg
qpwl: 2
mcfq: dmhr + sgsg
nhpp: wzvb * jhhs
psmh: 14
zvlw: 2
lpbg: 2
rzzd: 2
jlpf: 8
mhhc: 3
qvgz: hmmp + fcjq
lptz: 13
wcqm: 2
rqpt: 4
drcq: 3
gdnv: lngb + cmvm
qclr: fjmm * llcd
pjst: 7
rzvp: smcl / zmfj
cgrr: 5
mmmc: glzh - slnj
wqjc: 6
brcj: mqzc * pdqq
bzlf: 3
spsb: 2
bvtg: 7
swzz: lwgf / vdtg
hhrr: fcsp * lfvc
fvnd: 2
rgpl: 8
ltfg: hbrg + hndr
bllh: 12
fvcn: vllw * nfjw
hlfn: 2
jflj: dltv * lhfr
ljmm: 5
wpnm: lbgq * gjrg
pdcd: 4
rdlg: 6
vjdp: 2
qsfh: qfvv * hfwz
wbrc: 2
fbzf: mtgf + qzvv
nmlw: hbmq * gtdt
fgnz: 3
tqsf: 6
rjvz: 11
mnsq: 2
ltqs: wcqs + rjjb
bbjq: fwgm * htdb
qfgj: 2
lqsm: 2
nnhl: 2
dswc: bzfm / qdfp
drqb: ftlj + qwcc
vlhj: 4
ptvg: ndrl * pngj
lcfb: vmcr + wjrq
ttjn: jlpz * bdsl
bpbn: 4
cmbj: 3
wbwl: 5
rwrz: 2
trth: pqdm * vjhn
tlqv: 2
fhhg: 5
qhbj: 6
dwch: wwnt + jsbm
plts: rsdj + dpjg
gflc: lgbm + dwtz
bjrz: jbzz + cqbc
fslj: nhpp * jcwl
twms: rvcl / dgdq
rmsr: wjql + gqnm
rvmf: 4
qtjr: 5
pnqh: bcgm - scvq
zfzs: 3
qgzw: btcm + tdpv
wzcs: 5
shwb: 4
rnst: wbdw / dtdz
dfsv: ldhb * bwhl
jcvz: 18
vgmb: 4
dcnf: 11
czzh: dfdb + cmpj
dzft: 5
mjrc: npzw / lnbw
mdpb: 8
qgqr: mmbq * bzlf
jghr: jrsn * frpf
hcsj: sblt - njwd
cfhm: phnj * tflw
lqwh: 5
hzbg: 3
zqtw: wrbv * djgh
ssqd: vsvv + vbgg
lqfw: 5
cnrt: 5
dbhm: 2
ltpn: bzgh * mrfn
nhbr: 5
pvlf: 2
rhjw: ljpz - wrpl
tzzc: htfs + hcnr
hnvl: 6
zdln: 1
lhpz: bptz * pvnb
srdj: 3
sftc: 2
tnnj: 13
cznl: mndc + wgql
wttp: 16
cjlm: 2
pbbb: tbhm + rgpl
pzmz: glbj + rdpb
vgbr: 13
rfsj: 3
qwjp: 3
cmpj: jgtr * jpwp
htfs: 1
nrts: 2
zqgj: cpjh * rpdz
lndt: fslj + sttq
rscm: 3
lmlj: 2
tnbd: 3
vfhb: 4
rcns: 3
tshf: sqgn * mmtg
gcpg: wgcf + vqvl
zpzj: 5
fqpl: grmc * tgfp
hvgv: fcbj * mdhs
rnhq: qcdq * swlr
vlvf: nptc + bfhd
drts: 5
dtgf: gcjj + bcwn
clmm: 1
hgrm: lnvm + mhlr
plbr: 3
gttl: dcql - rbfg
szwg: zjvf + wllr
zplr: nrhp * hhvn
ltwr: 5
sftl: dfjz + rhrz
qjmn: 9
czpp: 3
cplh: hrzh * tfpl
rzwt: mjrc + cncf
tmpc: 2
zrcd: nlzh * gzsh
zwtg: bfgf * rdml
nggc: hwmv / tpct
wqsc: 6
gqhl: 9
rpln: dfpd + vpmg
qlmr: 4
swnc: 2
cfqb: 16
ghrp: pdsg * rzvv
smbs: rpgd + tlgs
jqcf: 7
fqlj: cnvp * jhsj
wlst: zgvr * fpfn
hptr: ddmn + qlph
lzpt: mvzw + jnfc
lmvf: 4
llqn: 12
glmh: 3
hrhc: 2
fjnr: 2
hfnn: rjpv * chpw
vjzp: vztl * fplr
hmwq: 3
nqpz: wmpn + zwjl
shbs: 5
jtvf: lhql * tlbt
twgb: 4
mdjr: 2
wpfm: dbhh + zczg
jpwp: 13
lqcf: 4
wlnp: dvrs * cdjh
fvmp: 5
zbtn: sdnc / zfbw
ntdb: 2
chpw: wcsp - twsh
dtcf: 2
bsvg: ddtt * jhfc
mttd: 2
dhrf: vjph + fcvp
ldhb: zdzf * tchd
zrhc: 8
hrzh: 3
psms: fltj - cccb
bwcd: 3
nznn: 2
sljw: fmzg * zqcv
ltbz: 16
glvc: bbvl * flbd
clrv: 3
gphb: 2
fvrf: 14
rrjb: rvmd + qjzr
wcjz: 20
wddt: 7
nsgp: ntzb / rwjg
tndj: vjzp + wnnd
tqbm: mhjz * wjqz
hphj: 4
lhvg: 2
fwbm: 2
zdzf: 2
zbdl: pnvb * lmrn
rdrh: tplr * mfpq
tztd: 2
jdsw: 5
wwwg: fnqj * ffvr
mvpm: 3
lzbt: 2
ntvs: 5
qjqz: drmd * nqst
rlfm: 3
tdjm: 3
gntp: zvvs + jvtr
pmdq: tbgb * bwcw
lcjn: nrlf * rfsj
dnfd: 5
wjqz: pvhf * pdjp
nrpn: 3
nzmp: bbvs + vnzp
rqqb: snrq + mjmg
lqjv: vfjb * dprb
nttv: 15
rdqp: 2
hmcl: 3
pnrz: 2
pscc: 5
bvmd: 2
lfgc: psqg + mfjh
vjvb: ccfq * scch
mdwm: zcqt / pgth
jbpl: 2
bmlm: 6
rpft: lzvl + sjhr
slhn: clmm + bqmn
cncf: zplr - dwrh
fpjm: 3
pqdm: dpvs * zjqq
fzrw: 2
bwmp: 2
dgrp: 6
mvzw: llzv + gmdd
bbvl: rznn + bbqj
jrwr: 3
mvvh: rwrz + cvqn
wdbz: 4
psmm: tztq * vgmb
jhnv: qbtl + qdgz
djmb: dgdm * fzdp
tnmv: glfp * szsb
wpvh: 8
trcn: bhld * vmls
nglb: 9
hfvw: plbr * vbwc
jqbw: svpv * ddbp
gggf: 2
jhfc: 2
grqd: 13
rrch: drts * bdrf
wbpm: ldgt * hwwn
tbrw: gnwd * bsmh
fssn: 8
bswr: hrlw * qwfd
whbr: hbwp * cvzj
nbsf: wmnl * jtjg
jllq: gtlb + nlvz
sztw: 4
zwjl: fmpj + mcmc
wllr: 3
vhpj: 2
pvfl: zpmz * vngc
djgh: 3
jzqh: qpgt * jflj
vwqt: 16
rspp: rrjb + vwtm
brgw: 3
tflw: 3
gzsh: 2
bzrb: 4
zbsf: mnsq * wfbr
qdgw: 1
bbtz: wfzt + lthh
gggn: qvmc * mpvz
tqfl: 14
mdgz: 2
zmnm: nqgl + flzn
fffq: fzqc * zdjz
zlqb: rhgp * jngs
jljl: bfns + tbrw
glfp: grqd * hmbv
wlfv: qjtb * pfqc
fmgv: wrtl + sbgd
jbld: 3
wggc: 1
vsbp: 5
ljzg: jmbp * ftvq
nmhj: fpwj + zrqh
lrnf: 2
fdrd: 2
hwmv: pvvs + jqqw
snct: 18
jnlg: 2
gmgt: ldvl + ssvc
vtzd: nggc * dcnd
hwhh: 9
smdg: 4
hrrc: qfcm / hjqv
twzw: jmrn * ttsb
gqtr: 2
pmgs: dmmf + gztm
chmr: jfmz + twcb
jlmj: 2
zfqh: 3
flcs: 2
hvrm: tndj * rzzd
jfqh: bldr / jlpf
lnbw: 2
slld: zmgc + lzhb
qlbz: pfwz * lgff
sftv: brzm + fgvg
nmww: 3
wzqv: npds + brgw
rbnj: qtqw / zvlw
lhfr: fwhw + drzt
bmpm: 2
qwcc: 5
vhcc: 4
qhww: lqfw * wlnp
jfjb: mvvh * fwvz
bsqj: mdbc + trmt
qfdb: 2
pzjh: lzbt * hcsj
qjls: 3
bbvs: brjh - zmph
sdtj: ltqs - mgjw
dhww: 1
tqbg: ngjz * snwp
zmts: mdjn + rthz
qwwt: 4
sdnc: mdbz + wvwt
ngvz: 9
zqcv: 3
humn: 1637
zfjq: 3
wtdb: 8
jzhs: wqnz * pfjn
tclq: 2
mbvp: rsfh * gcdg
wgsb: hgqq / vjnq
ddbs: 19
qwds: 8
dfdb: 3
jlzg: 5
slwp: mjpz + mcgg
sjcp: 2
mrvd: 2
bmgn: nzph + vsjj
dsfr: 2
sbvt: zwff * wzqv
pczm: 8
ltrp: vgrh * ptqj
wpww: dhrf + wnbf
gmmc: 11
trls: vwbt - vvvt
bgnw: sjdr + gfzt
vsng: crzh + lnrb
czcp: gjzg * qjmn
tgfp: vfhb + nmhz
mwvl: pftg + vdfq
ghpz: lwgj * zsnh
gmbz: 11
fsvf: 2
hdgt: dlgr * tmgd
lgbm: tsms + dmtj
pfwz: 10
rdtm: 2
wnwl: 3
nwgg: 1
htcs: bgvr + mdlr
mdbj: zpzj * lvpz
wngv: hflj + fbpf
zsnh: 4
vjhn: bmmd * gdld
jjqv: 16
pdjp: 3
zjqq: fmnf * mpql
qbjm: 15
lzcj: qvbf - spnp
pbcr: 3
vjnq: 2
bcwn: ngzc * svpd
qfnl: blsl * wcrc
vrsf: 3
vbjs: 3
qqlt: 2
bvqw: 2
cztn: 2
wshq: vjvb + vpdn
vcpr: 1
nptc: vwpz + ljmm
jlmh: qzvd * fdlg
ctzc: 2
wrpl: hgrm * zfvw
vwzc: 2
sblt: trls / wsnb
pvdm: 17
jwsp: 2
pbst: btfz + csnh
tpcr: 2
mtlh: cqnl + mbtd
dvrs: phdv * hjbp
jzhp: fwcr + jjpv
pzdq: lwgb * plwl
jqrm: 2
gfjf: dnqt * fhqm
cnlr: fjqv * rbrm
wqnt: 7
jrfq: 3
whzm: rgcz * gttl
bjlf: flbh + gcql
hwww: fjzl + rbvw
bgls: 5
qmjw: 5
srbc: dlrz + vgdm
sgqt: 3
srsz: ddrj * lwnl
gqwh: jqqr * hjqc
ntzb: wpff + ppws
zblv: grss + pscp
qdwl: 4
bnvh: 3
cvzj: fzmt - ldcw
ntmj: dzwl * qthd
gwzf: gcdr * nvdn
tjfc: qjmr * mjlb
wcfc: 2
mngv: bvqw * hmcj
nprf: 3
psnb: hzwj * pbhs
wlqj: 5
fczd: vlvt * hwjf
zpjq: nqqc * dhrd
lthh: wtnr / glmh
qjcz: 13
wwcr: 7
nftw: 7
mcft: 2
bjfp: 2
dtsv: 15
swvt: bsss + rhvv
bcgm: crmp * dzvd
bbqj: 7
rjzt: 5
hlhv: 17
dvlj: 2
dgcg: vzhr + wdbz
qtqw: mrgf + bpbn
rhnb: 2
wvvs: 3
blrb: 3
cccb: 5
bntn: 2
psjl: vlvf + jndt
wtqb: 4
qwdm: bzhw * zqfb
wqcn: 2
bbvw: 9
jzvm: djhm * gpwj
bzfm: gggf * wpww
vlvt: 2
psjt: 5
gjsh: 7
gqtv: 3
gncf: 6
dltv: 3
qcpw: 4
qwvt: 4
tmgd: jlzg * mrwc
lqbm: 2
mdjn: 1
tbhv: zrhc + dqnj
qpll: 3
wvdv: zztv * vgcs
qhtw: 9
vqsj: 1
lvsj: tlwj / qvdz
wcqs: qwvt * cvwb
dmhr: ndwm * wvvs
zdjz: 9
dcql: brmt / jrwr
lzjt: rhnb * zccw
pbbh: wvsr * bnvh
blnn: hvrm + djfb
dmtg: 4
llcd: 19
rvmd: zszm * fsdh
dwtz: 11
qhdn: 5
vgdm: zglm * qwwt
jhfw: 3
bgpl: wbpm / pwll
qmpr: gwzf * dptq
ccpm: fdrd * drmr
dtjc: mttg + sdcw
vrmc: 3
shcq: scdn + bggz
znsd: rqqb * qjls
lwgj: 5
rtqv: fcch + hjwl
crmp: scsc / qwnc
zmqw: 6
hmdf: dbcq * lslf
qdfp: 4
gcmh: lrmt + qwrs
hwbj: 3
bwhl: 2
gjrg: 2
lcfn: qlbn / wqjc
snrq: 1
bzth: 3
vdcg: 2
nffc: hmcl + wjpd
qvjb: 5
wzpw: tmvr + ltfg
cvtw: bjns + wlfv
rlgz: 2
pmjs: qpjm / wcfc
bwcw: 2
bdsl: 2
gvbl: rmzh + hjzv
blmz: nsjw * nrrr
tchc: 12
vbvz: ghlp + gntp
bgqb: 10
qwfg: 13
hzvf: 3
qwwf: 7
lsmb: 4
fvct: vwbq * ffgm
rtcb: 9
hrgn: 3
qldh: ncqt * ggqc
mrqc: 3
rbrm: 2
qlbn: bdjm * jjqf
jsbm: wclz + pvqz
cvrv: 2
fwtg: jqgh + zpwl
prtw: 5
jfmz: jqcf + pbbh
bqcm: zgtl * thfv
pfjn: ncdw + crql
rvtz: 9
cqhl: 5
zdvj: gpvf * sqfv
sjfb: dlrj + qzzn
mgws: 3
blpf: rqhb - bjlf
vzrp: vbfs * fzqv
wnbf: gccv / chst
dvzq: 1
pjhn: 5
dsct: lpqg + ghpz
hbpb: 9
fbct: wzwz + ssrb
fmwm: 3
prng: slbf * cqhl
jwnp: gfcr + lmwc
ntbn: cnlr - pvdm
npcl: ptft * hvtc
rqqs: 3
tplr: mbzl + vjjq
pbft: wgnl * lfnf
zrjn: 3
mghv: trcn - sglr
zmfj: 2
flbh: bllh + fvnt
qhwc: 15
frqj: ctdv * ggjd
fmpm: bbcg + zrlp
cmvm: 9
gnql: dwsw * frjt
dtdz: 3
ztcb: tncc + bhlp
cflv: mqsc - jwqw
nmdh: wtdb * fjvv
wjrw: 7
hwdd: 5
wnjn: 2
snvh: 4
zmqd: gqtr * zggm
zwdg: wwtl * dbqv
nhfv: lpbg * fjbw
djfq: tbfh * fzmw
mffs: qjvg * pndf
fscw: 2
jtjt: 3
wmgz: dgwv + wrbq
tcsd: fwqw * tlhd
nsbb: stds / zgqf
rzvv: 9
mwqs: 3
lbsl: lwlj * jdmq
fhqm: 2
npng: 3
hszb: 11
tgsl: 2
lcmn: 4
wmpz: rvpj + hdsm
lgww: hfvw * fjvt
cqfm: cbhs + ccpb
hmbv: 13
tplv: 4
zgqf: 3
pndf: 3
wjpd: zlgh * dsjh
cpbm: 2
mfqh: wggc + srfq
mfpq: 3
rhng: 4
dfrs: tgbp + wdtf
dhqb: vsbw + mdbj
dpjg: 4
gztm: 1
btsn: 2
fzqc: 2
nsvs: qgqr + ggzq
hrgj: lcfb * wbrc
zgrf: 3
rvhg: lzvm + pnrl
zszm: 2
lwlj: 2
ptqj: qfgj * qcqm
vbfs: 2
tcps: ncqq * vgbr
jmrn: 4
zhvt: 2
nrjm: jvvv * ldvn
sqpl: 9
wjgg: lbzg + tcct
ccpb: zlpj - slzc
jrsd: 18
wcsq: 2
hnpn: 2
hbrg: 12
hbmq: 12
flbd: 5
csgh: dswc * qfrj
gjcv: nfst * mwww
ltml: nbpf * bbls
gqgt: 3
wdtf: 9
mcgg: 4
jchv: 5
dvwh: 2
phnj: wvwq + cbnd
trld: fqcj / vrsf
whhr: dqqm * qjcz
wfmd: frcs + gtnd
bswf: 4
djrm: 3
ghpq: 8
frcs: rrnl * wgtv
vswp: 4
lrqv: rrlr + rpwl
ccdj: lcws + whbr
lzvm: fvtv - cqfm
qjtb: 12
sbdp: jwnp + mlgr
wrbv: npcl + sfpl
jqgh: fvrf * zrcd
tlnl: 4
mmtg: 3
mmzw: mnvd * psmm
wjfr: 3
nmhs: dmts - czbt
bhld: 2
tbgb: 3
bfbp: jljl + cpwc
jqqw: mhsv + dhcm
pfjr: cjrw + qwhm
lsww: 3
mzwc: zzsd * wjfr
scvq: cwfc / vrdz
hqpw: 4
qthd: 3
pjlj: 9
bszn: qcjz + prhf
dprb: 2
sccd: 3
cctv: jzhr / hfzc
vwpz: 1
zrqh: tljq + vcpr
llzv: tjrw + qhjc
ldgt: pvfl + lszh
zqzw: 4
jfbj: vtjs + zmzv
rzqj: 4
zgdm: flcs * lqjv
bpmf: rmmg * dpws
gdmw: 3
bldr: bpfw * jbdc
glbj: brrr * smbs
rvjh: mhch / hvdz
hpvh: 5
crgh: wfrs * jcgs
qfcm: rpft + fbct
fssj: 17
fjqv: cplh + ddmb
slrg: 6
mndf: 15
mtmr: 3
rqlg: 2
ltdv: hhtw * drjj
gpwj: 2
mhpm: 3
wtzb: dmtg + cwlb
smwb: 2
dzwr: prtw + qpwz
scdn: zbdl + fbzf
swrd: 2
mcbn: 2
ljtb: 2
nwdt: 3
tgmf: 3
pngj: 7
chst: 2
vngc: 2
bvnb: mtlh / swrd
zsrt: gflc - btsn
mqvl: srnv + lsnz
pvqz: qtjf + dlpq
lbrb: tgmm + brnl
sqdj: 2
tbfh: srfh / dlgz
vbqr: vrmc * fprv
rvmb: bdzw + mrpt
jzfh: wpfm / cztn
trdf: fvvw + pbsq
dfbd: 3
zbnl: 5
fltj: qqsn * bgnw
jvvv: 4
sbgd: gffz * nsvs
gzgv: gdzg * vhqb
lbgq: nrwn + vmqw
psst: 19
fdtt: 2
tljq: crst * rqqs
dnwv: htms / wflj
jdpj: nmjt * vvtv
gbfq: 4
twcb: bswf * pdcd
hznl: 17
shzj: mcjv * qmjw
prjj: qfrw / wflv
lwgb: 13
hmcj: 16
pvrr: llqn * drtq
dvqf: ntbd + zdfm
dlmb: 4
wjsb: 5
rdpb: wjrw * gqhl
nmjs: rzvp * nthm
mjpz: 2
mzdl: 3
qvbf: fdwr * jzbr
wclz: wzjg + djnm
nfjw: 17
qctv: 14
lmvl: 2
gcmm: wbdj + cgft
nrlf: 2
rjjb: 3
trmt: dbts * prjj
nqwg: 2
jbdc: rbnj + wmmr
spml: 7
jdmq: jvpd + fsfh
mwnd: gjmh + jghn
pdsg: 3
pscg: rjbs / zfwl
rbzj: 6
bqvg: 1
tgmm: 5
hzfn: bqqs + smbb
stgw: ghrp + qphg
lhtl: djqm + ptpp
hbwp: qmdp + fzvr
bmmd: 11
jcsh: 3
hwjf: zdln + pcvm
wplp: 20
rrnl: szqn + dhww
gbhf: mwnn * lqcf
vhld: chgv + smst
brbw: 13
mqqd: 5
zdzw: 12
qcqm: ztrc + ctsn
cjrw: rmrl - htjt
fdcs: 3
wdtn: 3
rzmp: bjth - ppbt
sjmh: 1
lbdt: qwfg * nprf
lctp: 3
zfwl: 2
drjj: cprb + zfcq
wzwz: 15
pnfl: jqrm + ntmj
lrbb: vrzb / twms
ctph: trth + shcq
slsn: 5
dgwv: 10
lpqg: hvpt + zvwz
zpwl: cbqt * rvtz
wzvb: 2
dtqz: trld * fjll
mgjw: 2
nwjq: djgt * nqmp
wmfb: 2
wsnb: 9
wjzm: lrqv * swzr
ghws: 2
fpzm: 5
plfl: smjq + tbtm
tfpl: 5
gqqf: 3
rrhc: 2
hjcm: 3
pgrw: bbjq + zlqb
pwmf: 2
stpj: 10
mrgf: mhsh * tjfc
rhvv: rhcl * jrfq
mslf: dvwh * sftl
stjs: 2
cvwb: gwzg - lqbm
fbfl: btvb + vqsj
swzr: 2
tlgs: fths / wmfb
qptz: 7
jdpf: 4
rqbc: 7
wffh: 2
tcct: 7
tfzp: tzjh + ffjh
fqcj: tbhv * smzb
nrwn: psqp * tfqv
wqjt: jdpj + fpbj
sfhv: 19
jtjg: 10
gwsc: jgdl + gncf
hqzg: svql * dlzv
gjts: 3
vsbw: shwb * nzmp
qjvg: 3
qllc: wgdv * wtqb
dlgr: 5
dpwd: bmmp + qctv
qsbb: tbmd + dttt
qhrt: 2
rhrz: 2
nzbl: fhjj * zgvm
lvfs: 4
smnj: 6
dmvn: mcbn * bfgl
mqjr: bwpw + wpvh
jfcg: 4
gfzr: dzhv * fvcn
fzwb: 5
jnmn: hzjc + wcjz
zhjc: 2
dlrj: mbvp / lgfh
mdwb: 19
rjsj: 5
fqrc: 2
nrgq: 3
fvvr: 5
rstj: nwpz + dtqq
qdbj: 4
bhwc: zjbv + pscg
mrfn: 15
bdjm: 3
zcqv: 3
hvdz: 9
hqlm: sbvt + rtqv
sqgn: sqdj * wbwl
cfng: 5
rthz: 9
wdww: hmdf / rqbc
lwdw: 2
qdlq: qsfz * gzgv
hdqw: 2
mnvd: mfqh * lcjn
fpwj: crbz * pndq
qdvw: tclq * cmlr
sfdl: 13
hnjf: jnlg * zzcw
sbdl: 12
mbzl: ncrd * dhgc
pbts: npwf - mrnc
zglm: qhdn * tpbg
dfpd: fvnd * rjqq
qnbm: 4
gnwd: sfdl * gbwt
dblg: zbsf * cmbj
ssrb: tjgw * dtcf
fcjq: tclc * wnwl
lpmd: 4
wbjl: 3
lchz: pdgq * zljq
rhcl: 3
dfjz: 5
znfz: hpvh * fjnr
jvtr: tvzn * wfmd
hvqp: 5
lzhb: dhvm * mqjr
prwp: pdwt * sccd
wgql: 15
wtzt: rmnj * zrnj
dbph: sljg * nznn
fmzg: 7
tgbd: 16
gfzt: sctr + bwbt
wrbq: gmmc + pczm
bjns: 1
fjbw: 13
bjcn: fffq + vpvg
zqcl: 5
nhtg: 6
tdpv: nhbb * jchg
bzgh: 4
htpr: 2
jhsj: 5
dpws: 19
jvpj: 4
prhf: mbld * tmpc
tncc: dwsm * wqhs
fwcr: qllc + rdrh
pwzt: 3
gvwp: ldnm * csrr
btdv: cczf * mhpm
rmgd: 2
mlfn: 3
nlvf: 3
tjvn: 16
vvwm: rspp + clqt
bwbt: 3
rwlm: ltdv + qbqw
zzcw: 4
bpfw: wjsb + mzpm
qtjf: chrr + hlml
dtqq: rpln / mtmr
lslf: 15
dpnq: vzrp * lctp
lqjp: 2
hnlc: 3
fwqw: 2
njwd: zblv + mdwg
fcch: rzsr * fqrc
vsgp: clgm * czfq
pmfg: 5
zvqb: 2
cwnl: sfhv * qvjb
ffqb: 2
vwbt: wvwv * dtgf
ggqc: 2
dnpw: 20
tvzn: spvd + qjqz
jbzg: 3
zlld: fmpm + drrd
cmlr: shbs * fmwh
vdcn: 8
zrjv: qvgz * npng
jhdv: lpql + fqpl
rwqs: rdtm * mqdr
dhcj: ffqb * sqfh
djtd: dbqn + tsfc
plsr: 4
rwjg: 12
cgvt: 13
tjgw: 17
btcm: 11
fjzl: 4
tmzm: 5
wmdz: wmdn + qhwc
zrzl: 4
cnsq: 2
zfps: 2
mdhs: bgpg + swzz
fnqj: 3
ldpw: 3
qfrw: ghnr + wplp
lqhg: 7
pfpm: 2
rbls: pmgs + ljhn
cpwc: mlsf * wwwg
jpcg: psjl + tmzc
zvvs: wlsb * wwcr
wdsq: gnql + qzqr
vjjr: 2
wqnd: dlrv + lzpt
hmbs: 3
dptq: zpnq * gcmm
qdnz: 2
swnr: 1
dmts: 11
fccb: 7
vsjj: 1
fwgr: 18
zfpg: psst * bmlm
bmmp: 3
bcdq: hrgj * qpwl
mfmd: lqjp * qgvz
bgpg: zdzw + pqsm
pwsq: 7
wnlr: mwnd / qsng
qzzn: 4
wwhf: bbft * bpgj
hgrl: vsbp * rvbp
gjsq: vbqr * jtjt
scpj: zlbf + mqvl
lzbm: 5
lltd: hhzd * mgws
jbbh: mcft * dqbf
mvbs: 7
qrfz: 2
szqn: gjts + hjcm
nfst: 3
dnvd: 17
mlgr: zhvt * qqsz
mtgf: blnn + bmgn
rwzc: 20
stbd: zmnm + zqtw
zhqd: qfvm + mzdl
mwnn: 16
rjbs: rfjg * zgls
scqp: 5
wwzr: blpf / cmhs
rbql: dvnp + cmsb
fplr: qsfh / wptw
mrnc: vpgn * nngg
nspt: gdbh + zczj
dhrd: rjvz + tcsd
zczg: sfvd + tqbm
slmq: zwdg / whrt
pdqq: 2
rfhj: 14
llnr: mwcq + whzm
htjt: 2
srfq: 6
cqrm: fmsh - jfbj
dsjh: 4
vgrp: dpwd + mrhq
ngpm: cwnl * jdsw
cmpp: 19
rvcl: bdjs * nspt
rhgp: 2
bptz: jnmz + pvtc
mbld: dhtw + wwpm
qlsz: pwzt * bqqm
lgff: 5
mdbz: 8
bqqs: 1
szmd: nsgp - pzdq
nrdc: 6
jttj: rstj / fmrj
lmnl: 2
gmdd: mzcf * tcjj
fhwr: 4
btfz: zmqw * bmgz
clgm: 2
vgbn: 3
nvdn: wshq + jndp
mrpt: jfcg * rwlg
fwvz: 3
zrnj: nlvf * rlfm
ptft: 3
npwf: bbvw * dwtj
ngzc: psfc * fpzm
ffgm: 3
qzqr: 3
wptw: 2
fzqv: 3
gbwt: 3
vgrh: rhvw + wwzr
vcjp: pnqs + svmv
csnh: gvfz * hzvf
jfgw: 4
fths: zfps * rvmb
fvcb: 20
ccfq: 7
qrbb: 2
vzrf: vzvj * bmrj
tfqv: 2
ldvn: 2
tgzp: 2
pchr: pzjh + jljb
htrs: 2
rszj: htrs * qlsb
slbf: qlqn + rzsg
wqnz: rnmv + snvh
nzjc: dnvd * fdcs
rbfg: jlmj * zgrf
ddtt: 16
bwpw: 9
zlgh: 7
pbsq: dzwr * dbhm
jcwl: 5
hjws: srdj * ssqd
ncqq: 2
nqgl: vhld * gccl
ghnr: rpmw * wtzb
djsp: qczl * zqdh
mbtd: lpjj * tzzc
svql: vsng + qldh
qrnl: 1
slzv: 5
nbjh: nmtv * jhfw
ddzd: 3
pcdm: 5
vbwc: 3
dwnn: ngvz * rrhc
rhhw: ncfz * nqpz
nsrs: 3
hqms: 5
chdd: nrgq * fjmb
znhz: cflv - hjrf
btlc: 4
fdwr: 8
bcpl: wzpw * rprp
hplq: 2
qdgz: 2
trnr: 7
qmbp: jllq * hvwq
cprb: 17
dchf: tzvg / sjcp
tmvr: dnhz * bmzn
ctjm: tjpn + srbc
nwpz: rhhw * ljtb
sslm: 13
rwlg: 3
jnnv: hdsj - gmbz
hcvc: bsvg + htcs
thfv: 3
zptd: szwg + vnsn
wlsb: zqcl * fdcr
gwpf: 4
gcdg: 19
lpjj: humn - pvrr
rnct: jzvm + znsd
wwtl: 4
hjqc: lsmb * ddzd
qtjw: 4
lgrc: 2
bhcm: vwzc * zptd
qfwc: zgdm / vswp
zqqq: 3
dsvz: hzbg * btfm
zczj: 10
pbhs: psmh + qlcc
mzdj: ldbt * ddcb
rgjl: 2
bhln: qccj * cwtc
qpjm: vmjc + sbnr
tcjj: cctv + jnwq
zpnq: 5
bbls: 3
crwz: qrhb * gfjf
zjfq: pdqn / wgrl
zpdq: 3
smzb: 3
cwfl: 2
clcc: 2
dwlj: 3
dpvs: 2
qzgc: 2
zvnp: swrw + fzzh
smst: qrjj * qnbm
cjrv: spsb * lvsj
bmzn: 2
cmsb: 9
brrr: 2
zmgc: zhwc * fjqz
qbbm: hwsm * vnjj
mndc: mspm + nrtc
gccl: 2
tcwm: hplw * qfwc
mftl: 7
lrcl: rwns - mcfq
tlqw: 2
bqwd: jttj - bnjb
mpss: 5
qmdp: 3
vwtr: hqzg + hfnn
dlrv: slld * pzqg
jzpn: wddt * bsqj
tpwp: 2
    "###.trim()
}
