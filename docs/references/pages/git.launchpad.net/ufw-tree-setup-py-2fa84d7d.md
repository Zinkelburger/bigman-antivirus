---
url: "https://git.launchpad.net/ufw/tree/setup.py"
title: "setup.py - ufw - [no description]"
captured: "2026-11-03T02:57:32Z"
via: "live"
sha256: "361ba2a02cbf5b1f63299be9f70e3e52ac537a4eddd3555559221d0fefea5a5c"
wayback: "https://web.archive.org/web/20260112175549/https://git.launchpad.net/ufw/tree/setup.py"
cited_in:
  - "docs/research/hardening.md"
---

# setup.py - ufw - [no description]

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://git.launchpad.net/ufw/tree/setup.py>
> Snapshot: <https://web.archive.org/web/20260112175549/https://git.launchpad.net/ufw/tree/setup.py>

| https://git.launchpad.net/  | [index](https://git.launchpad.net/) : [ufw](https://git.launchpad.net/ufw/) |

| [no description] |

|  [summary](https://git.launchpad.net/ufw/)[refs](https://git.launchpad.net/ufw/refs/)[log](https://git.launchpad.net/ufw/log/setup.py)[tree](https://git.launchpad.net/ufw/tree/setup.py)[commit](https://git.launchpad.net/ufw/commit/setup.py)[diff](https://git.launchpad.net/ufw/diff/setup.py) |

path: [root](https://git.launchpad.net/ufw/tree/)/[setup.py](https://git.launchpad.net/ufw/tree/setup.py)

blob: 0388f2ce0fa039ef4bad0335cda252e27210509c ([plain](https://git.launchpad.net/ufw/plain/setup.py))

|

```
[1](https://git.launchpad.net/ufw/tree/setup.py#n1)
[2](https://git.launchpad.net/ufw/tree/setup.py#n2)
[3](https://git.launchpad.net/ufw/tree/setup.py#n3)
[4](https://git.launchpad.net/ufw/tree/setup.py#n4)
[5](https://git.launchpad.net/ufw/tree/setup.py#n5)
[6](https://git.launchpad.net/ufw/tree/setup.py#n6)
[7](https://git.launchpad.net/ufw/tree/setup.py#n7)
[8](https://git.launchpad.net/ufw/tree/setup.py#n8)
[9](https://git.launchpad.net/ufw/tree/setup.py#n9)
[10](https://git.launchpad.net/ufw/tree/setup.py#n10)
[11](https://git.launchpad.net/ufw/tree/setup.py#n11)
[12](https://git.launchpad.net/ufw/tree/setup.py#n12)
[13](https://git.launchpad.net/ufw/tree/setup.py#n13)
[14](https://git.launchpad.net/ufw/tree/setup.py#n14)
[15](https://git.launchpad.net/ufw/tree/setup.py#n15)
[16](https://git.launchpad.net/ufw/tree/setup.py#n16)
[17](https://git.launchpad.net/ufw/tree/setup.py#n17)
[18](https://git.launchpad.net/ufw/tree/setup.py#n18)
[19](https://git.launchpad.net/ufw/tree/setup.py#n19)
[20](https://git.launchpad.net/ufw/tree/setup.py#n20)
[21](https://git.launchpad.net/ufw/tree/setup.py#n21)
[22](https://git.launchpad.net/ufw/tree/setup.py#n22)
[23](https://git.launchpad.net/ufw/tree/setup.py#n23)
[24](https://git.launchpad.net/ufw/tree/setup.py#n24)
[25](https://git.launchpad.net/ufw/tree/setup.py#n25)
[26](https://git.launchpad.net/ufw/tree/setup.py#n26)
[27](https://git.launchpad.net/ufw/tree/setup.py#n27)
[28](https://git.launchpad.net/ufw/tree/setup.py#n28)
[29](https://git.launchpad.net/ufw/tree/setup.py#n29)
[30](https://git.launchpad.net/ufw/tree/setup.py#n30)
[31](https://git.launchpad.net/ufw/tree/setup.py#n31)
[32](https://git.launchpad.net/ufw/tree/setup.py#n32)
[33](https://git.launchpad.net/ufw/tree/setup.py#n33)
[34](https://git.launchpad.net/ufw/tree/setup.py#n34)
[35](https://git.launchpad.net/ufw/tree/setup.py#n35)
[36](https://git.launchpad.net/ufw/tree/setup.py#n36)
[37](https://git.launchpad.net/ufw/tree/setup.py#n37)
[38](https://git.launchpad.net/ufw/tree/setup.py#n38)
[39](https://git.launchpad.net/ufw/tree/setup.py#n39)
[40](https://git.launchpad.net/ufw/tree/setup.py#n40)
[41](https://git.launchpad.net/ufw/tree/setup.py#n41)
[42](https://git.launchpad.net/ufw/tree/setup.py#n42)
[43](https://git.launchpad.net/ufw/tree/setup.py#n43)
[44](https://git.launchpad.net/ufw/tree/setup.py#n44)
[45](https://git.launchpad.net/ufw/tree/setup.py#n45)
[46](https://git.launchpad.net/ufw/tree/setup.py#n46)
[47](https://git.launchpad.net/ufw/tree/setup.py#n47)
[48](https://git.launchpad.net/ufw/tree/setup.py#n48)
[49](https://git.launchpad.net/ufw/tree/setup.py#n49)
[50](https://git.launchpad.net/ufw/tree/setup.py#n50)
[51](https://git.launchpad.net/ufw/tree/setup.py#n51)
[52](https://git.launchpad.net/ufw/tree/setup.py#n52)
[53](https://git.launchpad.net/ufw/tree/setup.py#n53)
[54](https://git.launchpad.net/ufw/tree/setup.py#n54)
[55](https://git.launchpad.net/ufw/tree/setup.py#n55)
[56](https://git.launchpad.net/ufw/tree/setup.py#n56)
[57](https://git.launchpad.net/ufw/tree/setup.py#n57)
[58](https://git.launchpad.net/ufw/tree/setup.py#n58)
[59](https://git.launchpad.net/ufw/tree/setup.py#n59)
[60](https://git.launchpad.net/ufw/tree/setup.py#n60)
[61](https://git.launchpad.net/ufw/tree/setup.py#n61)
[62](https://git.launchpad.net/ufw/tree/setup.py#n62)
[63](https://git.launchpad.net/ufw/tree/setup.py#n63)
[64](https://git.launchpad.net/ufw/tree/setup.py#n64)
[65](https://git.launchpad.net/ufw/tree/setup.py#n65)
[66](https://git.launchpad.net/ufw/tree/setup.py#n66)
[67](https://git.launchpad.net/ufw/tree/setup.py#n67)
[68](https://git.launchpad.net/ufw/tree/setup.py#n68)
[69](https://git.launchpad.net/ufw/tree/setup.py#n69)
[70](https://git.launchpad.net/ufw/tree/setup.py#n70)
[71](https://git.launchpad.net/ufw/tree/setup.py#n71)
[72](https://git.launchpad.net/ufw/tree/setup.py#n72)
[73](https://git.launchpad.net/ufw/tree/setup.py#n73)
[74](https://git.launchpad.net/ufw/tree/setup.py#n74)
[75](https://git.launchpad.net/ufw/tree/setup.py#n75)
[76](https://git.launchpad.net/ufw/tree/setup.py#n76)
[77](https://git.launchpad.net/ufw/tree/setup.py#n77)
[78](https://git.launchpad.net/ufw/tree/setup.py#n78)
[79](https://git.launchpad.net/ufw/tree/setup.py#n79)
[80](https://git.launchpad.net/ufw/tree/setup.py#n80)
[81](https://git.launchpad.net/ufw/tree/setup.py#n81)
[82](https://git.launchpad.net/ufw/tree/setup.py#n82)
[83](https://git.launchpad.net/ufw/tree/setup.py#n83)
[84](https://git.launchpad.net/ufw/tree/setup.py#n84)
[85](https://git.launchpad.net/ufw/tree/setup.py#n85)
[86](https://git.launchpad.net/ufw/tree/setup.py#n86)
[87](https://git.launchpad.net/ufw/tree/setup.py#n87)
[88](https://git.launchpad.net/ufw/tree/setup.py#n88)
[89](https://git.launchpad.net/ufw/tree/setup.py#n89)
[90](https://git.launchpad.net/ufw/tree/setup.py#n90)
[91](https://git.launchpad.net/ufw/tree/setup.py#n91)
[92](https://git.launchpad.net/ufw/tree/setup.py#n92)
[93](https://git.launchpad.net/ufw/tree/setup.py#n93)
[94](https://git.launchpad.net/ufw/tree/setup.py#n94)
[95](https://git.launchpad.net/ufw/tree/setup.py#n95)
[96](https://git.launchpad.net/ufw/tree/setup.py#n96)
[97](https://git.launchpad.net/ufw/tree/setup.py#n97)
[98](https://git.launchpad.net/ufw/tree/setup.py#n98)
[99](https://git.launchpad.net/ufw/tree/setup.py#n99)
[100](https://git.launchpad.net/ufw/tree/setup.py#n100)
[101](https://git.launchpad.net/ufw/tree/setup.py#n101)
[102](https://git.launchpad.net/ufw/tree/setup.py#n102)
[103](https://git.launchpad.net/ufw/tree/setup.py#n103)
[104](https://git.launchpad.net/ufw/tree/setup.py#n104)
[105](https://git.launchpad.net/ufw/tree/setup.py#n105)
[106](https://git.launchpad.net/ufw/tree/setup.py#n106)
[107](https://git.launchpad.net/ufw/tree/setup.py#n107)
[108](https://git.launchpad.net/ufw/tree/setup.py#n108)
[109](https://git.launchpad.net/ufw/tree/setup.py#n109)
[110](https://git.launchpad.net/ufw/tree/setup.py#n110)
[111](https://git.launchpad.net/ufw/tree/setup.py#n111)
[112](https://git.launchpad.net/ufw/tree/setup.py#n112)
[113](https://git.launchpad.net/ufw/tree/setup.py#n113)
[114](https://git.launchpad.net/ufw/tree/setup.py#n114)
[115](https://git.launchpad.net/ufw/tree/setup.py#n115)
[116](https://git.launchpad.net/ufw/tree/setup.py#n116)
[117](https://git.launchpad.net/ufw/tree/setup.py#n117)
[118](https://git.launchpad.net/ufw/tree/setup.py#n118)
[119](https://git.launchpad.net/ufw/tree/setup.py#n119)
[120](https://git.launchpad.net/ufw/tree/setup.py#n120)
[121](https://git.launchpad.net/ufw/tree/setup.py#n121)
[122](https://git.launchpad.net/ufw/tree/setup.py#n122)
[123](https://git.launchpad.net/ufw/tree/setup.py#n123)
[124](https://git.launchpad.net/ufw/tree/setup.py#n124)
[125](https://git.launchpad.net/ufw/tree/setup.py#n125)
[126](https://git.launchpad.net/ufw/tree/setup.py#n126)
[127](https://git.launchpad.net/ufw/tree/setup.py#n127)
[128](https://git.launchpad.net/ufw/tree/setup.py#n128)
[129](https://git.launchpad.net/ufw/tree/setup.py#n129)
[130](https://git.launchpad.net/ufw/tree/setup.py#n130)
[131](https://git.launchpad.net/ufw/tree/setup.py#n131)
[132](https://git.launchpad.net/ufw/tree/setup.py#n132)
[133](https://git.launchpad.net/ufw/tree/setup.py#n133)
[134](https://git.launchpad.net/ufw/tree/setup.py#n134)
[135](https://git.launchpad.net/ufw/tree/setup.py#n135)
[136](https://git.launchpad.net/ufw/tree/setup.py#n136)
[137](https://git.launchpad.net/ufw/tree/setup.py#n137)
[138](https://git.launchpad.net/ufw/tree/setup.py#n138)
[139](https://git.launchpad.net/ufw/tree/setup.py#n139)
[140](https://git.launchpad.net/ufw/tree/setup.py#n140)
[141](https://git.launchpad.net/ufw/tree/setup.py#n141)
[142](https://git.launchpad.net/ufw/tree/setup.py#n142)
[143](https://git.launchpad.net/ufw/tree/setup.py#n143)
[144](https://git.launchpad.net/ufw/tree/setup.py#n144)
[145](https://git.launchpad.net/ufw/tree/setup.py#n145)
[146](https://git.launchpad.net/ufw/tree/setup.py#n146)
[147](https://git.launchpad.net/ufw/tree/setup.py#n147)
[148](https://git.launchpad.net/ufw/tree/setup.py#n148)
[149](https://git.launchpad.net/ufw/tree/setup.py#n149)
[150](https://git.launchpad.net/ufw/tree/setup.py#n150)
[151](https://git.launchpad.net/ufw/tree/setup.py#n151)
[152](https://git.launchpad.net/ufw/tree/setup.py#n152)
[153](https://git.launchpad.net/ufw/tree/setup.py#n153)
[154](https://git.launchpad.net/ufw/tree/setup.py#n154)
[155](https://git.launchpad.net/ufw/tree/setup.py#n155)
[156](https://git.launchpad.net/ufw/tree/setup.py#n156)
[157](https://git.launchpad.net/ufw/tree/setup.py#n157)
[158](https://git.launchpad.net/ufw/tree/setup.py#n158)
[159](https://git.launchpad.net/ufw/tree/setup.py#n159)
[160](https://git.launchpad.net/ufw/tree/setup.py#n160)
[161](https://git.launchpad.net/ufw/tree/setup.py#n161)
[162](https://git.launchpad.net/ufw/tree/setup.py#n162)
[163](https://git.launchpad.net/ufw/tree/setup.py#n163)
[164](https://git.launchpad.net/ufw/tree/setup.py#n164)
[165](https://git.launchpad.net/ufw/tree/setup.py#n165)
[166](https://git.launchpad.net/ufw/tree/setup.py#n166)
[167](https://git.launchpad.net/ufw/tree/setup.py#n167)
[168](https://git.launchpad.net/ufw/tree/setup.py#n168)
[169](https://git.launchpad.net/ufw/tree/setup.py#n169)
[170](https://git.launchpad.net/ufw/tree/setup.py#n170)
[171](https://git.launchpad.net/ufw/tree/setup.py#n171)
[172](https://git.launchpad.net/ufw/tree/setup.py#n172)
[173](https://git.launchpad.net/ufw/tree/setup.py#n173)
[174](https://git.launchpad.net/ufw/tree/setup.py#n174)
[175](https://git.launchpad.net/ufw/tree/setup.py#n175)
[176](https://git.launchpad.net/ufw/tree/setup.py#n176)
[177](https://git.launchpad.net/ufw/tree/setup.py#n177)
[178](https://git.launchpad.net/ufw/tree/setup.py#n178)
[179](https://git.launchpad.net/ufw/tree/setup.py#n179)
[180](https://git.launchpad.net/ufw/tree/setup.py#n180)
[181](https://git.launchpad.net/ufw/tree/setup.py#n181)
[182](https://git.launchpad.net/ufw/tree/setup.py#n182)
[183](https://git.launchpad.net/ufw/tree/setup.py#n183)
[184](https://git.launchpad.net/ufw/tree/setup.py#n184)
[185](https://git.launchpad.net/ufw/tree/setup.py#n185)
[186](https://git.launchpad.net/ufw/tree/setup.py#n186)
[187](https://git.launchpad.net/ufw/tree/setup.py#n187)
[188](https://git.launchpad.net/ufw/tree/setup.py#n188)
[189](https://git.launchpad.net/ufw/tree/setup.py#n189)
[190](https://git.launchpad.net/ufw/tree/setup.py#n190)
[191](https://git.launchpad.net/ufw/tree/setup.py#n191)
[192](https://git.launchpad.net/ufw/tree/setup.py#n192)
[193](https://git.launchpad.net/ufw/tree/setup.py#n193)
[194](https://git.launchpad.net/ufw/tree/setup.py#n194)
[195](https://git.launchpad.net/ufw/tree/setup.py#n195)
[196](https://git.launchpad.net/ufw/tree/setup.py#n196)
[197](https://git.launchpad.net/ufw/tree/setup.py#n197)
[198](https://git.launchpad.net/ufw/tree/setup.py#n198)
[199](https://git.launchpad.net/ufw/tree/setup.py#n199)
[200](https://git.launchpad.net/ufw/tree/setup.py#n200)
[201](https://git.launchpad.net/ufw/tree/setup.py#n201)
[202](https://git.launchpad.net/ufw/tree/setup.py#n202)
[203](https://git.launchpad.net/ufw/tree/setup.py#n203)
[204](https://git.launchpad.net/ufw/tree/setup.py#n204)
[205](https://git.launchpad.net/ufw/tree/setup.py#n205)
[206](https://git.launchpad.net/ufw/tree/setup.py#n206)
[207](https://git.launchpad.net/ufw/tree/setup.py#n207)
[208](https://git.launchpad.net/ufw/tree/setup.py#n208)
[209](https://git.launchpad.net/ufw/tree/setup.py#n209)
[210](https://git.launchpad.net/ufw/tree/setup.py#n210)
[211](https://git.launchpad.net/ufw/tree/setup.py#n211)
[212](https://git.launchpad.net/ufw/tree/setup.py#n212)
[213](https://git.launchpad.net/ufw/tree/setup.py#n213)
[214](https://git.launchpad.net/ufw/tree/setup.py#n214)
[215](https://git.launchpad.net/ufw/tree/setup.py#n215)
[216](https://git.launchpad.net/ufw/tree/setup.py#n216)
[217](https://git.launchpad.net/ufw/tree/setup.py#n217)
[218](https://git.launchpad.net/ufw/tree/setup.py#n218)
[219](https://git.launchpad.net/ufw/tree/setup.py#n219)
[220](https://git.launchpad.net/ufw/tree/setup.py#n220)
[221](https://git.launchpad.net/ufw/tree/setup.py#n221)
[222](https://git.launchpad.net/ufw/tree/setup.py#n222)
[223](https://git.launchpad.net/ufw/tree/setup.py#n223)
[224](https://git.launchpad.net/ufw/tree/setup.py#n224)
[225](https://git.launchpad.net/ufw/tree/setup.py#n225)
[226](https://git.launchpad.net/ufw/tree/setup.py#n226)
[227](https://git.launchpad.net/ufw/tree/setup.py#n227)
[228](https://git.launchpad.net/ufw/tree/setup.py#n228)
[229](https://git.launchpad.net/ufw/tree/setup.py#n229)
[230](https://git.launchpad.net/ufw/tree/setup.py#n230)
[231](https://git.launchpad.net/ufw/tree/setup.py#n231)
[232](https://git.launchpad.net/ufw/tree/setup.py#n232)
[233](https://git.launchpad.net/ufw/tree/setup.py#n233)
[234](https://git.launchpad.net/ufw/tree/setup.py#n234)
[235](https://git.launchpad.net/ufw/tree/setup.py#n235)
[236](https://git.launchpad.net/ufw/tree/setup.py#n236)
[237](https://git.launchpad.net/ufw/tree/setup.py#n237)
[238](https://git.launchpad.net/ufw/tree/setup.py#n238)
[239](https://git.launchpad.net/ufw/tree/setup.py#n239)
[240](https://git.launchpad.net/ufw/tree/setup.py#n240)
[241](https://git.launchpad.net/ufw/tree/setup.py#n241)
[242](https://git.launchpad.net/ufw/tree/setup.py#n242)
[243](https://git.launchpad.net/ufw/tree/setup.py#n243)
[244](https://git.launchpad.net/ufw/tree/setup.py#n244)
[245](https://git.launchpad.net/ufw/tree/setup.py#n245)
[246](https://git.launchpad.net/ufw/tree/setup.py#n246)
[247](https://git.launchpad.net/ufw/tree/setup.py#n247)
[248](https://git.launchpad.net/ufw/tree/setup.py#n248)
[249](https://git.launchpad.net/ufw/tree/setup.py#n249)
[250](https://git.launchpad.net/ufw/tree/setup.py#n250)
[251](https://git.launchpad.net/ufw/tree/setup.py#n251)
[252](https://git.launchpad.net/ufw/tree/setup.py#n252)
[253](https://git.launchpad.net/ufw/tree/setup.py#n253)
[254](https://git.launchpad.net/ufw/tree/setup.py#n254)
[255](https://git.launchpad.net/ufw/tree/setup.py#n255)
[256](https://git.launchpad.net/ufw/tree/setup.py#n256)
[257](https://git.launchpad.net/ufw/tree/setup.py#n257)
[258](https://git.launchpad.net/ufw/tree/setup.py#n258)
[259](https://git.launchpad.net/ufw/tree/setup.py#n259)
[260](https://git.launchpad.net/ufw/tree/setup.py#n260)
[261](https://git.launchpad.net/ufw/tree/setup.py#n261)
[262](https://git.launchpad.net/ufw/tree/setup.py#n262)
[263](https://git.launchpad.net/ufw/tree/setup.py#n263)
[264](https://git.launchpad.net/ufw/tree/setup.py#n264)
[265](https://git.launchpad.net/ufw/tree/setup.py#n265)
[266](https://git.launchpad.net/ufw/tree/setup.py#n266)
[267](https://git.launchpad.net/ufw/tree/setup.py#n267)
[268](https://git.launchpad.net/ufw/tree/setup.py#n268)
[269](https://git.launchpad.net/ufw/tree/setup.py#n269)
[270](https://git.launchpad.net/ufw/tree/setup.py#n270)
[271](https://git.launchpad.net/ufw/tree/setup.py#n271)
[272](https://git.launchpad.net/ufw/tree/setup.py#n272)
[273](https://git.launchpad.net/ufw/tree/setup.py#n273)
[274](https://git.launchpad.net/ufw/tree/setup.py#n274)
[275](https://git.launchpad.net/ufw/tree/setup.py#n275)
[276](https://git.launchpad.net/ufw/tree/setup.py#n276)
[277](https://git.launchpad.net/ufw/tree/setup.py#n277)
[278](https://git.launchpad.net/ufw/tree/setup.py#n278)
[279](https://git.launchpad.net/ufw/tree/setup.py#n279)
[280](https://git.launchpad.net/ufw/tree/setup.py#n280)
[281](https://git.launchpad.net/ufw/tree/setup.py#n281)
[282](https://git.launchpad.net/ufw/tree/setup.py#n282)
[283](https://git.launchpad.net/ufw/tree/setup.py#n283)
[284](https://git.launchpad.net/ufw/tree/setup.py#n284)
[285](https://git.launchpad.net/ufw/tree/setup.py#n285)
[286](https://git.launchpad.net/ufw/tree/setup.py#n286)
[287](https://git.launchpad.net/ufw/tree/setup.py#n287)
[288](https://git.launchpad.net/ufw/tree/setup.py#n288)
[289](https://git.launchpad.net/ufw/tree/setup.py#n289)
[290](https://git.launchpad.net/ufw/tree/setup.py#n290)
[291](https://git.launchpad.net/ufw/tree/setup.py#n291)
[292](https://git.launchpad.net/ufw/tree/setup.py#n292)
[293](https://git.launchpad.net/ufw/tree/setup.py#n293)
[294](https://git.launchpad.net/ufw/tree/setup.py#n294)
[295](https://git.launchpad.net/ufw/tree/setup.py#n295)
[296](https://git.launchpad.net/ufw/tree/setup.py#n296)
[297](https://git.launchpad.net/ufw/tree/setup.py#n297)
[298](https://git.launchpad.net/ufw/tree/setup.py#n298)
[299](https://git.launchpad.net/ufw/tree/setup.py#n299)
[300](https://git.launchpad.net/ufw/tree/setup.py#n300)
[301](https://git.launchpad.net/ufw/tree/setup.py#n301)
[302](https://git.launchpad.net/ufw/tree/setup.py#n302)
[303](https://git.launchpad.net/ufw/tree/setup.py#n303)
[304](https://git.launchpad.net/ufw/tree/setup.py#n304)
[305](https://git.launchpad.net/ufw/tree/setup.py#n305)
[306](https://git.launchpad.net/ufw/tree/setup.py#n306)
[307](https://git.launchpad.net/ufw/tree/setup.py#n307)
[308](https://git.launchpad.net/ufw/tree/setup.py#n308)
[309](https://git.launchpad.net/ufw/tree/setup.py#n309)
[310](https://git.launchpad.net/ufw/tree/setup.py#n310)
[311](https://git.launchpad.net/ufw/tree/setup.py#n311)
[312](https://git.launchpad.net/ufw/tree/setup.py#n312)
[313](https://git.launchpad.net/ufw/tree/setup.py#n313)
[314](https://git.launchpad.net/ufw/tree/setup.py#n314)
[315](https://git.launchpad.net/ufw/tree/setup.py#n315)
[316](https://git.launchpad.net/ufw/tree/setup.py#n316)
[317](https://git.launchpad.net/ufw/tree/setup.py#n317)
[318](https://git.launchpad.net/ufw/tree/setup.py#n318)
[319](https://git.launchpad.net/ufw/tree/setup.py#n319)
[320](https://git.launchpad.net/ufw/tree/setup.py#n320)
[321](https://git.launchpad.net/ufw/tree/setup.py#n321)
[322](https://git.launchpad.net/ufw/tree/setup.py#n322)
[323](https://git.launchpad.net/ufw/tree/setup.py#n323)
[324](https://git.launchpad.net/ufw/tree/setup.py#n324)
[325](https://git.launchpad.net/ufw/tree/setup.py#n325)
[326](https://git.launchpad.net/ufw/tree/setup.py#n326)
[327](https://git.launchpad.net/ufw/tree/setup.py#n327)
[328](https://git.launchpad.net/ufw/tree/setup.py#n328)
[329](https://git.launchpad.net/ufw/tree/setup.py#n329)
[330](https://git.launchpad.net/ufw/tree/setup.py#n330)
[331](https://git.launchpad.net/ufw/tree/setup.py#n331)
[332](https://git.launchpad.net/ufw/tree/setup.py#n332)
[333](https://git.launchpad.net/ufw/tree/setup.py#n333)
[334](https://git.launchpad.net/ufw/tree/setup.py#n334)
[335](https://git.launchpad.net/ufw/tree/setup.py#n335)

```

|

```
#
# ufw: front-end for Linux firewalling
#
# Copyright 2008-2021 Canonical Ltd.
#
#    This program is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License version 3,
#    as published by the Free Software Foundation.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
#
# Install with:
# python3 ./setup.py install --root="/tmp/ufw"
#
# To specify a different interpreter for ufw:
# python3.8 ./setup.py install --root="/tmp/ufw"
#

from __future__ import print_function

# Removed in 3.12 but continues to work with python3-setuptools
from distutils.command.install import install as _install
from distutils.core import setup
import errno
import os
import re
import sys
import shutil
import subprocess

ufw_version = "0.37"

def cmd(command):
    """Try to execute the given command."""
    try:
        sp = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    except OSError as e:
        return [127, str(e)]

    out = sp.communicate()[0]
    return [sp.returncode, out]

class Install(_install, object):
    """Override distutils to install the files where we want them."""

    def run(self):
        if self.home != None and self.root != None:
            print("Don't specify --home and --root at same time")
            return

        real_confdir = os.path.join("/etc")
        real_statedir = os.path.join("/lib", "ufw")
        real_prefix = self.prefix
        if self.home != None:
            real_confdir = self.home + real_confdir
            real_statedir = self.home + real_statedir
            real_prefix = self.home + "/usr"
        real_sharedir = os.path.join(real_prefix, "share", "ufw")

        # Update the modules' paths
        for fn in ["common.py"]:
            # 'staging' is used with just 'install' but build_lib is used when
            # using 'build'. We could probably override 'def build()' but this
            # at least works
            for d in [os.path.join(self.build_lib, "ufw"), "staging"]:
                f = os.path.join(d, fn)
                if not os.path.exists(f):
                    continue
                print("Updating " + f)
                subprocess.call(
                    ["sed", "-i", "s%#CONFIG_PREFIX#%" + real_confdir + "%g", f]
                )

                subprocess.call(
                    ["sed", "-i", "s%#STATE_PREFIX#%" + real_statedir + "%g", f]
                )

                subprocess.call(["sed", "-i", "s%#PREFIX#%" + real_prefix + "%g", f])

                subprocess.call(
                    ["sed", "-i", "s%#IPTABLES_DIR#%" + iptables_dir + "%g", f]
                )

                subprocess.call(
                    ["sed", "-i", "s%#SHARE_DIR#%" + real_sharedir + "%g", f]
                )

                if (
                    fn == "common.py"
                    and "UFW_SKIP_CHECKS" in os.environ
                    and os.environ["UFW_SKIP_CHECKS"] != ""
                ):
                    print("Updating do_checks")
                    subprocess.call(
                        ["sed", "-i", "s%do_checks = True%do_checks = False%g", f]
                    )

        # Now byte-compile everything
        super(Install, self).run()

        # Install script and data files
        prefix = real_prefix
        if self.root != None:
            prefix = self.root + real_prefix

        script = os.path.join(prefix, "sbin", "ufw")
        manpage = os.path.join(prefix, "share", "man", "man8", "ufw.8")
        manpage_f = os.path.join(prefix, "share", "man", "man8", "ufw-framework.8")

        for f in [script, manpage, manpage_f]:
            self.mkpath(os.path.dirname(f))

        # update the interpreter to that of the one the user specified for setup
        print("Updating staging/ufw to use %s" % (sys.executable))
        subprocess.call(
            [
                "sed",
                "-i",
                "1s%^#.*python.*%#! /usr/bin/env " + sys.executable + "%g",
                "staging/ufw",
            ]
        )
        self.copy_file("staging/ufw", script)
        self.copy_file("doc/ufw.8", manpage)
        self.copy_file("doc/ufw-framework.8", manpage_f)

        # Install state files and helper scripts
        statedir = real_statedir
        if self.root != None:
            statedir = self.root + real_statedir
        self.mkpath(statedir)

        init_helper = os.path.join(statedir, "ufw-init")
        init_helper_functions = os.path.join(statedir, "ufw-init-functions")
        self.copy_file("src/ufw-init", init_helper)
        self.copy_file("src/ufw-init-functions", init_helper_functions)

        # Install translations
        transdir = real_sharedir
        if self.root != None:
            transdir = self.root + real_sharedir
        i18ndir = os.path.join(transdir, "messages")
        self.mkpath(i18ndir)
        if len(os.listdir("locales/mo")) == 0:
            subprocess.call(["make", "mo"])
        self.copy_tree("locales/mo", i18ndir)

        # Install configuration files
        confdir = real_confdir
        if self.root != None:
            confdir = self.root + real_confdir

        defaults = os.path.join(confdir, "default", "ufw")
        ufwconf = os.path.join(confdir, "ufw", "ufw.conf")
        sysctl = os.path.join(confdir, "ufw", "sysctl.conf")
        before_rules = os.path.join(confdir, "ufw", "before.rules")
        after_rules = os.path.join(confdir, "ufw", "after.rules")
        before6_rules = os.path.join(confdir, "ufw", "before6.rules")
        after6_rules = os.path.join(confdir, "ufw", "after6.rules")
        user_rules = os.path.join(confdir, "ufw", "user.rules")
        user6_rules = os.path.join(confdir, "ufw", "user6.rules")

        apps_dir = os.path.join(confdir, "ufw", "applications.d")
        init_before_hook = os.path.join(confdir, "ufw", "before.init")
        init_after_hook = os.path.join(confdir, "ufw", "after.init")

        for f in [defaults, ufwconf]:
            self.mkpath(os.path.dirname(f))

        self.mkpath(apps_dir)
        for f in os.listdir("profiles"):
            self.copy_file(os.path.join("profiles", f), apps_dir)

        self.copy_file("conf/ufw.defaults", defaults)
        self.copy_file("conf/ufw.conf", ufwconf)
        self.copy_file("conf/sysctl.conf", sysctl)
        self.copy_file("conf/before.rules", before_rules)
        self.copy_file("conf/after.rules", after_rules)
        self.copy_file("conf/before6.rules", before6_rules)
        self.copy_file("conf/after6.rules", after6_rules)
        self.copy_file("conf/user.rules", user_rules)
        self.copy_file("conf/user6.rules", user6_rules)
        self.copy_file("src/before.init", init_before_hook)
        self.copy_file("src/after.init", init_after_hook)

        # Update the installed rules files' permissions
        for f in [
            before_rules,
            after_rules,
            before6_rules,
            after6_rules,
            user_rules,
            user6_rules,
            init_before_hook,
            init_after_hook,
        ]:
            os.chmod(f, 0o640)

        # Update the installed files' paths
        for f in [
            defaults,
            ufwconf,
            before_rules,
            after_rules,
            before6_rules,
            after6_rules,
            script,
            manpage,
            manpage_f,
            sysctl,
            init_helper,
            init_helper_functions,
            init_before_hook,
            init_after_hook,
        ]:
            print("Updating " + f)
            subprocess.call(
                ["sed", "-i", "s%#CONFIG_PREFIX#%" + real_confdir + "%g", f]
            )

            subprocess.call(["sed", "-i", "s%#PREFIX#%" + real_prefix + "%g", f])

            subprocess.call(
                ["sed", "-i", "s%#STATE_PREFIX#%" + real_statedir + "%g", f]
            )

            subprocess.call(["sed", "-i", "s%#VERSION#%" + ufw_version + "%g", f])

        # Install pristine copies of rules files
        sharedir = real_sharedir
        if self.root != None:
            sharedir = self.root + real_sharedir
        rulesdir = os.path.join(sharedir, "iptables")
        self.mkpath(rulesdir)
        for f in [
            before_rules,
            after_rules,
            before6_rules,
            after6_rules,
            user_rules,
            user6_rules,
        ]:
            self.copy_file(f, rulesdir)

if os.path.exists("staging"):
    shutil.rmtree("staging")
shutil.copytree("src", "staging")
os.unlink(os.path.join("staging", "ufw-init"))
os.unlink(os.path.join("staging", "ufw-init-functions"))

iptables_exe = ""
iptables_dir = ""

for e in ["iptables"]:
    # Historically iptables was in /sbin, then later also symlinked from
    # /usr/sbin/iptables to /sbin/iptables. Debian bullseye moves iptables
    # to /usr/sbin with no symlink in /sbin except on upgrades. To accomodate
    # buildds that may still have the old iptables, search /usr/sbin first
    for dir in [
        "/usr/sbin",
        "/sbin",
        "/usr/bin",
        "/bin",
        "/usr/local/sbin",
        "/usr/local/bin",
    ]:
        if e == "iptables":
            if os.path.exists(os.path.join(dir, e)):
                iptables_dir = dir
                iptables_exe = os.path.join(iptables_dir, "iptables")
                print("Found '%s'" % iptables_exe)
            else:
                continue

        if iptables_exe != "":
            break

if iptables_exe == "":
    print("ERROR: could not find required binary 'iptables'", file=sys.stderr)
    sys.exit(1)

for e in ["ip6tables", "iptables-restore", "ip6tables-restore"]:
    if not os.path.exists(os.path.join(iptables_dir, e)):
        print("ERROR: could not find required binary '%s'" % (e), file=sys.stderr)
        sys.exit(1)

(rc, out) = cmd([iptables_exe, "-V"])
if rc != 0:
    raise OSError(errno.ENOENT, "Could not find version for '%s'" % (iptables_exe))
version = re.sub("^v", "", str(out).split()[1])
print("Found '%s' version '%s'" % (iptables_exe, version))
if version < "1.4":
    print(
        "WARN: version '%s' has limited IPv6 support. See README for details."
        % (version),
        file=sys.stderr,
    )

setup(
    name="ufw",
    version=ufw_version,
    description="front-end for Linux firewalling",
    long_description="front-end for Linux firewalling",
    author="Jamie Strandboge",
    author_email="jdstrand@ubuntu.com",
    url="https://launchpad.net/ufw",
    license="GPL-3",
    cmdclass={"install": Install},
    package_dir={"ufw": "staging"},
    py_modules=[
        "ufw.backend",
        "ufw.backend_iptables",
        "ufw.common",
        "ufw.frontend",
        "ufw.util",
        "ufw.applications",
        "ufw.parser",
        "ufw.kernel_log_backend",
        "ufw.netfilter_log_backend",
        "ufw.log_backend",
    ],
)

shutil.rmtree("staging")

```

generated by [cgit v1.2.3](https://git.zx2c4.com/cgit/about/) ([git 2.25.1](https://git-scm.com/)) at 2026-11-03 02:57:32 +0000
