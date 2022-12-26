use std::collections::HashMap;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut nodes = Nodes::parse();
    nodes.mix();
    let order = nodes.order();
    println!("{}", order[1000 % order.len()] + order[2000 % order.len()] + order[3000 % order.len()]);
}

fn part2() {
}

#[derive(Debug)]
struct Node {
    name: usize,
    val: i64,
    left: usize,
    right: usize,
}

#[derive(Debug)]
struct Nodes {
    nodes: HashMap<usize, Node>,
    zero_name: usize,
}

impl Nodes {
    fn parse() -> Self {
        let mut v = vec![];
        let mut zero_name = 0;

        for (name, line) in input().lines().enumerate() {
            let line = line.trim();
            let val = line.parse::<i64>().unwrap();
            if val == 0 {
                zero_name = name;
            }
            v.push(Node{
                name,
                val,
                left: if name > 0 { name-1 } else { 0 },
                right: name+1,
            });
        }
        v.last_mut().unwrap().right = 0;
        v.first_mut().unwrap().left = v.len()-1;

        let mut nodes = HashMap::new();
        for node in v {
            nodes.insert(node.name, node);
        }

        Self { nodes, zero_name }
    }

    fn mix(&mut self) {
        for name in 0..self.nodes.len() {
            let mut node = self.nodes.get(&name).unwrap();
            let mut move_count = node.val;

            if move_count == 0 {
                continue;
            }

            if move_count > 0 {
                while move_count > 0 {
                    node = self.nodes.get(&node.right).unwrap();
                    if node.name != name {
                        move_count -= 1;
                    }
                }
                self.move_node_after(name, node.name);
            } else if move_count < 0 {
                while move_count < 0 {
                    node = self.nodes.get(&node.left).unwrap();
                    if node.name != name {
                        move_count += 1;
                    }
                }
                self.move_node_after(name, node.left); // move one more to the left, to insert after
            }
        }
    }

    fn order(&self) -> Vec<i64> {
        let mut v = vec![];

        let mut curr = self.nodes.get(&self.zero_name).unwrap();
        for _ in 0..self.nodes.len() {
            v.push(curr.val);
            curr = self.nodes.get(&curr.right).unwrap();
        }

        v
    }

    fn move_node_after(&mut self, from: usize, after: usize) {
        let node = self.nodes.get(&from).unwrap();
        let old_left = node.left;
        let old_right = node.right;

        let new_left = after;
        let node = self.nodes.get(&after).unwrap();
        let new_right = node.right;

        // finish - update pointers
        let node = self.nodes.get_mut(&from).unwrap();
        node.left = new_left;
        node.right = new_right;

        let node = self.nodes.get_mut(&new_left).unwrap();
        node.right = from;
        let node = self.nodes.get_mut(&new_right).unwrap();
        node.left = from;

        let node = self.nodes.get_mut(&old_left).unwrap();
        node.right = old_right;
        let node = self.nodes.get_mut(&old_right).unwrap();
        node.left = old_left;
    }
}

fn input_test() -> &'static str {
    r###"
1
2
-3
3
-2
0
4
    "###.trim()
}

fn input() -> &'static str {
    r###"
-8023
4822
4250
-6738
3764
3108
-7753
-5309
1238
4873
4965
-1325
4263
4527
-7204
2112
4965
-951
-5522
121
6050
-3218
-39
-6167
-2840
-6724
3214
9673
9088
-4225
-2356
8319
-3077
-6953
526
-6838
-8191
-992
7283
-3930
7037
-950
2864
5691
5879
2262
-6195
7315
788
4012
-1184
9516
5797
1804
411
-4212
-8684
7399
2589
6604
8988
-7867
-8723
4407
2869
-8451
9449
8478
8311
-1449
1296
-228
-7569
182
-2073
3570
3364
5487
-562
-9317
-8935
-7435
1812
-5884
25
-6608
7146
-5062
-9990
3995
-7287
-7114
3925
-2492
5397
7850
6050
-900
-6219
-4800
-5892
-4035
-5988
-9456
-39
272
3962
-6147
-9725
5640
1173
-8680
-911
281
7378
890
7027
-7184
-1364
-790
3175
3951
-8565
-3879
1626
-1344
443
4717
1615
3393
8565
-1462
6265
1090
6497
-3291
4722
2444
419
4809
-5004
8918
-976
457
-1964
7745
-9437
5448
5621
-9477
-8743
943
-7532
4549
-680
-1907
7654
5421
6433
9182
1529
-8114
3151
4345
9180
8477
1716
-6844
-565
4135
-931
-9295
8994
-2790
-6228
8953
6607
-1228
-3602
8153
2635
-1396
6676
-7341
9904
6483
7026
-1424
9653
-3819
-6082
8511
-5625
-4542
3831
-6999
-5638
-4988
-4431
-5161
3429
2572
-732
4012
9727
-8945
21
-175
-8622
6786
-5465
4992
8885
8786
10
8152
-1102
-6223
4827
8391
2157
2564
-2744
5178
-4775
8952
-7928
8439
5791
3893
6177
-8156
8031
1162
-3529
4826
-7618
-9539
4352
6555
-9610
-7473
-155
3393
-2274
1177
-4060
4410
4756
9339
7698
8232
952
1891
-9647
-7115
-4131
2272
3210
3721
9545
-5152
6410
846
-9001
-1610
-8632
-3023
-8205
-240
7815
-1559
-2085
-7735
526
6608
-9284
-9136
4857
1953
1561
5585
5686
6243
5568
3548
-5206
6105
-5450
-1232
-4631
-6367
3621
-3078
9908
1164
-3729
3094
6815
8922
-113
9993
373
-1823
5805
9903
3110
-9048
7645
-1422
6698
-3824
-9675
-9073
-4488
7110
-4512
-9447
5580
-377
-7945
-5425
1475
2285
2153
5567
1427
-3603
-3529
9078
-2182
-7558
4366
568
343
-5507
4725
2080
5025
-8079
8507
6022
6752
5808
-4101
-9759
-5653
8434
3869
-6838
616
5647
1700
1661
5181
-6501
-3819
-9814
1557
7258
6457
7008
6435
-2659
8524
-1162
-1045
-9354
-410
-7513
9423
6658
-634
-3602
-2142
6783
-7251
2704
-3923
3474
710
7541
-1128
7730
6375
-3572
-8286
3783
-5407
-5722
8221
5021
-295
-959
-736
-1151
-5370
-8900
9790
9211
-4812
7315
-6530
7772
-1970
6371
-1840
3595
7004
2059
-6223
3353
-991
-1390
8719
6821
-3520
2162
1385
-1410
9280
-5821
-4530
-6407
3135
4152
-6240
-8847
-2244
3783
4209
-9734
-1165
-6031
-9857
-8032
3026
-4780
7023
900
2175
-4338
8439
6002
8088
3079
-8935
-8032
5674
7692
-8264
1126
-6736
2840
-7922
-5625
-2391
-8007
8932
-8007
-4304
-4736
3679
8077
-9866
2877
-8386
-1416
-3034
-386
9875
-3739
9650
-3594
5146
6251
-3271
7102
-7753
-3996
966
4504
4253
-951
5683
-8041
6006
-6686
4074
9988
9560
4132
9433
-7672
-8619
-3827
-6251
208
295
8790
1864
4642
8354
-2453
-3063
442
-8948
-5792
-122
-10
-6018
3121
4807
-4615
9439
-5625
4248
7685
8262
5763
7847
1981
-4046
8724
8896
-6017
-5997
7710
-2190
7850
784
5860
-4956
-2264
9475
-314
4858
-4979
-6696
4068
-6510
-148
8821
7468
6010
-1822
-570
-8966
829
2726
2263
-6694
9913
-4763
-6287
514
-2957
1372
-4569
437
-1582
8655
-6208
-3530
710
-1031
1530
4503
-9933
6141
-3485
-175
-23
-4775
-3611
-8077
9734
1301
-6765
-2663
-6794
-7908
-2054
-8055
-5321
1820
-5381
9888
-6046
2680
6560
-8550
5314
-9736
-4294
-7228
-5522
5888
-9563
9850
3353
8337
-6161
4167
-3388
-6588
6018
8628
3587
3071
-4565
2192
-3794
4020
-1000
-9901
7777
-4198
-5480
8011
-9529
2372
6555
-8203
578
7580
8921
9269
-562
319
8019
7529
-7681
4057
815
-9536
4725
-3198
2118
-3800
3813
-6496
-5722
-1390
-6733
7277
5508
3619
9650
9040
6634
2522
-8032
-5505
-769
6469
-6234
6622
-868
6416
-9377
6357
3665
6788
8037
-4794
-1382
-6509
199
-1410
-9991
-9323
-9095
-7841
-9399
3842
-7077
9731
7546
2160
-7368
8283
3151
5570
-1544
867
1383
2728
7106
224
-7127
8251
-1439
7966
3390
-7681
830
7248
-261
3548
8752
-2335
9882
1223
-4446
-8350
-9360
9904
-1490
-7095
8257
5580
-9615
-8570
-571
-680
-6886
786
5597
-3850
-7179
-2391
9556
-7841
1608
-9343
4910
-5213
-1308
-6497
-6390
-4057
7162
4407
-1232
-8703
-899
-7831
-4245
-7793
-1896
3176
-8219
-8162
-8579
-4710
-9209
661
7835
-3572
-4352
4249
9052
-8260
-5381
-983
-9358
7706
4288
3402
7357
-3809
662
-6796
-7221
9540
8915
-983
-6017
-1756
5991
-6851
1321
-5381
-3411
-9238
8096
2261
208
-4561
-7145
4154
1926
-4461
-6784
-7149
7098
-5110
-3128
7924
-8459
6328
-1473
-5425
9640
2151
1414
-5985
1452
275
-7017
-3274
5113
-1108
-4735
-6712
-6510
2492
3870
6283
1436
3184
9168
6674
-5944
-4402
-1408
8536
-8703
-5295
-1405
982
6027
8819
-2985
-6765
-9886
-170
1402
-7541
7120
7649
6055
-6622
6105
2039
3637
-5427
-5770
-7003
301
-839
1474
-8669
982
9187
-4142
-5836
-8480
351
7684
9857
6371
-4376
-5964
23
8230
9820
-1649
-8670
-8926
4778
-5641
-826
-2632
-5080
6602
-9866
-4979
-8607
52
5787
-9009
-1132
2112
7079
-8828
8148
-3391
910
5083
-572
-6578
3811
-5190
-2639
-7272
4952
-1214
-3582
4307
-3100
16
-5763
-6029
-2744
-1970
-6971
6940
-1344
-816
-6435
4354
-6930
-7032
-720
2522
-4279
9843
-8156
-8621
-8621
-2065
-7498
-853
2163
-1556
-2638
-5309
-1248
819
-994
8407
9483
6537
-4564
7075
-4073
1751
7660
1477
41
-8908
287
3214
1587
2807
-5266
1955
8612
173
5239
-2033
7700
-5548
3235
5561
-9901
-1472
4265
997
-1214
2739
8434
8314
-301
8375
2597
9550
1424
-5381
-7821
-9112
-6953
4398
6096
-2173
-4225
7598
-8140
7992
-4237
9019
-8832
-5612
-6539
-8804
8990
1416
-2025
1398
897
786
9709
4964
-3566
1998
-2321
3452
-5038
3515
-9026
3932
6777
8507
5608
526
1685
6442
-2139
-1907
7347
6638
-9008
-9045
2364
1953
1529
1561
-1670
7081
3626
-272
-563
-9380
-8447
-3321
9374
-1416
5645
2067
7036
6532
-1433
-5710
-7669
-8330
3071
2782
-9257
-7821
9470
-1289
-760
-4737
-2138
-5019
6518
3353
-8177
1108
5516
5870
1976
-4257
-4245
-1265
8701
-8051
-9444
-7105
-8386
8398
2825
-6296
-2801
-4037
5521
-3551
2515
9150
1506
9329
-7099
-1912
9177
2059
-8542
9361
-5571
2999
-6973
4371
-1705
842
-9391
4244
9894
-1767
5181
9599
-4172
-4972
9343
-3580
5444
2859
9957
-5387
5092
-9364
-4553
-2576
2887
8011
4662
-3632
-6739
-2145
-272
-3077
3190
-2016
9602
798
-9700
-536
3079
1540
-9335
-1786
6989
9597
4644
-849
-3665
-9047
4066
3504
-9547
2153
-8052
2563
-3874
-6102
2079
-641
-6188
730
9906
9598
-3814
-9259
4899
7054
-1894
-5402
-4631
3047
-1035
-1364
-8432
-8622
-1541
-5612
-8227
-3031
-4600
-6755
879
-8421
-4349
-8946
7702
5190
6846
-6357
3489
3151
7194
1030
2511
29
233
1127
-4385
6091
-175
2423
-5953
1877
6547
2596
-1490
6745
2582
9867
-2994
1447
-5630
-5792
7729
-366
8656
-2221
-4299
4044
1009
3081
7540
-997
8818
-9714
-8286
8650
9952
2739
3937
9874
-3385
-1224
-8260
-2197
231
9234
-9960
3110
-5235
8216
-3355
-352
992
5652
-330
-8061
-5702
6585
-8172
1651
2097
-7357
-199
6098
-2949
4837
9288
7486
4107
-3240
3891
505
-3865
-3955
-8966
5891
-7237
-9435
-7198
-8966
952
4739
4786
-6844
-7512
-4005
8845
2080
3089
-5634
-143
-551
-1666
-2276
2465
-5373
7242
1416
-1489
9434
-9632
-2234
2794
3796
-5161
-2500
290
3089
2006
9532
-5718
-7196
3712
-5649
-3067
8665
569
-2286
5021
-2687
272
654
1635
-5690
-5355
-8520
6488
-6586
7417
2144
3089
9540
2998
9481
9798
-4153
1689
-8421
6438
2588
-5146
-1683
-7443
-8599
-5975
4142
-4395
-8247
-2103
-9021
-9516
-7205
-9659
-3204
914
-9857
3037
-5694
-4245
-1559
9180
3105
8545
6477
7316
-4903
-9534
-5407
8585
4591
-4697
-2501
-3047
6014
-3619
10
3628
7621
1314
2541
8328
-9309
-4306
-3360
4982
-458
-7666
-3758
-5432
-9128
2508
-1803
8738
5176
9358
8513
-8275
-2894
-250
868
-3258
3079
-4168
-8032
-8651
7113
6233
8439
-7394
2450
4565
-2185
8292
9064
5327
3973
4956
9178
507
-31
1824
9478
-9675
503
7672
2432
-2403
9119
-9695
-1972
5987
2636
277
-8964
3912
-8156
4249
-9667
4738
9908
9088
-651
-238
-7598
-2607
-4341
-2030
-7401
-1393
8045
3558
-5612
2989
-8966
1965
-7164
5289
8767
2180
-8894
7380
-7037
-3077
-8255
2726
-3352
3969
5992
7107
96
-8162
-2341
-4979
-3967
3338
3595
7023
-8857
-8681
-1627
-3902
2780
3000
-1029
8239
-5958
-2985
-5297
7877
-9600
-3149
-1274
-4812
-6202
7379
-5112
-9783
5791
317
-5104
-4789
272
3185
-4926
6518
-7691
-5792
2237
5593
4109
-8423
-6775
6184
-6550
-8896
-4564
-1646
-9632
-1342
-9288
2148
2939
7015
-403
-4055
-5722
-1454
4432
-9085
734
-9866
-4447
-5202
-2065
-8579
-8392
8018
4710
-8945
-1743
2153
9906
-3968
314
9874
-3851
-6791
-756
-7469
-362
-1927
-2846
2413
5683
5735
1974
1213
-7631
-9456
-7024
-9604
3627
-4289
3415
5625
4288
-2380
3202
1147
8045
9943
7818
920
-564
-6412
-7911
-6745
6212
3621
626
637
8682
-1509
3360
-9867
169
4160
6879
-2572
-410
-5195
-606
7906
3770
-6840
7532
-2103
-7567
9088
-9536
-8125
2905
3730
-5172
9113
5400
4816
-8118
3228
3779
-5695
-8215
-5981
8600
-8542
-7466
7564
-7416
442
2559
1161
6118
1589
7137
7756
6098
1224
-4245
-213
9843
9072
423
-749
-4937
-7323
1955
-8888
565
-5171
6082
8955
-1777
-7872
-6588
-273
4710
-2724
-9655
-9541
733
-7172
2614
-4027
-5104
448
2794
-6724
-687
3817
8487
-9109
3409
2907
-4429
-5080
662
5279
2304
5201
281
208
-9354
1243
6992
7673
-4174
2180
6807
8953
5279
-1422
9011
-8174
6036
6527
-5580
-326
909
-6768
7080
-6086
-1960
-3737
-6727
-7618
-5499
6235
7576
-7601
-5192
6816
-5179
-718
6096
-3434
75
-9131
6512
3190
3105
3278
-3693
1716
-4516
8945
-6251
-7251
-6195
-8260
914
3795
7558
-3368
-2181
-2150
7063
-720
-3542
8584
-3217
2407
-9092
-1119
-6780
-751
9109
8555
1337
-2638
-1251
5316
252
4825
-9510
-3187
5644
3797
-8140
8337
7836
-1165
5854
-9695
-6744
-6735
3618
9384
-1348
-2181
-4348
-8260
6498
9929
9603
1176
-2423
6757
6598
-1112
2345
-8670
2385
-1516
7345
3978
-410
5980
9168
-7500
6082
7999
401
301
4377
390
-2088
1176
4484
-4270
7578
3001
5530
-2847
-9876
9339
-9260
6702
8088
-7196
3085
281
4598
5976
-719
-4342
-4131
2465
867
7077
9136
-1823
5745
-8065
-7527
-2217
25
3452
-9039
-8890
2269
4886
1936
7196
3176
767
-2336
-8007
-6295
-520
-4979
4349
-5314
1744
-8281
-8366
4972
277
5540
8738
7598
-571
-3088
-6259
3898
-7431
6266
8393
4342
-9425
1127
-1490
-7798
5652
8624
-9388
-5028
-2182
-6274
-305
4599
4910
1105
-8512
5199
-9602
6290
-5400
-5598
6251
503
-1739
3715
8251
-7569
-7394
-6977
4645
9903
-9778
1320
-1233
-5985
-266
-5211
-8100
-1840
-1910
482
9713
5961
20
-1374
4756
-2875
-2717
-3582
-6578
6659
611
9462
-8159
-2136
186
-8156
1167
4074
-6508
6745
3591
786
-9329
4611
9249
-8703
-1168
-3450
562
-3556
-3416
2166
-749
527
-5260
-2161
-1984
-703
6106
-5099
2275
-3879
-8221
6795
-3819
-7028
-7832
-8330
740
760
5009
-6818
-1054
-3068
4751
6801
844
8334
-616
-3059
5793
-196
-3418
-1224
-42
699
7768
-4996
1530
-8531
-6494
8609
-4241
-428
7411
8331
-7500
1083
-3523
7902
-3133
-6342
-4383
-2562
1295
-7329
-1007
-6011
-1047
-1637
-3077
-1123
-6551
-1795
-6510
-8240
7835
1283
-8842
-1237
-1525
1181
3211
4958
3067
2413
2056
2190
3901
-2678
7025
7198
3581
-7731
2815
-1530
-3829
-3078
1301
-8054
630
-3045
-722
5036
4431
-2804
1952
2490
8443
-8712
-1426
-4397
-2644
-4394
792
7117
-8298
1689
1077
-4553
1888
4401
9418
-7337
-5048
-4241
-5100
199
-8166
-7341
1587
7316
6542
5827
-1147
-70
-1037
1313
6752
-6313
2830
-3985
-3491
-7963
9599
5430
-4727
-3530
-7489
-2455
-9894
7844
-1589
5745
-6848
8939
-5189
1568
7919
-420
1122
655
4058
-1454
9126
1724
-6508
1122
-2306
-3848
-7986
-5259
-1282
7541
-1481
6150
103
-9207
-5266
-1551
519
6167
5056
-5705
1642
8719
583
6033
611
-7042
-1232
441
-1313
-8794
6264
-4608
8439
1812
5888
-951
312
8629
8177
6555
121
-1842
5178
9288
6799
3110
2535
-1100
-2298
2719
8018
-5711
1578
-841
9345
9526
7380
-8326
2212
-4063
6536
-3965
5586
6814
-7417
2423
-8418
-8493
9901
-315
-1936
-6161
4645
2807
3095
-6468
2968
5710
7175
444
-9556
3228
5046
-9845
-4809
6125
6214
-8309
8935
-3373
-3996
6808
-6914
-4008
-1109
4401
5325
-1667
-5413
-8286
-1424
-8586
-157
-6362
8251
3398
-3614
-9982
7702
5580
65
-7633
6400
7380
6961
8031
-5770
-9390
-7444
-677
-3294
-7914
-813
-3244
3186
5519
3075
-7498
6424
3047
-3824
6724
2529
5902
3193
-6591
9362
2153
-9710
6752
9806
3548
-1348
-4008
9236
-8830
2180
-3059
-8156
-2191
2346
8596
9339
-191
-5631
-5275
8804
-5841
-9919
-6349
2599
1537
-9736
-6099
-175
4001
-6908
-5850
721
-764
463
-7110
-1124
-632
4959
9450
-8602
6176
3469
6603
-7419
2026
-3749
-8375
8529
3218
5813
9573
9091
-9030
3967
-5078
7912
4915
-7109
-5179
-2042
1091
-6729
5190
-424
9568
4841
3551
1110
-4684
-9004
-2964
6627
8734
-817
9655
41
4220
-856
5461
6373
-757
4706
6375
-1082
-4225
-2516
-8574
7844
-5939
3096
-3583
-6325
2728
-8861
2630
6532
3141
-6738
-4381
-3054
1126
-6408
5147
-2576
8053
-3617
-5311
-6171
-3545
-9056
2522
751
-9014
3064
1295
9361
3744
6980
6749
900
-5939
-6078
-1746
-3582
-5705
-2664
9178
-3180
7417
9640
7347
-6449
-5162
-6408
1065
2940
-1978
900
-9822
-6881
8624
8557
-8830
8069
7110
-4051
-3528
-3737
-3627
-5206
1698
6442
-3528
-3787
-425
3190
8226
7661
4081
5290
9594
8358
1050
-559
2961
-2665
-4365
8204
-7916
-7735
-641
2285
8434
-8623
-8503
340
5923
6128
-4118
-8306
2897
1450
46
-5958
3703
-1405
-7828
2739
-2804
-8296
6037
-8480
9682
2785
6027
-612
-2779
8945
-1885
9918
-1454
4032
-7797
-2005
-837
4360
-9298
6959
7798
1846
-79
-1714
-7671
8458
-5906
999
8873
9115
6171
-2849
9365
-2202
8031
7992
6567
4206
306
3889
-8219
2539
-8758
1353
-69
-4008
-7633
-6921
4611
5622
-3915
4238
-5239
-3244
-5202
-6308
817
8738
2407
-5464
508
-5493
8296
-5569
8754
-5826
-9866
-7278
-9033
7802
-2065
-2849
-1126
5795
9573
-8373
-844
-6412
5181
-669
-4298
-7221
-3606
-833
-5373
-266
-2625
9734
-8391
-565
-971
-4722
-6334
5508
-1569
182
2363
7552
5647
-3843
8347
-941
3597
5456
-7392
-1760
8381
-7601
-8125
-8118
2247
1482
3145
-7245
-5492
-3522
-6802
1906
-2347
2550
-9680
-9887
-7569
-4979
-9857
9117
7347
-723
-7936
5508
-7821
65
-670
-1891
-1149
-6739
5146
1710
-9736
2525
7526
9904
-2645
3469
6152
5119
-4528
537
6937
6634
-6843
-3693
9266
5167
-7408
-5145
301
9119
3353
8159
6275
-2234
-8423
5268
4457
6512
-982
-326
3502
149
136
-1231
-3513
-2541
-788
8513
-6018
8090
-1419
-6310
1429
6166
-3383
9621
7853
8638
-5350
8650
-9407
-2765
-5580
5508
5791
-8933
1313
5728
-6778
-471
943
-6287
-4004
-860
612
6400
9074
294
4109
9713
211
-4986
-7198
7510
-9067
-5587
2919
3128
-9539
-4572
-9558
-7343
5449
-1102
-1866
-8934
6879
5073
-1764
-9257
-3672
-3625
3868
1911
-9390
333
-1734
7347
845
-9568
289
552
8792
-4783
3995
-7007
4776
1470
5976
-466
-2443
4509
-8847
-4521
-9766
8082
3214
-2562
-3246
9784
-6485
-5295
208
116
-3072
4122
4317
9182
-9030
3351
3390
8718
173
-8861
-2638
-5495
-5093
6497
1091
-898
-2553
3005
-7159
325
8840
-3258
7798
-1024
8736
-4512
-7867
-9539
-755
2610
3076
9731
360
6433
-3182
-9026
-6090
-831
5244
9888
-6591
3496
9415
-4065
2125
-1730
-7003
7736
3537
-2821
-8001
9174
2062
-8815
-129
-1448
7363
-5495
-2708
9650
-7554
-3061
-8011
4103
-3124
-2046
4518
1906
-8108
7180
7743
9191
-8680
-9321
6175
-9886
-4042
-9324
-5980
-160
1414
-3418
40
1677
3132
164
-4060
-3363
3360
4585
-2423
94
-592
9748
5114
9918
1129
-1913
9852
-8469
-3996
146
5245
-8955
-4737
-9163
7138
9733
-5152
-2955
-7521
6840
-5592
-2093
3432
-2139
-8052
3121
7535
5325
-7017
781
1534
7700
-4072
6846
-1764
489
7138
2630
-8799
173
4439
6890
289
2840
5514
-9635
5122
-1864
-2105
4241
6045
-2333
-4812
2458
-8976
5299
3081
2635
6164
-7064
-8255
-2025
-3630
-6598
-8599
-7410
3595
4857
4739
8621
-8656
-751
8972
9108
-5390
-9843
-5944
-7598
8819
-5777
-814
124
-9043
-4426
-8739
2001
2859
1814
-6841
-816
9564
-3523
-4564
9128
1979
-562
-1178
9234
-5587
6278
272
-1370
-9672
-532
4565
1035
-5922
2185
317
-4411
-6451
8906
1710
-8738
-6739
-3169
7079
3673
-604
-2577
8486
-7064
2905
-7706
4800
-2955
-730
2432
-1180
-314
4935
-8739
3954
-9466
3766
6022
7146
154
982
-1429
-3442
79
-4980
-6599
7569
-324
-6641
-1836
4781
-8861
-8888
9633
8017
-1474
-7431
4371
1892
7799
2810
6786
-659
-6569
9774
6553
-5350
482
2768
-2950
-9578
7690
-986
1052
7913
-7877
2728
2559
-3911
270
111
9352
1090
336
-6569
-5695
-621
-4735
3010
-2994
-6183
8576
674
-272
-2574
4115
-118
-8742
713
-2338
-7287
-3528
-5302
9772
7277
5344
1678
5108
2156
2044
-4392
-1110
-5174
-8519
3047
-4982
4722
4815
9607
-295
2846
9250
-225
694
-6086
954
-8032
-7824
-515
-1786
7217
3879
-650
7357
1859
819
-3113
9668
1250
796
7449
6639
-125
-6745
-4986
-1527
-3615
2728
-5276
-1129
3995
-9790
-3936
8296
9002
-3675
1705
1063
4300
-1570
-4034
-4121
-6992
-6712
3956
-5995
5092
-2380
9748
-3585
-9004
3053
-1085
-4777
6754
1224
-850
308
-383
-2075
-5101
-5992
-1101
6128
-5397
-268
8284
6807
7312
4571
2490
-4408
9019
6659
-8894
-410
6355
5659
655
9558
-1712
6395
759
1953
-1516
17
-5394
-3989
2522
7495
-1047
5287
-7221
-9544
-2294
-4832
4040
8478
7468
6702
-8938
7532
-4394
-5710
7135
-1196
-7343
3556
-9030
-1848
2202
-7220
2959
7848
-3965
-1987
-4538
-9704
9598
4107
-1126
6061
3325
-1344
1450
-6591
-2801
-8688
-5180
2757
-6086
6447
2637
-4749
-7025
-1628
-3822
-1583
752
7890
1295
-6479
5245
3005
-9934
3889
-1783
-7482
-5963
-9537
4807
6022
1971
1007
-9635
2667
-2821
-6586
8375
6959
3506
9435
7660
7660
5794
1078
9817
3745
-3169
1225
7815
5816
8090
-3843
-4722
-6912
8201
-9843
-3003
3402
-5019
3772
2892
1127
-5967
-5788
8701
2998
2055
-2584
-6684
9439
436
9073
-6687
7913
8621
2374
7121
-8447
-4972
8600
1955
894
1861
1546
6818
35
-8282
-7831
-6032
7818
6207
-225
1971
2354
-5106
-1244
-2459
-1391
780
-2234
-2046
3511
-5465
-9536
-3720
2402
-1893
-9522
5631
-1190
173
-9859
6265
-140
-1495
-1210
9534
989
4141
-3675
-1101
9245
2486
8880
-4299
9105
-9232
-3372
-9778
-2256
8257
3325
-3542
358
168
9890
5146
1614
-4593
2867
-6654
8342
6716
-2935
1480
-7483
8633
4453
2993
-1029
-7809
-5447
3430
6320
-5533
3118
3110
3628
2355
-129
6724
-8172
-2955
6322
8896
9820
-7947
-410
-5051
417
4288
5683
6587
-3563
-6733
-4566
-7865
-1126
-4034
-7406
1820
1571
8164
8917
-5874
-8970
1615
-6802
-2576
1097
-3511
3215
-8852
7387
-2653
-8277
-3418
-591
9192
-4802
6300
-5922
2878
-9888
-9495
-918
9808
9401
8882
6518
-7675
-7179
-5169
5311
-9016
7844
-1432
-8314
7800
6613
-9207
-8447
-449
-7380
3195
9736
2189
3456
-9843
-6023
-9395
-4075
-4846
-9642
-2090
6555
5060
6322
-6588
2860
-9750
5941
2794
4177
4596
-6654
-282
-3211
-5508
-680
-7888
4857
-8417
-2765
-4988
-8131
1286
7108
-4814
4758
1046
-1703
2595
-324
1966
7065
-6508
4900
-5534
-6802
8524
-7588
-3245
9342
-951
9269
-3331
9943
8667
8038
3522
2324
-5206
-3391
-5836
-8451
-65
9110
2955
6658
-3730
7636
-8480
-5192
-3752
-7016
-1390
900
7242
412
-5493
9651
-2264
-225
-5841
4914
-4227
3869
6641
-3887
-9639
8090
3151
5972
3143
9022
-4120
-5499
-3239
2972
146
-1783
-4591
1783
6252
-1816
-7561
1162
2652
7492
7236
-1610
817
3556
-906
-8789
814
-6090
-7935
-7657
-1151
-790
8114
-8203
-1997
1466
5005
9191
1062
-5077
-9081
-410
-5538
-7090
-7085
-9257
1284
1735
9313
-7582
-8080
-7898
3631
224
-5727
8060
-9749
1840
1382
-1907
-4275
-5152
-7274
-9247
2559
-9738
642
-2584
-6522
-1153
1055
5092
-3068
-9075
-7637
2423
5202
9291
5142
3669
-1908
1623
-9388
-9007
-8488
3226
-9581
-3346
5344
-8330
2607
5615
-473
-6892
-425
1128
3110
6069
5971
-1364
-7368
8088
2758
8478
-8033
-4557
9052
-452
2495
9470
-1014
5094
-6494
-1790
-9778
-6024
-8297
2940
-8036
3703
1128
-2361
1019
8381
-9763
1561
6353
259
-9951
2221
-3342
995
-4225
1129
3586
5827
-849
1792
-9536
-982
1655
-8133
308
-976
276
-245
-6257
9329
-6549
-9713
-1361
8700
1924
-1019
-319
2474
-4833
-6412
-8579
442
1709
8042
4914
-9056
-5426
8354
9534
-2854
-3540
934
7836
-2263
6512
-1228
-1119
-5665
-9778
-8969
-7798
4915
-7171
1416
9762
2635
6203
7636
-5396
9987
3427
-8969
5444
2424
-3867
1530
1642
2371
3766
8761
-2137
195
8862
-9539
-9606
-3617
-9343
-2298
-313
-7466
-1449
-2821
-6518
6375
507
5050
-8113
3015
-5029
-8087
8563
472
-3935
-6190
-7061
5176
-9908
3887
-2455
-4526
5147
2184
-7964
-5578
-6302
7962
-9567
-9845
-3632
-6464
5176
-1404
-9380
-4552
274
-1739
-2139
-8904
2329
2163
-549
-7009
-9613
189
7612
6469
-2445
-774
604
2728
9320
8163
4062
-8118
9449
-7716
-141
-6849
9481
-511
5710
-9845
-9523
4371
-8782
-8719
6196
900
-8799
185
7395
-8350
2240
9110
-7541
6537
-9437
-951
-8212
2962
6511
497
1284
-9742
914
-314
9869
4112
329
-1234
7790
-1703
-8837
3602
3621
-3028
5094
-4710
-734
-4520
-382
6846
-8837
-1181
944
-3733
8585
2302
8919
-227
1382
2245
1474
1783
860
-5019
3901
5687
-2515
6771
-1307
-7691
-7790
1286
-7580
1648
7355
-5292
-9984
-9700
173
-9207
2873
-1022
-7922
-8714
9966
2514
4725
4763
8587
-4980
-401
-6381
-3166
-1730
-790
5317
-4486
4300
-4918
1794
-7444
-5275
-8383
-5335
-6723
6384
8819
5910
6247
9835
-7153
3087
561
9702
2113
9969
5396
-6150
-1382
4481
-8582
8550
-8613
9381
8019
2789
8569
7632
-849
4050
1215
-4525
1527
182
-1483
1434
8340
-7410
3164
-7541
-9638
2157
787
6969
5961
337
4202
5611
-2033
-7064
-8935
8149
2325
7784
9653
7119
-4917
-1907
-9554
-4365
1167
-8955
-1123
-3454
-9768
-4483
9517
-202
6933
959
-9886
6096
-4348
233
8520
-760
1894
-3539
-9291
664
-4170
-1035
-8457
8597
2125
308
-9224
521
5080
-1836
-52
2601
-7066
-3008
-7889
9088
-6030
7356
9930
-5988
-8621
-6752
8457
-3578
-9960
9144
-9839
-2380
-3329
8851
7708
4974
-6505
-715
-3149
8900
-3233
1368
2057
7150
-5731
-9694
3359
-1936
-2886
1608
9951
-3187
6043
-837
-6424
-7947
9073
2254
417
-591
7434
4453
9798
-7041
4857
-2476
7982
5965
-1024
9695
4724
-3108
3607
-4363
323
1956
5567
-8326
-4660
3007
5302
-5631
-8708
9966
9621
-3887
4872
4527
-2130
-6317
1585
288
-8035
-3159
-6336
-7842
9321
6433
-5988
4782
8443
8545
6818
8337
-4900
1760
1888
103
-8941
-976
2960
5943
-8627
2658
-9539
-4198
1127
8502
-8158
-5795
-316
1507
-6055
-5923
-3644
1753
-466
-3955
-9934
-7898
1925
2959
-7228
-8760
3382
-3013
7612
-8882
2596
1263
-8408
-8114
-3660
-4439
2658
5207
-7209
-4431
6604
1965
4053
-4305
-5171
6114
4756
6091
3772
-9546
-5995
6275
-9991
-8041
-656
-7360
1736
6671
3753
4800
482
-7500
2228
797
-8862
-3285
3110
-1615
-7392
5778
-7792
448
6118
-4046
5710
36
7589
2221
-8082
6069
-9147
-7599
2144
1185
2422
-5761
2820
4961
-9791
-3994
4474
-4980
-171
-4990
5728
5092
-231
-232
5650
1886
3053
-3926
-8110
-2560
-5638
8708
-8598
6568
-3313
6275
-7017
-7468
-8235
929
-6651
-6789
-454
1562
-3682
4058
7645
-2092
-8593
7586
-2823
4452
-3277
-9613
6933
-7675
9682
3402
-5206
-4299
-5980
-3257
867
-1713
-9227
8057
5683
2698
-5456
7510
4629
9234
5240
-7048
7485
7146
9131
-3211
-3817
-9934
158
-5578
5907
-5988
-8423
7532
7376
4982
-2332
1320
9052
-5309
-1779
-8542
-7832
-1954
-231
5813
-4217
-6452
-1235
-9615
-3172
2411
4642
3702
3081
-8291
-5192
7911
6113
-2996
7745
3089
-7889
4807
-4397
2388
2579
4837
-8970
5169
-8549
-9082
-1454
-1988
9882
1522
-8262
-8264
1762
2189
-1374
-4980
-6287
-118
-7431
1042
-8888
-8065
-1582
6309
5870
8063
3095
7641
6287
7927
-1232
-2094
1475
-2060
7025
-4402
5730
725
-1909
9238
-9317
-3523
-1397
-9225
-1541
4973
9328
2362
9848
-5538
7387
5005
2584
-8739
1083
6397
-3918
7777
2888
1899
-140
9449
7398
9676
2839
-634
6467
1931
3805
8710
7264
-8884
-623
8994
1240
347
3095
3809
-8486
-3239
2618
7784
6246
3649
5529
-9255
8045
-4759
1159
699
-3851
-360
5332
6435
569
-1635
-7419
6197
6043
3863
-6830
3899
-313
-5400
-3363
-5498
272
8358
1792
-7736
-9822
-6287
9815
-3723
-6455
4352
-6427
1575
2726
2391
-6496
9882
2848
-7757
-9784
460
4367
904
3000
8258
-1925
8536
2277
-9092
-7543
-6580
6036
26
-2000
-6793
2840
7204
-9866
274
6815
-9221
-5992
-8026
-9019
-9602
8583
3113
-5600
5795
-2330
-4476
-5647
-6730
-9888
7301
-1972
-699
9285
-8007
8138
6203
1497
-7466
9843
1971
1678
9616
-7682
3626
9192
8790
1741
-2032
6727
5092
2579
-4689
-2541
7037
9049
-8669
-9959
6134
-9329
-9762
8511
5666
-9323
2163
-7320
-9195
10
2947
943
-725
-1364
1264
7913
-9444
8989
-9514
9519
-5688
-9006
-5223
-9712
-1892
2960
9765
8970
-983
-5285
-8172
1953
786
3609
-239
4058
3607
-701
8795
-8211
-3841
-5167
1988
-2924
7495
6752
-4763
710
-5499
-9413
9894
3689
-8783
-9015
-5988
4044
-9712
8883
-9030
5162
3288
-5982
-2224
-3351
289
-5632
-5875
-5711
9018
-3994
7844
-8195
6560
-3936
-4555
-1393
-2882
6007
3686
6061
9433
-8069
-4394
-6440
-4358
8692
2340
8033
-8669
-3001
-7274
8261
2440
-8742
3241
8337
-4583
-4055
-718
-202
-172
-2073
-3239
5287
9224
3721
9959
949
-1601
-8419
4023
4596
-5247
4240
-6074
-35
-839
-1768
4644
2961
-5892
8875
-719
2815
5798
-155
-4194
-3524
-2103
5793
-3890
6251
1602
-2353
9590
2999
6196
-9437
8148
-1201
-5993
-1469
-1745
-9762
9598
6222
1124
-757
-9052
-3365
1091
277
4982
3863
-8480
-5426
-5223
-4286
-1525
9390
5529
-3968
3241
9128
2525
-3366
-2639
5439
-5174
-7610
-1921
8759
-7090
5046
8873
4103
4154
-6780
-2501
-5193
-1124
-9857
7288
-8815
9118
-2033
5279
5140
6497
-6319
-7798
2909
-9632
-1894
-2391
4158
-522
-9302
-9360
-5539
-172
-3710
4591
7957
8710
2807
7586
8676
-8651
2638
-9695
9918
792
-8990
3716
1162
-330
-2943
-8373
-6510
-5451
-8272
3530
-7712
2989
2512
4355
-4954
7708
-7171
-5898
-4061
4093
6578
4669
1587
-9192
-4135
-7755
5608
-4372
-5980
-2187
-2179
-8162
-5793
-6597
-7213
-4194
7546
-9139
-9317
-5907
8650
-3865
-4747
-4470
-9266
3329
6469
7236
2116
1050
-9136
5532
-9225
-8708
7534
-4179
-5677
1744
1301
-9661
9904
-5705
-4986
-2061
-966
3817
3899
8176
-9405
-1377
-9700
-1434
-1232
2005
-9888
-4070
4253
3029
-1578
-7917
5355
-2764
-1147
-5201
496
-7484
4108
-1552
9653
-3545
-6593
-1349
9867
3162
-1016
-7783
4097
-6419
-5285
-6471
-7024
9525
584
-4777
-9672
7028
5838
6954
-5206
7400
-5153
6191
-7090
-9257
-891
2730
760
9502
-4552
6795
5189
6103
6568
4585
5662
-1965
-4204
935
2360
5146
7645
2424
-3693
1645
-1415
-6780
-3435
-7126
-7897
-9843
5305
-1211
8710
6320
-9667
-6122
-9537
-916
9599
9606
2999
-9514
6649
-3797
-734
2630
9694
7526
-5019
-6718
4719
5691
-4926
3765
295
3963
2432
5471
9697
-3218
-9897
4345
233
364
-158
-1270
-9456
-3424
-1813
417
-3857
-3817
7271
6735
1240
-9069
-7473
-449
-9511
6547
1733
5209
6899
-9689
-5602
8717
-1136
-9020
3934
794
-7576
-8704
7180
-6628
8786
4582
-5162
-3198
-1228
-3734
1536
2632
-9007
4899
7347
-2086
-2360
9001
5519
2940
-6302
-8321
-6855
1546
471
1635
508
-174
1839
6296
7609
-5981
1903
-5803
-3718
1127
-5702
-4304
1455
-5270
9495
-2483
8792
5800
-5399
3776
-5493
-7497
3300
-4198
-2817
-259
-6381
7416
-7410
-2379
-9096
7934
7992
669
7353
-9885
-7757
-9260
7417
-4385
-9159
-2186
-8629
1971
9572
7573
8945
-6891
-6064
4413
-9260
8994
-1370
539
4307
272
-4051
-7528
-8631
-9912
2884
-9902
5421
8395
7062
-4735
3430
-9002
8798
-6744
8351
-1305
-4899
-8821
41
2401
5321
7916
8345
-2849
5805
-9494
-2234
-9449
8585
3692
7844
5303
4483
1921
2067
7558
8063
3788
-1783
-9256
-5162
-8815
-3955
2474
0
-1108
-853
-5120
6568
1352
2262
-1897
7912
6197
-6266
-9343
7433
294
9958
-4571
-4036
2705
2599
-6786
1905
4901
6568
4198
630
5078
-7009
9495
5428
3957
-9736
9201
-3629
3459
-1526
-9377
7684
1051
8056
4885
6880
-1749
7532
-9391
8345
-3962
-1619
8222
-7718
-2286
-5988
-3146
9443
6757
34
-8087
-2832
-4564
3578
-330
4433
4145
-4198
5726
-2414
3053
9117
-4520
7437
6638
-7545
-4926
6686
6068
7195
4248
-680
-9632
-4348
6767
9061
-4905
6937
6017
-1168
301
9442
-7776
-9027
-9161
8611
-3690
-7394
4691
-3336
-5763
-7050
4667
654
502
4872
-8383
8350
1626
4545
-5628
-1054
-6346
-946
-92
-8570
-880
9527
-9232
-2549
1587
616
3096
4312
9313
-5875
2087
-5705
3346
-6073
-5246
3329
-1006
-3172
-8833
-2884
-9982
-3707
-4109
-2437
6128
-7052
-1896
6395
-21
7210
-6717
-3630
4368
-5722
4101
3698
5536
826
2015
5505
556
-2653
-221
-3398
-6789
-6167
-970
-7037
-3093
-5397
4446
-594
669
1124
-551
-8373
-8688
3574
8833
9826
-5860
-4082
-7410
308
-1646
-6308
5838
    "###.trim()
}
