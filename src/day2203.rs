use std::collections::HashSet;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut answer = 0;

    for s in parse_input() {
        let half1: HashSet<char> = HashSet::from_iter(s[..s.len()/2].chars().into_iter());
        let half2: HashSet<char> = HashSet::from_iter(s[s.len()/2..].chars().into_iter());

        answer += char_value(*half1.intersection(&half2).next().unwrap());
    }

    println!("{}", answer);
}

fn part2() {
    let mut answer = 0;
    let mut hs: HashSet<char> = HashSet::new();

    for (i, s) in parse_input().iter().enumerate() {
        let chars: HashSet<char> = HashSet::from_iter(s.chars().into_iter());
        if i % 3 == 0 {
            hs = chars;
        } else {
            hs = hs.intersection(&chars).map(|x| *x).collect();
        }

        if i % 3 == 2 {
            answer += char_value(*hs.iter().next().unwrap());
        }
    }

    println!("{}", answer);
}

fn char_value(c: char) -> i32 {
    if c.is_lowercase() {
        (c as u8 - b'a') as i32 + 1
    } else {
        (c as u8 - b'A') as i32 + 27
    }
}

fn parse_input() -> Vec<String> {
    let mut v = vec![];

    for l in input().lines() {
        v.push(String::from(l.trim()))
    }

    v
}

fn input() -> &'static str {
    r###"
WVHGHwddqSsNjsjwqVvdwZRCbcJcZTCcsZbLcJJsCZ
hngprFFhFDFhrDpzzQDhtnBJJRJZbZvTcvbfRCJfBRcBJl
DmptngtFwvvMmwmm
HFddrJnLdqtHBMQBmmVm
gbvNsbhsvQtmZTbQPT
vDshDlczcDhcssscwzQwslLJrSJLpqrrzpnCrSfLSnqq
pDGQDSpFDGzFDQSJqzDhjhQMTjTrwTstbTBTjTtLtbTMBT
zgzVNHHgMwMLbLNB
WRWPgdHCZccggJmJGzJmzGhGCD
sVJNlhldShpdpnnwVnwCwtwggt
WLFFcHWQLPPZQCgtnCgmbtbHwt
MPLWzRMMcGgRrWNDpSSSfDflMlTd
BBMZJcgBRjCZZzBpSvHQvbLvvHCQLQ
VlVTFwDTVGnfWSQPtsDPbvrpDS
wWdqhWlwGVfGwlfTVqFWfWWjzZZBJmMZMNdzZJMpjzNjgJ
FBWFphQBmDmpmMSpDWVcVcvsPcRbrjPMcMsr
HHtdnHnwNCHCTJRTPTzrbvVbcVRr
lHqHwlnlqnGCNGGmWDvvZfpZvG
mfVtmPtsccMmHcHCFfmhVmnpgZLbWPZqWnpqZbZWpgPW
zzvwBrzdQDvpZJfQJZJpLf
BrTBwRdNcfNmfStc
sTlhFLfZTTLcfsLlLDZflvQvRNqRJFNvRMRNvQQRBQ
CWcgwStWwCWWwvgNQvJBvQMQRB
wptGzbzGWVGSCVVlVlLDcVVsfhLTlf
HVnMVGwLLbsGnVsLnwLSBggMhjmgmgmhtmqhtgMhMj
zrZzJRZfzZfrPCrFcWccPdTdHHlvdmlgTghCtmtTgq
NFfcZWWzZrrHLBpBBGVGNG
HqFhhCBCBLmwwCqJCHFvvFdcprrrSSrjRFRjpgrggb
VGzWtQzGGQPVtlVNslVWsPdRpmcRrjpSzcrcbdSmSnSg
WPPGllQMPGmTLvLJBCwM
PvDWRSmTVvSvRhbZRpRpbjjjzM
GBFGHLglHrrrLgGrttbMjpbcpcZJBsBp
lrHgrrndgdNnlHGFQPMMmWPTvvWSCDQn
mmhQShhmhQfzNfTTlShbHJrRtltltJJtHlRLLZ
WscggNqwPWjcGcWWcpNcRJHHprZvZHrvtttZJpJr
jGjgcMGCwPNsGDCcszBfhhQQQDnFnTVVBV
mcGjrwzQcrZtQzZQDZcPssvPVVCPCVLwswwPBC
NJbqHddNSgdPWvvsVHVLPs
NqglNSlJFNSbSNdldNlNdNbTRFDrvRmQrQGtmDrvttQmmtDj
zzcBPnHBjgHjWJvbJQTvScbwcQ
qdspVCFqVqfFqLFCqtpTwtpTbSTbJpwBST
FRLFRCNNqMfdWNmZPBPZrHmm
VmtRRJmtrDrwhRcvPspltvgqtqsd
WGQBZzMMBGBGbZTTWWCMNSgggqnPlsfbqndndccglffg
CWQQZMFWdzMQdJJwJVFrwmmmRw
rZsFfGfNhznzsjhzZfVjGVvVdvSTSJHSDDtcmmmttC
wWpRBWlbWMWlQDvCcRSvJRSStm
LPlwWqbgwqjjcFshNf
lsppsGphmPrRQnvHdRpd
qBgjLqMjgjTLPnzHPrPRLnzv
gSMfNjNtttVbqBbtTSStjTqlhmlZDsDsbWZWFFFsGhlWPm
sPDPDzrGzBsGRsbwrjtSVvthVfQtQw
ClpgFZgNqMWCgqCpMNZqNWmNdtSwtljtVHQhtwfvdHtSdhSj
FpNCJpNcpfCpgNWPGBLcbTGTzTzPnG
mssNLCZqSqmNCHmrqHChJTjTjnRRnnqVnTTGngGTRn
dbwptFwQbvdtcvpZDcDddgzGPjTGgpPTRpzzzgRzTn
BwZdtZldDbrSsNrsrSHl
MLnFWMRWpnpnLnLCmPGTqQsFzBttTQ
SwNlDHNcddglSDBjrqmqGQqqmGtGGwszPP
vdSlNcrvvvnBMbBR
psZPRmTpRpgrlrDRBFgV
jvCqNhwnjhGNqCMqVgFHWtgHBrtwHFrJ
cGvbNcjvvhhjcvbQGcZdZSQpzdpmpPVpdZpd
drTHDdlHzllZDTzTQRQLsPPSsBbSjQdL
MfVVWmNvMnqNmVVpMMgfgMmvBFFfRRLQPPsPfsFLFCFRSFjR
whMnNVnqWmlllHswJTZT
ZSQTTLLlTsbmmDZlmNQSNFfPwHwqCjCCfjwFPwfwLr
MctMJMBVttnhJcBBVctwRHjHRJwJwjFfqPfRwj
vzqgqhBVzzTlZmmTlN
WgvlHJFvljvdBmzcvcwpmchc
TQqZsTZttLZbRZsLLMzzppBmNShCmBNTcNCN
LPMZsMZLMQVgglFPhFHlFl
qsBCPVPqVbwfnMQNmZJnqJgR
hHdrvvLWtvtjWQnZJTMrmpTZgN
DShSShLZdFGPGDPGsPsG
qRBddRzFFqFqHnNnPSnnmmSpgpJm
ssZDQMvvMwppNJWRDRpW
MMvwlsRMcQBjcLqLBBqc
ZGHpwFGvwpHrvfFTMtDfccMjntMntc
RgSCLRLJRSRSQQqJmTDMPMTtsJjnclBjtj
LVmmSSddLCwVHDbzDzZr
psgWdsBjnnJjbZWQDDLNrDcrLVQjLM
zPSCCHqCfqfmWNMcrVSLRM
TPHzWPFTGztqTGgdJdsssvZgwb
gcFgBChcClJjNCPb
sWZdZdrSmWmSZRwSmsvPlsTtTtNMnnlvnJJv
GSWrHZdGQpRrrSGmpWQmQfLfpVzDfghppzBVLBlBqg
BFNqFzBNhqVwmTtsqVst
dMwMwMfCMWbDtDvDssCC
ldMwMSHHMMWJpRpPLLpBzPZjgnZPhN
WczRJhcWggVBdzPPLnCjdvjm
lSpSTpTSsCCmmntNdp
wSFDCTwsGDqQqQVWWcJw
RqPqhDGBhRDrrhBFmPmbgssZbwbgCbwsmZsQ
nCtjMppjfTpjJJfVZwtzZtllLZwLss
MHfpMWdHpSCSfnSTJWhDDFDFBGqDGvvDBDFd
MCCGMCSHVGNTspVWQznddndg
rttLtvRbrhLZrbcQdJnnQdfddsrggf
BbRqltRtHsNNllNC
ncFpcsLLdFmWlRmnllTR
bMMVzVqMzjNVDblmRTPGlSmmPlqG
gNDDJMVZNCbNJNDNQCbZCbscvBsdBvrRHfcpdQpfFFff
VnWFbZvFbHWhFjZWVJZJLZFWTttpMCspQTTzQCHpgQMgztzT
dGcfdNdGrlRlBDGNSllfBMspgzmTgtQQgztMtzpmcT
lBNdqRsBRdfPNrLPFVVPVJPZZvhj
TLWgggJzwjgWgjgGnnmQnzQfNNNQsm
SpPbBlPBMlvFZpbbBmQGsmCJmCstsdNGBQ
MhSHhZPrPbvSFrJPpPMSbMcLjjqTLHDRTDDTTLDLTqgH
fprRRbbznFbcQVPDdQPdFV
LTvmsLmcsHmvDvSZDZVVSS
jWtmLccssJTLjHmLWWJwnwlBfwnBbllpCBnffbBr
plPBWzbnFLPPtGqMMwlMGwmS
ZQjDHjrQjdjVFwdMvCSfmwMqdt
DDhhrRDjQghHJjhWBbgbTccbsTzpWF
vgCbbwsTbWWWgwBWDGGDqtPGtMgGlFMH
znrznJNhLSLphRRRDlFPMmpFPjjHtMFF
llNcSQVSNcRbvCwwWcTdwZ
qpnJbnRRnJhRFhFHRgQSzHlSRHCCCg
fMBttBvsBjffvsQTtfGTWlCWsSgSmHCzZmLlHgzZ
ffMdjrfdwjfwwnhJPFchhqwQ
NCVSTCVCQCCRVDQSJsqFPsPNspFhhsgjPh
btvtWtcWnpgmFhjmmt
cfnffBfcWcrMdbvMQJDDrDTDVCpCDrGD
fZNhBWFSlFQFjWQTTldHgCwvTvqqdr
zznVzCznmHvnwgdH
PMMbCGPMDPcLbJhFhWhBhRScQZBQ
WQMrDWGHbSWHMNrTQRhghmgPZccmqDLwPqPg
svCzfpdzzdsnslCsnPZcHZPlJcqZgmqPPc
nntVpdpVsfjCHzvnsCzRTBrtWGbNNQSMbTNSRr
SnpDQdBqGpDSBMfQGcMQBDJPNstvJcWNsPJCtJtNRWPC
VrVHrhTHlPHTvvNtbhNRNswC
TzlFHHmrVlgTlTGSzGqpdMGBPQBS
zrCDnrDVCnCgnrHgGDnVVCZsNttQZmjtsmbMqGqsjbqj
TlRRWPSwwFwbSwTTTpNQQqNjqZZlmMMQQt
wvbwbRTLWdFFwvRBTbvTTRzrnznnJrDDCzBczBfHCJnz
SvTdmLNNNdvTBmvmLvSvDpgczzjfgjggpcjcNPzD
VJHQsJVlHpjjpzsjzP
VRlJbJQrVbVHJJPMhBdnBRCSLZZZnvnLvv
tMGcpGtMtLtsCGspLzNCBBmwCzQRzBBRWQ
hdlHFllDdZgDbDDlDHTWWTnzBBBvzmNHzwRz
FSddDlFRqDFqFSdPVqdhcfGMsVtVfLjrfGfjtMcs
RGMWnBMWfCCMBHTDptJJgZStRPmSRD
bqzFqjqcFLNLZZSmpSBgZZ
rFrQNbNBlNcbrQlNQvvclMswTCTCnwrwHrWGsGCswn
WLhJQddCQwRNCQNHczHNzMvZcZvcNc
SlSpSlrpDqnbqDjlGjGGljTjMZZPPMMfVPgfHMMVgVvqfgcw
SbGsDspbbnjTjBldCFmLwFCJLBmtJB
TMDjMvMqMvDTzcmFCgrJCr
ZZZJSZWVBHZWSSZQJhVhWnHJwczGGwGcCCFzwgmzcwFgwVzc
pLHNQSnNJsMLRJds
TsLZGwdsDFWHBZJFfZ
mqhRvqrzJRbmzJBFfgHHgWgHrrlH
JvvNhJmvtDdsNTwdLV
wwnSVSmwtbstznwgbzzVMTNpTNWdlCSlSWTffWNCSN
cFvccLGFGvvGHZflnNTpnZpZcB
GPqGDhGGqrDhVRgbnbttPmgs
rzSZJScLrcBLvjvsqMPZvjQl
nnpDqgDqFTgwqHHvMHvvvTvPMM
GnqCGpDqqVhccLmrmSmCRL
tJSTmdfddDTDJCPmbQvQLHvqqqbrbvlP
zWGsjcwwGGcVVjcGWcNjvNjQqrQtNFFQHHrF
RZnRVsswRsGWcwVBZVtBRdDJgCffTgmgfnnCpfTfTM
FnCrzhTrNPrMcnhMTnZZZNPwDPdbDmdDtwjdtjbmQwDt
sBvWrpppvLBsLRVBfHSfbbQmbwSjStDSwSwS
LVRRRJqqlHNlNTChrhMG
WNsfsstMvtMvNNGPZwmZmqZPLWZcww
rDCdDRCDFQjSVLcmZcDq
bBBHqTgBbQlQRCQFbgqdhvshGvTJMnfTtnnThnsN
VwWBTNQcVzDtrgfrtzzt
LLbpShLGvlbCmLjpGSCSCpvFdrgdddcHtrtGgfqHcDHqrd
pmvLmlpmjbLbpljJPBBcTBBQRZBBVJRZ
cVTcVTNvvghNhvggPPgtCVSpSQmzCqZDRCmDZDZS
dGJMWFsFMFWsnlzRlQzlzqpzlZzD
HdLFssFMsJbnbFjqbhPgjNggcrhg
LLVhQCTvRvmWlCppQfQQjPrwszNsfzNz
BZSgncHgnJStJHJgntMWzGsrPqGwsfPfGPwwwZ
bdBdJMBcShWCLbhWVC
vjdpGNGwSNCTwwRbfnWgQMLjQWMnLQ
DcmFPFtHmlcgpqWDnMbDLf
FZJPtcprHtPPHplZHPZclwwGBSZSvSwCwvZzNdwvvw
CdJLJCJPWPWcbtzJtqJzFrQvBhfjBBvjjvdjpFjr
sBRgsZGDNSBBRGDwphrrrThpHpgHvhpQ
DwDsGBDNwGmMNlMlMDSPmztJVCbVCCWqPqJLmW
LSTMgDSRSMHbMDWLHSvDScwtCGqGrjGrcLftqVGtVC
hzJPmlphCGrCwVrJ
zhPNdNnQZBZBhZnNZSgMWMDbMHwWSDWNDH
rcdvvcwvrHrMZBjHSZ
sDtWblgnltsDFlgFqltCCVQTMTgSHVTfSQfSHj
tDtRWFpFbWWWWNNDWsNqWvmzvhzhzGmzjjGvLwJmpc
nFSSnnbhSfgLSSnVjdjfHMgfMzGzmqlNGGmTPlqqTzTNNzlT
pBZsJJvccbBmlWGlNb
cvsssvZwsDwrDdfFgnbDfVbgng
mWRNWNCTdwdCwhCddbWWmhsZVgJQJBVBfsBsJQLQBLJb
qFFlGzFtjjcqzHtFtlRfVfsZfQHVfBHQRHgf
jqGjtcDnGnPzFRlzrnMdWrrCMMddNNWT
MHWCjjGMcHhbhPDLphHQ
nRVJrtgssdLgCppvLQbg
RlVVZNVRJlsstldsBCNlczfjjSZmWTcmGmTSfmSm
RTHqgTgMwgnGTRzqTHCGfdFdfhmBrJrdvbFJMhPB
lNZNNNLttLWJBPBdZBFmdZ
SppscpLVStclNPWtCczqnQQwHTTgCGwq
hSHRCbZRSZhbRZBctnMVjwwtWtwh
GrdFzQrDdJstjcWttwsF
drPJLDPGPvDvzrJPQLdDHpZlwLgRmwCHLpwgSbff
zMSSnCtCdSdCtdfMdHMdtVBDjhWDHBqbTVVBqhbDjr
cPNhFFNRlNDlTBqjlTBG
RvmvRpPNRgwgPvFwhmdCssmCzdMshMmL
tttjgrpTwmCgCwgwrrlrHzbzqqFNzdJqqZnddJwNbh
cQjMjPMBfcLBSjGQBndFnzNdNnhzzNGFbF
sSQPLMfVPBVSfBMvVLSPfHCttDjCDRRtrVVglgpttD
vdTvdpBvcTPdSSvCLrCCDLDCQGDl
sRfnFgmFRMVsnqgRmqzmrrDBDwtHlLHtrLCDGL
qRMVjJgRFnJfMssMsgZScPJpZbPbPPWhBZSp
ZJgNJhGZglMZZFDTPSNqFSqTSb
mwdvwpsjrcjBvpwFrvbHcDqbWHRWDSPWDHSR
CsvpsLLjFzhlLGFZ
sDNQrMrNfrlQjJRgGjbTllHG
ZRhSnWFVSwBtFRBVvVgHgbzjgGTJnngmGmHC
vWZLShhvZLVtSFSLqwVrQdqpcqMDddRNQMdsNP
hQhSQbbwtHzShwhSQPbJRsLwRCjJmDCcvmqCcs
FNdBTBTNMsRqqCjTjL
GNdrdMBVFShhSLSGGL
cZzcCmjjcvdzdWqgWTZgPZgZhh
wSwVGSJFTffgJTNh
FSVpVlBMShzbjzcpvp
qqlblClRbnTvqTmRqlmnTwrdfdwFFNrngfddDBrNtr
PcLcQLMVLGMzHLMchhLcjLFrrNrBfrfFNJtNgJDDBNzt
sSjjGcGQscSVSMjHVMSVPSQsWmCmppZCmtCWbbWTlZTqTl
qWlVJmDJHWJHVJlsdVTdhbFNNgFhwhhhFhwwZg
npjnvQpStCQLvBpPnvtBtBpGSGbzbGDggGNbgwghzZNGGN
jBvLtvjnrtMDmmDRTTrsWc
pmwdwzJtFmmlpFsWwtstJPGgvNgCCLWCvPgNNPQCQv
RfbfTRBnRGQvPNnncc
ZTbPZSDSBfSBVSbbBRbbbtrFdtlFmVsswtFwzdpszw
hVphQcmdcWWprWWhChFQBsfHjDTTBCHlSsTSBgSH
vqBRqqzbqMZPMwSTDjJjlHDllgHZ
PMnMLqtMnntQhWBccthB
vqqvCSvHSSwqvqCddnvQFmNbVjbJVVmGNNVHNNlH
pggrhzWgptWhZsmVlFmgNNVNbj
RzpMLLhhphtzrRrSSbQTBQwSTDBwQM
DSFQDlDFRddDHQHQtFlDVsVMTzrMCLSWZLZffSzLWrfCJz
jjBBvpgmbppBPbMwBBBNbbZWZzzCCTzzZgzWcJccLzWz
bvPwNwmpnBNhPmqpPvnwwNmtRQGQMdQDQlsGVVGhRlGFsl
SfJJwDJgpGdSGJNSTwTVJDRbWWfLtCWCLtRLHWrtbWBf
cQQPnFhjjQlczhqllhszhqsQRWnrbrHdHtbWrBWBbtvvHBrW
qMqqqqzFFmPjmmsFjmzsmhjcDGSZTJgTdpZwZgwSZVpMTNVG
czrcHMcMJtCCPnpFmH
DwGGlvLljGmDRdwLdLjfhtFsssnFVpfttpptsnFPnp
TlRTghTjwTDRTDlZZQgWMMrMJMSZmM
BzdNzNdgNNPfgdNsdQdNvVMLLVQVMcCRCMRmvCGc
zHpplwwZrZlqlWWrpZwqlHhLvqMCRDCGVmLcqGMVvCMmMD
rWrjwWwHplZbwpZtHtJJbgfFTfsNnBbsfbSdTzgB
jPRRppDLDGDTLLggMMjpLTGcrJWHsttJfwnWrMvrJnvnrNfJ
blqbzBdzmhhbQWnsNHtJvfssfd
lhFhzSzzSZVNSlVPgDPCPCGTRcGR
cqWcNWffPftvsvfpqPtZsBzrbmbFddBmbcLbdDHbHz
TJgljTnGgnLBTZbHdBFz
JgSnJwSlgGJRwMtfPtvfwsZQZZtv
hHhPbQPTwsdwdHqtgttjpNfjDt
FFlCmSzRCCmlzzRGCFNvRpvjvtZNZqsRfNRg
mVmsFMGFzJFBwQTMnMQndd
QQVpQGcVdGmspHHLtbqfqfbt
JvZTFDFzJzhFCWCZZDzWPBCJfLbnnwLqttnsHHNPwtbHLwjn
DssTMWvvvGMcQGQGld
sshRHZSZRbSZHhBFBMpMWpFgbbtb
JfjTjmwwTPvfTNPTQlmFFFqqmFMBBqFgFt
vDTvJffQTJjJvPvTNSHRzhCsShRRRDtZHz
NFLsRDNNDNBDlgPPgBglQlzj
HJhdZpfJzlWQjjHw
ffJTppZZqTNlGnNsMG
ZMrWcWwqqvPZMndGdqlnnDLnVT
HpCsshCfpFfHHJDDSlSVQQGGflDQ
zssNzRJFhjNHNNHpJRwbwMMzWWtZPcbBbwbG
HlNHHLHsBDRpHLlsHRlJnMhfWZMRnvCCCnWhZj
wtqSmQqttzSSQdPmmwZhChJjWJjPggCZCfZJ
SSwtTbTQmbtdqmGTTcfqzDLHFsBLDGLNGrsBHFGrLB
FFDvWznMWWMrPnPnWPgsmgQbhJRslHbwHwVVsVHjBsHb
ZtSffffpdLqpSCLfCNqfLqLCjHjHbwhpBwJllHlRVQllphjj
ZcNCtcGSctZScqfNGScLNcczPJFmzmDzGzWnrWFPFWDvrM
DnTPspmTPsTCDQWRZzZzZRCRfCfHfh
BNcqTBcFgbVchVJhVR
dTwdrBrwTSPPWnnmSmsn
pfbbDbHpNBFmQbpNNBSlLtlDStSdSPJLtLJR
ZcszvwgVCZswFzVTRTlTlRLgRJSWJR
jzZvVwFjcjjnwvzwZcjMpqMpbGQbQmhhHhmfHQmh
hTbddhQCtdNmdtwtdhTBbCddRSWscczwcRSWLJzcFJzDsFsR
NflgfPZPcgSLJcWD
lPVNZMMMpZlZZvfrMvpbQHQhtbtqdTQHthrqhd
JlWSStwhWJSRJpJvJBjTwTqcwTsDjsCTCB
dqFzgFZGGQNVmTcCrjrzsBrB
fdgLFQLnPdnqShRMPhlJMpWW
TMPcsPDjdDhsDcDcTTTDvdvghBNFGGtmNrSrgSSBGNtNFg
CVCbJqlRVVWWpRqRQZRWVWJZBtmSFGNmggGmtmmBFbrGMGMt
JRqHVJVCRLZWTjMnfLTPcfLd
TRTZFTTrghrZVhVWdWZpMmbzbdzBmtDpDDzmzB
wcsSSsjfPfGPqQwqsQcfJJCtJGpppCBJzCbzJzCb
sPjflcwljfjfvqNcTZTRhtVWrNrVLnrR
rVLLsmwmCWTmsCTdwQrdTmqWDjDHjNGNPbjDBPNDNsZRDBjH
cFcSvgJvfhfLnShtMJtPHRRvRbBBGBPNBHPbND
hgLcgcLpJSMwzmrmzqQrmp
    "###.trim()
}