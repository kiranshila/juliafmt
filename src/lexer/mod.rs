use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

// Identifiers are pretty cursed, I generated the best regex I could to match them
// function gen_range_re(s,e)
// 	if s == e
// 		return "\\u{$(string(s,base=16))}"
// 	else
// 		return "[\\u{$(string(s,base=16))}-\\u{$(string(e,base=16))}]"
// 	end
// end
// interleave(args...) = collect(Iterators.flatten(zip(args...)))
// intersperse(A,c) = interleave(A,repeat(c,length(A)))[1:end-1]
// function gen_id_start_re()
//    range = 67:0x10FFFF
//    ranges = String[]
//    range_start = 65
//    last_good = 66
//    for c ∈ range
// 	   if Base.is_id_char(Char(c))
// 		   if c !== last_good + 1
// 			   push!(ranges,gen_range_re(range_start,last_good))
// 			   range_start = last_good = c
// 		   else
// 			   last_good = c
// 		   end
// 	   end
//    end
//    return ranges
// end
// string(intersperse(gen_id_start_re(),"|")...) |> println

// We're deriving the *Primitive stuff here to provide seamless conversion
// to rowan's types
#[derive(
    Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive, Eq, PartialOrd, Ord, Hash,
)]
#[logos(subpattern id_start_char = r"[\u{41}-\u{5a}]|\u{5f}|[\u{61}-\u{7a}]|[\u{a2}-\u{a5}]|[\u{a9}-\u{aa}]|\u{ae}|\u{b0}|\u{b5}|\u{ba}|[\u{c0}-\u{d6}]|[\u{d8}-\u{f6}]|[\u{f8}-\u{2c1}]|[\u{2c6}-\u{2d1}]|[\u{2e0}-\u{2e4}]|\u{2ec}|\u{2ee}|[\u{370}-\u{374}]|[\u{376}-\u{377}]|[\u{37a}-\u{37d}]|\u{37f}|\u{386}|[\u{388}-\u{38a}]|\u{38c}|[\u{38e}-\u{3a1}]|[\u{3a3}-\u{3f5}]|[\u{3f7}-\u{482}]|[\u{48a}-\u{52f}]|[\u{531}-\u{556}]|\u{559}|[\u{560}-\u{588}]|[\u{58d}-\u{58f}]|[\u{5d0}-\u{5ea}]|[\u{5ef}-\u{5f2}]|\u{60b}|[\u{60e}-\u{60f}]|[\u{620}-\u{64a}]|[\u{66e}-\u{66f}]|[\u{671}-\u{6d3}]|\u{6d5}|\u{6de}|[\u{6e5}-\u{6e6}]|\u{6e9}|[\u{6ee}-\u{6ef}]|[\u{6fa}-\u{6ff}]|\u{710}|[\u{712}-\u{72f}]|[\u{74d}-\u{7a5}]|\u{7b1}|[\u{7ca}-\u{7ea}]|[\u{7f4}-\u{7f6}]|\u{7fa}|[\u{7fe}-\u{815}]|\u{81a}|\u{824}|\u{828}|[\u{840}-\u{858}]|[\u{860}-\u{86a}]|[\u{870}-\u{887}]|[\u{889}-\u{88e}]|[\u{8a0}-\u{8c9}]|[\u{904}-\u{939}]|\u{93d}|\u{950}|[\u{958}-\u{961}]|[\u{971}-\u{980}]|[\u{985}-\u{98c}]|[\u{98f}-\u{990}]|[\u{993}-\u{9a8}]|[\u{9aa}-\u{9b0}]|\u{9b2}|[\u{9b6}-\u{9b9}]|\u{9bd}|\u{9ce}|[\u{9dc}-\u{9dd}]|[\u{9df}-\u{9e1}]|[\u{9f0}-\u{9f3}]|[\u{9fa}-\u{9fc}]|[\u{a05}-\u{a0a}]|[\u{a0f}-\u{a10}]|[\u{a13}-\u{a28}]|[\u{a2a}-\u{a30}]|[\u{a32}-\u{a33}]|[\u{a35}-\u{a36}]|[\u{a38}-\u{a39}]|[\u{a59}-\u{a5c}]|\u{a5e}|[\u{a72}-\u{a74}]|[\u{a85}-\u{a8d}]|[\u{a8f}-\u{a91}]|[\u{a93}-\u{aa8}]|[\u{aaa}-\u{ab0}]|[\u{ab2}-\u{ab3}]|[\u{ab5}-\u{ab9}]|\u{abd}|\u{ad0}|[\u{ae0}-\u{ae1}]|\u{af1}|\u{af9}|[\u{b05}-\u{b0c}]|[\u{b0f}-\u{b10}]|[\u{b13}-\u{b28}]|[\u{b2a}-\u{b30}]|[\u{b32}-\u{b33}]|[\u{b35}-\u{b39}]|\u{b3d}|[\u{b5c}-\u{b5d}]|[\u{b5f}-\u{b61}]|[\u{b70}-\u{b71}]|\u{b83}|[\u{b85}-\u{b8a}]|[\u{b8e}-\u{b90}]|[\u{b92}-\u{b95}]|[\u{b99}-\u{b9a}]|\u{b9c}|[\u{b9e}-\u{b9f}]|[\u{ba3}-\u{ba4}]|[\u{ba8}-\u{baa}]|[\u{bae}-\u{bb9}]|\u{bd0}|[\u{bf3}-\u{bfa}]|[\u{c05}-\u{c0c}]|[\u{c0e}-\u{c10}]|[\u{c12}-\u{c28}]|[\u{c2a}-\u{c39}]|\u{c3d}|[\u{c58}-\u{c5a}]|\u{c5d}|[\u{c60}-\u{c61}]|[\u{c7f}-\u{c80}]|[\u{c85}-\u{c8c}]|[\u{c8e}-\u{c90}]|[\u{c92}-\u{ca8}]|[\u{caa}-\u{cb3}]|[\u{cb5}-\u{cb9}]|\u{cbd}|[\u{cdd}-\u{cde}]|[\u{ce0}-\u{ce1}]|[\u{cf1}-\u{cf2}]|[\u{d04}-\u{d0c}]|[\u{d0e}-\u{d10}]|[\u{d12}-\u{d3a}]|\u{d3d}|[\u{d4e}-\u{d4f}]|[\u{d54}-\u{d56}]|[\u{d5f}-\u{d61}]|[\u{d79}-\u{d7f}]|[\u{d85}-\u{d96}]|[\u{d9a}-\u{db1}]|[\u{db3}-\u{dbb}]|\u{dbd}|[\u{dc0}-\u{dc6}]|[\u{e01}-\u{e30}]|[\u{e32}-\u{e33}]|[\u{e3f}-\u{e46}]|[\u{e81}-\u{e82}]|\u{e84}|[\u{e86}-\u{e8a}]|[\u{e8c}-\u{ea3}]|\u{ea5}|[\u{ea7}-\u{eb0}]|[\u{eb2}-\u{eb3}]|\u{ebd}|[\u{ec0}-\u{ec4}]|\u{ec6}|[\u{edc}-\u{edf}]|[\u{f00}-\u{f03}]|\u{f13}|[\u{f15}-\u{f17}]|[\u{f1a}-\u{f1f}]|\u{f34}|\u{f36}|\u{f38}|[\u{f40}-\u{f47}]|[\u{f49}-\u{f6c}]|[\u{f88}-\u{f8c}]|[\u{fbe}-\u{fc5}]|[\u{fc7}-\u{fcc}]|[\u{fce}-\u{fcf}]|[\u{fd5}-\u{fd8}]|[\u{1000}-\u{102a}]|\u{103f}|[\u{1050}-\u{1055}]|[\u{105a}-\u{105d}]|\u{1061}|[\u{1065}-\u{1066}]|[\u{106e}-\u{1070}]|[\u{1075}-\u{1081}]|\u{108e}|[\u{109e}-\u{10c5}]|\u{10c7}|\u{10cd}|[\u{10d0}-\u{10fa}]|[\u{10fc}-\u{1248}]|[\u{124a}-\u{124d}]|[\u{1250}-\u{1256}]|\u{1258}|[\u{125a}-\u{125d}]|[\u{1260}-\u{1288}]|[\u{128a}-\u{128d}]|[\u{1290}-\u{12b0}]|[\u{12b2}-\u{12b5}]|[\u{12b8}-\u{12be}]|\u{12c0}|[\u{12c2}-\u{12c5}]|[\u{12c8}-\u{12d6}]|[\u{12d8}-\u{1310}]|[\u{1312}-\u{1315}]|[\u{1318}-\u{135a}]|[\u{1380}-\u{1399}]|[\u{13a0}-\u{13f5}]|[\u{13f8}-\u{13fd}]|[\u{1401}-\u{166d}]|[\u{166f}-\u{167f}]|[\u{1681}-\u{169a}]|[\u{16a0}-\u{16ea}]|[\u{16ee}-\u{16f8}]|[\u{1700}-\u{1711}]|[\u{171f}-\u{1731}]|[\u{1740}-\u{1751}]|[\u{1760}-\u{176c}]|[\u{176e}-\u{1770}]|[\u{1780}-\u{17b3}]|\u{17d7}|[\u{17db}-\u{17dc}]|[\u{1820}-\u{1878}]|[\u{1880}-\u{1884}]|[\u{1887}-\u{18a8}]|\u{18aa}|[\u{18b0}-\u{18f5}]|[\u{1900}-\u{191e}]|\u{1940}|[\u{1950}-\u{196d}]|[\u{1970}-\u{1974}]|[\u{1980}-\u{19ab}]|[\u{19b0}-\u{19c9}]|[\u{19de}-\u{1a16}]|[\u{1a20}-\u{1a54}]|\u{1aa7}|[\u{1b05}-\u{1b33}]|[\u{1b45}-\u{1b4c}]|[\u{1b61}-\u{1b6a}]|[\u{1b74}-\u{1b7c}]|[\u{1b83}-\u{1ba0}]|[\u{1bae}-\u{1baf}]|[\u{1bba}-\u{1be5}]|[\u{1c00}-\u{1c23}]|[\u{1c4d}-\u{1c4f}]|[\u{1c5a}-\u{1c7d}]|[\u{1c80}-\u{1c88}]|[\u{1c90}-\u{1cba}]|[\u{1cbd}-\u{1cbf}]|[\u{1ce9}-\u{1cec}]|[\u{1cee}-\u{1cf3}]|[\u{1cf5}-\u{1cf6}]|\u{1cfa}|[\u{1d00}-\u{1dbf}]|[\u{1e00}-\u{1f15}]|[\u{1f18}-\u{1f1d}]|[\u{1f20}-\u{1f45}]|[\u{1f48}-\u{1f4d}]|[\u{1f50}-\u{1f57}]|\u{1f59}|\u{1f5b}|\u{1f5d}|[\u{1f5f}-\u{1f7d}]|[\u{1f80}-\u{1fb4}]|[\u{1fb6}-\u{1fbc}]|\u{1fbe}|[\u{1fc2}-\u{1fc4}]|[\u{1fc6}-\u{1fcc}]|[\u{1fd0}-\u{1fd3}]|[\u{1fd6}-\u{1fdb}]|[\u{1fe0}-\u{1fec}]|[\u{1ff2}-\u{1ff4}]|[\u{1ff6}-\u{1ffc}]|\u{2071}|[\u{207a}-\u{207f}]|[\u{208a}-\u{208e}]|[\u{2090}-\u{209c}]|[\u{20a0}-\u{20c0}]|[\u{2100}-\u{214a}]|[\u{214c}-\u{214f}]|[\u{2160}-\u{2188}]|[\u{218a}-\u{218b}]|\u{2202}|[\u{2205}-\u{2207}]|[\u{220e}-\u{2211}]|[\u{221e}-\u{2222}]|[\u{222b}-\u{2233}]|\u{223f}|[\u{22a4}-\u{22a5}]|[\u{22be}-\u{22c3}]|[\u{2300}-\u{2307}]|[\u{230c}-\u{231f}]|[\u{2322}-\u{2328}]|[\u{232b}-\u{233e}]|[\u{2340}-\u{237b}]|[\u{237d}-\u{239a}]|[\u{23b4}-\u{23db}]|[\u{23e2}-\u{2426}]|[\u{2440}-\u{244a}]|[\u{249c}-\u{24e9}]|[\u{2500}-\u{25b6}]|[\u{25b8}-\u{25c0}]|[\u{25c2}-\u{2767}]|[\u{2794}-\u{27c1}]|[\u{27d8}-\u{27d9}]|[\u{2800}-\u{28ff}]|[\u{299b}-\u{29b4}]|[\u{2a00}-\u{2a06}]|[\u{2a09}-\u{2a16}]|[\u{2a1b}-\u{2a1c}]|[\u{2b00}-\u{2b2f}]|[\u{2b45}-\u{2b46}]|[\u{2b4d}-\u{2b73}]|[\u{2b76}-\u{2b95}]|[\u{2b97}-\u{2cee}]|[\u{2cf2}-\u{2cf3}]|[\u{2d00}-\u{2d25}]|\u{2d27}|\u{2d2d}|[\u{2d30}-\u{2d67}]|\u{2d6f}|[\u{2d80}-\u{2d96}]|[\u{2da0}-\u{2da6}]|[\u{2da8}-\u{2dae}]|[\u{2db0}-\u{2db6}]|[\u{2db8}-\u{2dbe}]|[\u{2dc0}-\u{2dc6}]|[\u{2dc8}-\u{2dce}]|[\u{2dd0}-\u{2dd6}]|[\u{2dd8}-\u{2dde}]|\u{2e2f}|[\u{2e50}-\u{2e51}]|[\u{2e80}-\u{2e99}]|[\u{2e9b}-\u{2ef3}]|[\u{2f00}-\u{2fd5}]|[\u{2ff0}-\u{2ffb}]|[\u{3004}-\u{3007}]|[\u{3012}-\u{3013}]|[\u{3020}-\u{3029}]|[\u{3031}-\u{303c}]|[\u{303e}-\u{303f}]|[\u{3041}-\u{3096}]|[\u{309b}-\u{309f}]|[\u{30a1}-\u{30fa}]|[\u{30fc}-\u{30ff}]|[\u{3105}-\u{312f}]|[\u{3131}-\u{318e}]|[\u{3190}-\u{3191}]|[\u{3196}-\u{31e3}]|[\u{31f0}-\u{321e}]|[\u{322a}-\u{3247}]|\u{3250}|[\u{3260}-\u{327f}]|[\u{328a}-\u{32b0}]|[\u{32c0}-\u{a48c}]|[\u{a490}-\u{a4c6}]|[\u{a4d0}-\u{a4fd}]|[\u{a500}-\u{a60c}]|[\u{a610}-\u{a61f}]|[\u{a62a}-\u{a62b}]|[\u{a640}-\u{a66e}]|[\u{a67f}-\u{a69d}]|[\u{a6a0}-\u{a6ef}]|[\u{a717}-\u{a71f}]|[\u{a722}-\u{a788}]|[\u{a78b}-\u{a7ca}]|[\u{a7d0}-\u{a7d1}]|\u{a7d3}|[\u{a7d5}-\u{a7d9}]|[\u{a7f2}-\u{a801}]|[\u{a803}-\u{a805}]|[\u{a807}-\u{a80a}]|[\u{a80c}-\u{a822}]|[\u{a828}-\u{a82b}]|[\u{a836}-\u{a839}]|[\u{a840}-\u{a873}]|[\u{a882}-\u{a8b3}]|[\u{a8f2}-\u{a8f7}]|\u{a8fb}|[\u{a8fd}-\u{a8fe}]|[\u{a90a}-\u{a925}]|[\u{a930}-\u{a946}]|[\u{a960}-\u{a97c}]|[\u{a984}-\u{a9b2}]|\u{a9cf}|[\u{a9e0}-\u{a9e4}]|[\u{a9e6}-\u{a9ef}]|[\u{a9fa}-\u{a9fe}]|[\u{aa00}-\u{aa28}]|[\u{aa40}-\u{aa42}]|[\u{aa44}-\u{aa4b}]|[\u{aa60}-\u{aa7a}]|[\u{aa7e}-\u{aaaf}]|\u{aab1}|[\u{aab5}-\u{aab6}]|[\u{aab9}-\u{aabd}]|\u{aac0}|\u{aac2}|[\u{aadb}-\u{aadd}]|[\u{aae0}-\u{aaea}]|[\u{aaf2}-\u{aaf4}]|[\u{ab01}-\u{ab06}]|[\u{ab09}-\u{ab0e}]|[\u{ab11}-\u{ab16}]|[\u{ab20}-\u{ab26}]|[\u{ab28}-\u{ab2e}]|[\u{ab30}-\u{ab5a}]|[\u{ab5c}-\u{ab69}]|[\u{ab70}-\u{abe2}]|[\u{ac00}-\u{d7a3}]|[\u{d7b0}-\u{d7c6}]|[\u{d7cb}-\u{d7fb}]|[\u{f900}-\u{fa6d}]|[\u{fa70}-\u{fad9}]|[\u{fb00}-\u{fb06}]|[\u{fb13}-\u{fb17}]|\u{fb1d}|[\u{fb1f}-\u{fb28}]|[\u{fb2a}-\u{fb36}]|[\u{fb38}-\u{fb3c}]|\u{fb3e}|[\u{fb40}-\u{fb41}]|[\u{fb43}-\u{fb44}]|[\u{fb46}-\u{fbb1}]|[\u{fbd3}-\u{fd3d}]|[\u{fd40}-\u{fd8f}]|[\u{fd92}-\u{fdc7}]|\u{fdcf}|[\u{fdf0}-\u{fdff}]|\u{fe69}|[\u{fe70}-\u{fe74}]|[\u{fe76}-\u{fefc}]|\u{ff04}|[\u{ff21}-\u{ff3a}]|[\u{ff41}-\u{ff5a}]|[\u{ff66}-\u{ffbe}]|[\u{ffc2}-\u{ffc7}]|[\u{ffca}-\u{ffcf}]|[\u{ffd2}-\u{ffd7}]|[\u{ffda}-\u{ffdc}]|[\u{ffe0}-\u{ffe1}]|[\u{ffe4}-\u{ffe6}]|\u{ffe8}|[\u{ffed}-\u{ffee}]|[\u{10000}-\u{1000b}]|[\u{1000d}-\u{10026}]|[\u{10028}-\u{1003a}]|[\u{1003c}-\u{1003d}]|[\u{1003f}-\u{1004d}]|[\u{10050}-\u{1005d}]|[\u{10080}-\u{100fa}]|[\u{10137}-\u{10174}]|[\u{10179}-\u{10189}]|[\u{1018c}-\u{1018e}]|[\u{10190}-\u{1019c}]|\u{101a0}|[\u{101d0}-\u{101fc}]|[\u{10280}-\u{1029c}]|[\u{102a0}-\u{102d0}]|[\u{10300}-\u{1031f}]|[\u{1032d}-\u{1034a}]|[\u{10350}-\u{10375}]|[\u{10380}-\u{1039d}]|[\u{103a0}-\u{103c3}]|[\u{103c8}-\u{103cf}]|[\u{103d1}-\u{103d5}]|[\u{10400}-\u{1049d}]|[\u{104b0}-\u{104d3}]|[\u{104d8}-\u{104fb}]|[\u{10500}-\u{10527}]|[\u{10530}-\u{10563}]|[\u{10570}-\u{1057a}]|[\u{1057c}-\u{1058a}]|[\u{1058c}-\u{10592}]|[\u{10594}-\u{10595}]|[\u{10597}-\u{105a1}]|[\u{105a3}-\u{105b1}]|[\u{105b3}-\u{105b9}]|[\u{105bb}-\u{105bc}]|[\u{10600}-\u{10736}]|[\u{10740}-\u{10755}]|[\u{10760}-\u{10767}]|[\u{10780}-\u{10785}]|[\u{10787}-\u{107b0}]|[\u{107b2}-\u{107ba}]|[\u{10800}-\u{10805}]|\u{10808}|[\u{1080a}-\u{10835}]|[\u{10837}-\u{10838}]|\u{1083c}|[\u{1083f}-\u{10855}]|[\u{10860}-\u{10878}]|[\u{10880}-\u{1089e}]|[\u{108e0}-\u{108f2}]|[\u{108f4}-\u{108f5}]|[\u{10900}-\u{10915}]|[\u{10920}-\u{10939}]|[\u{10980}-\u{109b7}]|[\u{109be}-\u{109bf}]|\u{10a00}|[\u{10a10}-\u{10a13}]|[\u{10a15}-\u{10a17}]|[\u{10a19}-\u{10a35}]|[\u{10a60}-\u{10a7c}]|[\u{10a80}-\u{10a9c}]|[\u{10ac0}-\u{10ae4}]|[\u{10b00}-\u{10b35}]|[\u{10b40}-\u{10b55}]|[\u{10b60}-\u{10b72}]|[\u{10b80}-\u{10b91}]|[\u{10c00}-\u{10c48}]|[\u{10c80}-\u{10cb2}]|[\u{10cc0}-\u{10cf2}]|[\u{10d00}-\u{10d23}]|[\u{10e80}-\u{10ea9}]|[\u{10eb0}-\u{10eb1}]|[\u{10f00}-\u{10f1c}]|\u{10f27}|[\u{10f30}-\u{10f45}]|[\u{10f70}-\u{10f81}]|[\u{10fb0}-\u{10fc4}]|[\u{10fe0}-\u{10ff6}]|[\u{11003}-\u{11037}]|[\u{11071}-\u{11072}]|\u{11075}|[\u{11083}-\u{110af}]|[\u{110d0}-\u{110e8}]|[\u{11103}-\u{11126}]|\u{11144}|\u{11147}|[\u{11150}-\u{11172}]|\u{11176}|[\u{11183}-\u{111b2}]|[\u{111c1}-\u{111c4}]|\u{111da}|\u{111dc}|[\u{11200}-\u{11211}]|[\u{11213}-\u{1122b}]|[\u{11280}-\u{11286}]|\u{11288}|[\u{1128a}-\u{1128d}]|[\u{1128f}-\u{1129d}]|[\u{1129f}-\u{112a8}]|[\u{112b0}-\u{112de}]|[\u{11305}-\u{1130c}]|[\u{1130f}-\u{11310}]|[\u{11313}-\u{11328}]|[\u{1132a}-\u{11330}]|[\u{11332}-\u{11333}]|[\u{11335}-\u{11339}]|\u{1133d}|\u{11350}|[\u{1135d}-\u{11361}]|[\u{11400}-\u{11434}]|[\u{11447}-\u{1144a}]|[\u{1145f}-\u{11461}]|[\u{11480}-\u{114af}]|[\u{114c4}-\u{114c5}]|\u{114c7}|[\u{11580}-\u{115ae}]|[\u{115d8}-\u{115db}]|[\u{11600}-\u{1162f}]|\u{11644}|[\u{11680}-\u{116aa}]|\u{116b8}|[\u{11700}-\u{1171a}]|[\u{1173f}-\u{11746}]|[\u{11800}-\u{1182b}]|[\u{118a0}-\u{118df}]|[\u{118ff}-\u{11906}]|\u{11909}|[\u{1190c}-\u{11913}]|[\u{11915}-\u{11916}]|[\u{11918}-\u{1192f}]|\u{1193f}|\u{11941}|[\u{119a0}-\u{119a7}]|[\u{119aa}-\u{119d0}]|\u{119e1}|\u{119e3}|\u{11a00}|[\u{11a0b}-\u{11a32}]|\u{11a3a}|\u{11a50}|[\u{11a5c}-\u{11a89}]|\u{11a9d}|[\u{11ab0}-\u{11af8}]|[\u{11c00}-\u{11c08}]|[\u{11c0a}-\u{11c2e}]|\u{11c40}|[\u{11c72}-\u{11c8f}]|[\u{11d00}-\u{11d06}]|[\u{11d08}-\u{11d09}]|[\u{11d0b}-\u{11d30}]|\u{11d46}|[\u{11d60}-\u{11d65}]|[\u{11d67}-\u{11d68}]|[\u{11d6a}-\u{11d89}]|\u{11d98}|[\u{11ee0}-\u{11ef2}]|\u{11fb0}|[\u{11fd5}-\u{11ff1}]|[\u{12000}-\u{12399}]|[\u{12400}-\u{1246e}]|[\u{12480}-\u{12543}]|[\u{12f90}-\u{12ff0}]|[\u{13000}-\u{1342e}]|[\u{14400}-\u{14646}]|[\u{16800}-\u{16a38}]|[\u{16a40}-\u{16a5e}]|[\u{16a70}-\u{16abe}]|[\u{16ad0}-\u{16aed}]|[\u{16b00}-\u{16b2f}]|[\u{16b3c}-\u{16b43}]|\u{16b45}|[\u{16b63}-\u{16b77}]|[\u{16b7d}-\u{16b8f}]|[\u{16e40}-\u{16e7f}]|[\u{16f00}-\u{16f4a}]|\u{16f50}|[\u{16f93}-\u{16f9f}]|[\u{16fe0}-\u{16fe1}]|\u{16fe3}|[\u{17000}-\u{187f7}]|[\u{18800}-\u{18cd5}]|[\u{18d00}-\u{18d08}]|[\u{1aff0}-\u{1aff3}]|[\u{1aff5}-\u{1affb}]|[\u{1affd}-\u{1affe}]|[\u{1b000}-\u{1b122}]|[\u{1b150}-\u{1b152}]|[\u{1b164}-\u{1b167}]|[\u{1b170}-\u{1b2fb}]|[\u{1bc00}-\u{1bc6a}]|[\u{1bc70}-\u{1bc7c}]|[\u{1bc80}-\u{1bc88}]|[\u{1bc90}-\u{1bc99}]|\u{1bc9c}|[\u{1cf50}-\u{1cfc3}]|[\u{1d000}-\u{1d0f5}]|[\u{1d100}-\u{1d126}]|[\u{1d129}-\u{1d164}]|[\u{1d16a}-\u{1d16c}]|[\u{1d183}-\u{1d184}]|[\u{1d18c}-\u{1d1a9}]|[\u{1d1ae}-\u{1d1ea}]|[\u{1d200}-\u{1d241}]|\u{1d245}|[\u{1d300}-\u{1d356}]|[\u{1d400}-\u{1d454}]|[\u{1d456}-\u{1d49c}]|[\u{1d49e}-\u{1d49f}]|\u{1d4a2}|[\u{1d4a5}-\u{1d4a6}]|[\u{1d4a9}-\u{1d4ac}]|[\u{1d4ae}-\u{1d4b9}]|\u{1d4bb}|[\u{1d4bd}-\u{1d4c3}]|[\u{1d4c5}-\u{1d505}]|[\u{1d507}-\u{1d50a}]|[\u{1d50d}-\u{1d514}]|[\u{1d516}-\u{1d51c}]|[\u{1d51e}-\u{1d539}]|[\u{1d53b}-\u{1d53e}]|[\u{1d540}-\u{1d544}]|\u{1d546}|[\u{1d54a}-\u{1d550}]|[\u{1d552}-\u{1d6a5}]|[\u{1d6a8}-\u{1d7cb}]|[\u{1d7ce}-\u{1d7e1}]|[\u{1d800}-\u{1d9ff}]|[\u{1da37}-\u{1da3a}]|[\u{1da6d}-\u{1da74}]|[\u{1da76}-\u{1da83}]|[\u{1da85}-\u{1da86}]|[\u{1df00}-\u{1df1e}]|[\u{1e100}-\u{1e12c}]|[\u{1e137}-\u{1e13d}]|[\u{1e14e}-\u{1e14f}]|[\u{1e290}-\u{1e2ad}]|[\u{1e2c0}-\u{1e2eb}]|\u{1e2ff}|[\u{1e7e0}-\u{1e7e6}]|[\u{1e7e8}-\u{1e7eb}]|[\u{1e7ed}-\u{1e7ee}]|[\u{1e7f0}-\u{1e7fe}]|[\u{1e800}-\u{1e8c4}]|[\u{1e900}-\u{1e943}]|\u{1e94b}|\u{1ecac}|\u{1ecb0}|\u{1ed2e}|[\u{1ee00}-\u{1ee03}]|[\u{1ee05}-\u{1ee1f}]|[\u{1ee21}-\u{1ee22}]|\u{1ee24}|\u{1ee27}|[\u{1ee29}-\u{1ee32}]|[\u{1ee34}-\u{1ee37}]|\u{1ee39}|\u{1ee3b}|\u{1ee42}|\u{1ee47}|\u{1ee49}|\u{1ee4b}|[\u{1ee4d}-\u{1ee4f}]|[\u{1ee51}-\u{1ee52}]|\u{1ee54}|\u{1ee57}|\u{1ee59}|\u{1ee5b}|\u{1ee5d}|\u{1ee5f}|[\u{1ee61}-\u{1ee62}]|\u{1ee64}|[\u{1ee67}-\u{1ee6a}]|[\u{1ee6c}-\u{1ee72}]|[\u{1ee74}-\u{1ee77}]|[\u{1ee79}-\u{1ee7c}]|\u{1ee7e}|[\u{1ee80}-\u{1ee89}]|[\u{1ee8b}-\u{1ee9b}]|[\u{1eea1}-\u{1eea3}]|[\u{1eea5}-\u{1eea9}]|[\u{1eeab}-\u{1eebb}]|[\u{1f000}-\u{1f02b}]|[\u{1f030}-\u{1f093}]|[\u{1f0a0}-\u{1f0ae}]|[\u{1f0b1}-\u{1f0bf}]|[\u{1f0c1}-\u{1f0cf}]|[\u{1f0d1}-\u{1f0f5}]|[\u{1f10d}-\u{1f1ad}]|[\u{1f1e6}-\u{1f202}]|[\u{1f210}-\u{1f23b}]|[\u{1f240}-\u{1f248}]|[\u{1f250}-\u{1f251}]|[\u{1f260}-\u{1f265}]|[\u{1f300}-\u{1f3fa}]|[\u{1f400}-\u{1f6d7}]|[\u{1f6dd}-\u{1f6ec}]|[\u{1f6f0}-\u{1f6fc}]|[\u{1f700}-\u{1f773}]|[\u{1f780}-\u{1f7d8}]|[\u{1f7e0}-\u{1f7eb}]|\u{1f7f0}|[\u{1f800}-\u{1f80b}]|[\u{1f810}-\u{1f847}]|[\u{1f850}-\u{1f859}]|[\u{1f860}-\u{1f887}]|[\u{1f890}-\u{1f8ad}]|[\u{1f8b0}-\u{1f8b1}]|[\u{1f900}-\u{1fa53}]|[\u{1fa60}-\u{1fa6d}]|[\u{1fa70}-\u{1fa74}]|[\u{1fa78}-\u{1fa7c}]|[\u{1fa80}-\u{1fa86}]|[\u{1fa90}-\u{1faac}]|[\u{1fab0}-\u{1faba}]|[\u{1fac0}-\u{1fac5}]|[\u{1fad0}-\u{1fad9}]|[\u{1fae0}-\u{1fae7}]|[\u{1faf0}-\u{1faf6}]|[\u{1fb00}-\u{1fb92}]|[\u{1fb94}-\u{1fbca}]|[\u{20000}-\u{2a6df}]|[\u{2a700}-\u{2b738}]|[\u{2b740}-\u{2b81d}]|[\u{2b820}-\u{2cea1}]|[\u{2ceb0}-\u{2ebe0}]|[\u{2f800}-\u{2fa1d}]")]
#[logos(subpattern id_extras = r"\u{21}|[\u{30}-\u{39}]|\u{a8}|\u{af}|[\u{b2}-\u{b4}]|[\u{b8}-\u{b9}]|[\u{bc}-\u{be}]|[\u{2c2}-\u{2c5}]|[\u{2d2}-\u{2df}]|[\u{2e5}-\u{2eb}]|\u{2ed}|[\u{2ef}-\u{36f}]|\u{375}|[\u{384}-\u{385}]|[\u{483}-\u{489}]|[\u{591}-\u{5bd}]|\u{5bf}|[\u{5c1}-\u{5c2}]|[\u{5c4}-\u{5c5}]|\u{5c7}|[\u{610}-\u{61a}]|[\u{64b}-\u{669}]|\u{670}|[\u{6d6}-\u{6dc}]|[\u{6df}-\u{6e4}]|[\u{6e7}-\u{6e8}]|[\u{6ea}-\u{6ed}]|[\u{6f0}-\u{6f9}]|\u{711}|[\u{730}-\u{74a}]|[\u{7a6}-\u{7b0}]|[\u{7c0}-\u{7c9}]|[\u{7eb}-\u{7f3}]|\u{7fd}|[\u{816}-\u{819}]|[\u{81b}-\u{823}]|[\u{825}-\u{827}]|[\u{829}-\u{82d}]|[\u{859}-\u{85b}]|\u{888}|[\u{898}-\u{89f}]|[\u{8ca}-\u{8e1}]|[\u{8e3}-\u{903}]|[\u{93a}-\u{93c}]|[\u{93e}-\u{94f}]|[\u{951}-\u{957}]|[\u{962}-\u{963}]|[\u{966}-\u{96f}]|[\u{981}-\u{983}]|\u{9bc}|[\u{9be}-\u{9c4}]|[\u{9c7}-\u{9c8}]|[\u{9cb}-\u{9cd}]|\u{9d7}|[\u{9e2}-\u{9e3}]|[\u{9e6}-\u{9ef}]|[\u{9f4}-\u{9f9}]|\u{9fe}|[\u{a01}-\u{a03}]|\u{a3c}|[\u{a3e}-\u{a42}]|[\u{a47}-\u{a48}]|[\u{a4b}-\u{a4d}]|\u{a51}|[\u{a66}-\u{a71}]|\u{a75}|[\u{a81}-\u{a83}]|\u{abc}|[\u{abe}-\u{ac5}]|[\u{ac7}-\u{ac9}]|[\u{acb}-\u{acd}]|[\u{ae2}-\u{ae3}]|[\u{ae6}-\u{aef}]|[\u{afa}-\u{aff}]|[\u{b01}-\u{b03}]|\u{b3c}|[\u{b3e}-\u{b44}]|[\u{b47}-\u{b48}]|[\u{b4b}-\u{b4d}]|[\u{b55}-\u{b57}]|[\u{b62}-\u{b63}]|[\u{b66}-\u{b6f}]|[\u{b72}-\u{b77}]|\u{b82}|[\u{bbe}-\u{bc2}]|[\u{bc6}-\u{bc8}]|[\u{bca}-\u{bcd}]|\u{bd7}|[\u{be6}-\u{bf2}]|[\u{c00}-\u{c04}]|\u{c3c}|[\u{c3e}-\u{c44}]|[\u{c46}-\u{c48}]|[\u{c4a}-\u{c4d}]|[\u{c55}-\u{c56}]|[\u{c62}-\u{c63}]|[\u{c66}-\u{c6f}]|[\u{c78}-\u{c7e}]|[\u{c81}-\u{c83}]|\u{cbc}|[\u{cbe}-\u{cc4}]|[\u{cc6}-\u{cc8}]|[\u{cca}-\u{ccd}]|[\u{cd5}-\u{cd6}]|[\u{ce2}-\u{ce3}]|[\u{ce6}-\u{cef}]|[\u{d00}-\u{d03}]|[\u{d3b}-\u{d3c}]|[\u{d3e}-\u{d44}]|[\u{d46}-\u{d48}]|[\u{d4a}-\u{d4d}]|[\u{d57}-\u{d5e}]|[\u{d62}-\u{d63}]|[\u{d66}-\u{d78}]|[\u{d81}-\u{d83}]|\u{dca}|[\u{dcf}-\u{dd4}]|\u{dd6}|[\u{dd8}-\u{ddf}]|[\u{de6}-\u{def}]|[\u{df2}-\u{df3}]|\u{e31}|[\u{e34}-\u{e3a}]|[\u{e47}-\u{e4e}]|[\u{e50}-\u{e59}]|\u{eb1}|[\u{eb4}-\u{ebc}]|[\u{ec8}-\u{ecd}]|[\u{ed0}-\u{ed9}]|[\u{f18}-\u{f19}]|[\u{f20}-\u{f33}]|\u{f35}|\u{f37}|\u{f39}|[\u{f3e}-\u{f3f}]|[\u{f71}-\u{f84}]|[\u{f86}-\u{f87}]|[\u{f8d}-\u{f97}]|[\u{f99}-\u{fbc}]|\u{fc6}|[\u{102b}-\u{103e}]|[\u{1040}-\u{1049}]|[\u{1056}-\u{1059}]|[\u{105e}-\u{1060}]|[\u{1062}-\u{1064}]|[\u{1067}-\u{106d}]|[\u{1071}-\u{1074}]|[\u{1082}-\u{108d}]|[\u{108f}-\u{109d}]|[\u{135d}-\u{135f}]|[\u{1369}-\u{137c}]|[\u{1712}-\u{1715}]|[\u{1732}-\u{1734}]|[\u{1752}-\u{1753}]|[\u{1772}-\u{1773}]|[\u{17b4}-\u{17d3}]|\u{17dd}|[\u{17e0}-\u{17e9}]|[\u{17f0}-\u{17f9}]|[\u{180b}-\u{180d}]|[\u{180f}-\u{1819}]|[\u{1885}-\u{1886}]|\u{18a9}|[\u{1920}-\u{192b}]|[\u{1930}-\u{193b}]|[\u{1946}-\u{194f}]|[\u{19d0}-\u{19da}]|[\u{1a17}-\u{1a1b}]|[\u{1a55}-\u{1a5e}]|[\u{1a60}-\u{1a7c}]|[\u{1a7f}-\u{1a89}]|[\u{1a90}-\u{1a99}]|[\u{1ab0}-\u{1ace}]|[\u{1b00}-\u{1b04}]|[\u{1b34}-\u{1b44}]|[\u{1b50}-\u{1b59}]|[\u{1b6b}-\u{1b73}]|[\u{1b80}-\u{1b82}]|[\u{1ba1}-\u{1bad}]|[\u{1bb0}-\u{1bb9}]|[\u{1be6}-\u{1bf3}]|[\u{1c24}-\u{1c37}]|[\u{1c40}-\u{1c49}]|[\u{1c50}-\u{1c59}]|[\u{1cd0}-\u{1cd2}]|[\u{1cd4}-\u{1ce8}]|\u{1ced}|\u{1cf4}|[\u{1cf7}-\u{1cf9}]|[\u{1dc0}-\u{1dff}]|\u{1fbd}|[\u{1fbf}-\u{1fc1}]|[\u{1fcd}-\u{1fcf}]|[\u{1fdd}-\u{1fdf}]|[\u{1fed}-\u{1fef}]|[\u{1ffd}-\u{1ffe}]|[\u{2032}-\u{2037}]|[\u{203f}-\u{2040}]|\u{2054}|\u{2057}|\u{2070}|[\u{2074}-\u{2079}]|[\u{2080}-\u{2089}]|[\u{20d0}-\u{20f0}]|[\u{2150}-\u{215f}]|\u{2189}|[\u{2460}-\u{249b}]|[\u{24ea}-\u{24ff}]|[\u{2776}-\u{2793}]|[\u{2cef}-\u{2cf1}]|\u{2cfd}|\u{2d7f}|[\u{2de0}-\u{2dff}]|[\u{302a}-\u{302f}]|[\u{3099}-\u{309a}]|[\u{3192}-\u{3195}]|[\u{3220}-\u{3229}]|[\u{3248}-\u{324f}]|[\u{3251}-\u{325f}]|[\u{3280}-\u{3289}]|[\u{32b1}-\u{32bf}]|[\u{a620}-\u{a629}]|[\u{a66f}-\u{a672}]|[\u{a674}-\u{a67d}]|[\u{a69e}-\u{a69f}]|[\u{a6f0}-\u{a6f1}]|[\u{a700}-\u{a716}]|[\u{a720}-\u{a721}]|[\u{a789}-\u{a78a}]|\u{a802}|\u{a806}|\u{a80b}|[\u{a823}-\u{a827}]|\u{a82c}|[\u{a830}-\u{a835}]|[\u{a880}-\u{a881}]|[\u{a8b4}-\u{a8c5}]|[\u{a8d0}-\u{a8d9}]|[\u{a8e0}-\u{a8f1}]|[\u{a8ff}-\u{a909}]|[\u{a926}-\u{a92d}]|[\u{a947}-\u{a953}]|[\u{a980}-\u{a983}]|[\u{a9b3}-\u{a9c0}]|[\u{a9d0}-\u{a9d9}]|\u{a9e5}|[\u{a9f0}-\u{a9f9}]|[\u{aa29}-\u{aa36}]|\u{aa43}|[\u{aa4c}-\u{aa4d}]|[\u{aa50}-\u{aa59}]|[\u{aa7b}-\u{aa7d}]|\u{aab0}|[\u{aab2}-\u{aab4}]|[\u{aab7}-\u{aab8}]|[\u{aabe}-\u{aabf}]|\u{aac1}|[\u{aaeb}-\u{aaef}]|[\u{aaf5}-\u{aaf6}]|\u{ab5b}|[\u{ab6a}-\u{ab6b}]|[\u{abe3}-\u{abea}]|[\u{abec}-\u{abed}]|[\u{abf0}-\u{abf9}]|\u{fb1e}|[\u{fbb2}-\u{fbc2}]|[\u{fe00}-\u{fe0f}]|[\u{fe20}-\u{fe2f}]|[\u{fe33}-\u{fe34}]|[\u{fe4d}-\u{fe4f}]|[\u{ff10}-\u{ff19}]|[\u{ff3e}-\u{ff40}]|\u{ffe3}|[\u{10107}-\u{10133}]|[\u{10175}-\u{10178}]|[\u{1018a}-\u{1018b}]|\u{101fd}|[\u{102e0}-\u{102fb}]|[\u{10320}-\u{10323}]|[\u{10376}-\u{1037a}]|[\u{104a0}-\u{104a9}]|[\u{10858}-\u{1085f}]|[\u{10879}-\u{1087f}]|[\u{108a7}-\u{108af}]|[\u{108fb}-\u{108ff}]|[\u{10916}-\u{1091b}]|[\u{109bc}-\u{109bd}]|[\u{109c0}-\u{109cf}]|[\u{109d2}-\u{109ff}]|[\u{10a01}-\u{10a03}]|[\u{10a05}-\u{10a06}]|[\u{10a0c}-\u{10a0f}]|[\u{10a38}-\u{10a3a}]|[\u{10a3f}-\u{10a48}]|[\u{10a7d}-\u{10a7e}]|[\u{10a9d}-\u{10a9f}]|[\u{10ae5}-\u{10ae6}]|[\u{10aeb}-\u{10aef}]|[\u{10b58}-\u{10b5f}]|[\u{10b78}-\u{10b7f}]|[\u{10ba9}-\u{10baf}]|[\u{10cfa}-\u{10cff}]|[\u{10d24}-\u{10d27}]|[\u{10d30}-\u{10d39}]|[\u{10e60}-\u{10e7e}]|[\u{10eab}-\u{10eac}]|[\u{10f1d}-\u{10f26}]|[\u{10f46}-\u{10f54}]|[\u{10f82}-\u{10f85}]|[\u{10fc5}-\u{10fcb}]|[\u{11000}-\u{11002}]|[\u{11038}-\u{11046}]|[\u{11052}-\u{11070}]|[\u{11073}-\u{11074}]|[\u{1107f}-\u{11082}]|[\u{110b0}-\u{110ba}]|\u{110c2}|[\u{110f0}-\u{110f9}]|[\u{11100}-\u{11102}]|[\u{11127}-\u{11134}]|[\u{11136}-\u{1113f}]|[\u{11145}-\u{11146}]|\u{11173}|[\u{11180}-\u{11182}]|[\u{111b3}-\u{111c0}]|[\u{111c9}-\u{111cc}]|[\u{111ce}-\u{111d9}]|[\u{111e1}-\u{111f4}]|[\u{1122c}-\u{11237}]|\u{1123e}|[\u{112df}-\u{112ea}]|[\u{112f0}-\u{112f9}]|[\u{11300}-\u{11303}]|[\u{1133b}-\u{1133c}]|[\u{1133e}-\u{11344}]|[\u{11347}-\u{11348}]|[\u{1134b}-\u{1134d}]|\u{11357}|[\u{11362}-\u{11363}]|[\u{11366}-\u{1136c}]|[\u{11370}-\u{11374}]|[\u{11435}-\u{11446}]|[\u{11450}-\u{11459}]|\u{1145e}|[\u{114b0}-\u{114c3}]|[\u{114d0}-\u{114d9}]|[\u{115af}-\u{115b5}]|[\u{115b8}-\u{115c0}]|[\u{115dc}-\u{115dd}]|[\u{11630}-\u{11640}]|[\u{11650}-\u{11659}]|[\u{116ab}-\u{116b7}]|[\u{116c0}-\u{116c9}]|[\u{1171d}-\u{1172b}]|[\u{11730}-\u{1173b}]|[\u{1182c}-\u{1183a}]|[\u{118e0}-\u{118f2}]|[\u{11930}-\u{11935}]|[\u{11937}-\u{11938}]|[\u{1193b}-\u{1193e}]|\u{11940}|[\u{11942}-\u{11943}]|[\u{11950}-\u{11959}]|[\u{119d1}-\u{119d7}]|[\u{119da}-\u{119e0}]|\u{119e4}|[\u{11a01}-\u{11a0a}]|[\u{11a33}-\u{11a39}]|[\u{11a3b}-\u{11a3e}]|\u{11a47}|[\u{11a51}-\u{11a5b}]|[\u{11a8a}-\u{11a99}]|[\u{11c2f}-\u{11c36}]|[\u{11c38}-\u{11c3f}]|[\u{11c50}-\u{11c6c}]|[\u{11c92}-\u{11ca7}]|[\u{11ca9}-\u{11cb6}]|[\u{11d31}-\u{11d36}]|\u{11d3a}|[\u{11d3c}-\u{11d3d}]|[\u{11d3f}-\u{11d45}]|\u{11d47}|[\u{11d50}-\u{11d59}]|[\u{11d8a}-\u{11d8e}]|[\u{11d90}-\u{11d91}]|[\u{11d93}-\u{11d97}]|[\u{11da0}-\u{11da9}]|[\u{11ef3}-\u{11ef6}]|[\u{11fc0}-\u{11fd4}]|[\u{16a60}-\u{16a69}]|[\u{16ac0}-\u{16ac9}]|[\u{16af0}-\u{16af4}]|[\u{16b30}-\u{16b36}]|[\u{16b50}-\u{16b59}]|[\u{16b5b}-\u{16b61}]|[\u{16e80}-\u{16e96}]|\u{16f4f}|[\u{16f51}-\u{16f87}]|[\u{16f8f}-\u{16f92}]|\u{16fe4}|[\u{16ff0}-\u{16ff1}]|[\u{1bc9d}-\u{1bc9e}]|[\u{1cf00}-\u{1cf2d}]|[\u{1cf30}-\u{1cf46}]|[\u{1d165}-\u{1d169}]|[\u{1d16d}-\u{1d172}]|[\u{1d17b}-\u{1d182}]|[\u{1d185}-\u{1d18b}]|[\u{1d1aa}-\u{1d1ad}]|[\u{1d242}-\u{1d244}]|[\u{1d2e0}-\u{1d2f3}]|[\u{1d360}-\u{1d378}]|[\u{1d7e2}-\u{1d7ff}]|[\u{1da00}-\u{1da36}]|[\u{1da3b}-\u{1da6c}]|\u{1da75}|\u{1da84}|[\u{1da9b}-\u{1da9f}]|[\u{1daa1}-\u{1daaf}]|[\u{1e000}-\u{1e006}]|[\u{1e008}-\u{1e018}]|[\u{1e01b}-\u{1e021}]|[\u{1e023}-\u{1e024}]|[\u{1e026}-\u{1e02a}]|[\u{1e130}-\u{1e136}]|[\u{1e140}-\u{1e149}]|\u{1e2ae}|[\u{1e2ec}-\u{1e2f9}]|[\u{1e8c7}-\u{1e8d6}]|[\u{1e944}-\u{1e94a}]|[\u{1e950}-\u{1e959}]|[\u{1ec71}-\u{1ecab}]|[\u{1ecad}-\u{1ecaf}]|[\u{1ecb1}-\u{1ecb4}]|[\u{1ed01}-\u{1ed2d}]|[\u{1ed2f}-\u{1ed3d}]|[\u{1f100}-\u{1f10c}]|[\u{1f3fb}-\u{1f3ff}]|[\u{1fbf0}-\u{1fbf9}]")]
pub enum RawToken {
    // AST NODES, not lexing nodes
    Root,
    BinaryExpr,
    UnaryExpr,
    // Lexing
    #[error]
    Error,
    #[regex(r"[ \t\u{a0}]+")]
    Whitespace,
    // Numbers
    #[regex(r"[0-9]+")]
    Integer,
    #[regex(r"-?0x[0-9a-fA-F]+")]
    Hex,
    #[regex(r"-?0o[0-7]+")]
    Octal,
    #[regex(r"-?0b[01]+")]
    Binary,
    #[regex(r"([0-9]*[.][0-9]+)|([0-9]+[.][0-9]*)")]
    Float,
    #[regex(r"[eEfF][0-9]+")]
    Exponential,
    // Symbols
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    RBrace,
    #[token("}")]
    LBrace,
    #[regex(r"\r?\n")]
    Newline,
    #[token(";")]
    Semicolon,
    // Comments
    #[regex(r"#[^\r\n]*(\r\n|\n)?")]
    InlineComment,
    #[regex(r"#=([^=]|=[^#])*=#")]
    BlockComment,
    // Keywords
    #[token("baremodule")]
    Baremodule,
    #[token("begin")]
    Begin,
    #[token("break")]
    Break,
    #[token("catch")]
    Catch,
    #[token("const")]
    Const,
    #[token("continue")]
    Continue,
    #[token("do")]
    Do,
    #[token("else")]
    Else,
    #[token("elseif")]
    Elseif,
    #[token("end")]
    End,
    #[token("export")]
    Export,
    #[token("false")]
    False,
    #[token("finally")]
    Finally,
    #[token("for")]
    For,
    #[token("function")]
    Function,
    #[token("global")]
    Global,
    #[token("if")]
    If,
    #[token("import")]
    Import,
    #[token("let")]
    Let,
    #[token("local")]
    Local,
    #[token("macro")]
    Macro,
    #[token("module")]
    Module,
    #[token("quote")]
    Quote,
    #[token("return")]
    Return,
    #[token("struct")]
    Struct,
    #[token("true")]
    True,
    #[token("try")]
    Try,
    #[token("using")]
    Using,
    #[token("while")]
    While,
    // Identifiers
    #[regex(r"(?&id_start_char)((?&id_start_char)|(?&id_extras))*")]
    Identifier,
    // Operators
    #[token(".")]
    Dot,
    #[token("$")]
    Dollar,
    #[regex(r"\$=|:=|~|=|\+=|-=|−=|\*=|/=|//=|\\=|\^=|÷=|%=|<<=|>>=|>>>=|\|=|\&=|⊻=|≔|⩴|≕")]
    Assignment,
    #[token("=>")]
    Pair,
    #[token("?")]
    Conditional,
    #[regex(r"←|→|↔|↚|↛|↞|↠|↢|↣|↦|↤|↮|⇎|⇍|⇏|⇐|⇒|⇔|⇴|⇶|⇷|⇸|⇹|⇺|⇻|⇼|⇽|⇾|⇿|⟵|⟶|⟷|⟹|⟺|⟻|⟼|⟽|⟾|⟿|⤀|⤁|⤂|⤃|⤄|⤅|⤆|⤇|⤌|⤍|⤎|⤏|⤐|⤑|⤔|⤕|⤖|⤗|⤘|⤝|⤞|⤟|⤠|⥄|⥅|⥆|⥇|⥈|⥊|⥋|⥎|⥐|⥒|⥓|⥖|⥗|⥚|⥛|⥞|⥟|⥢|⥤|⥦|⥧|⥨|⥩|⥪|⥫|⥬|⥭|⥰|⧴|⬱|⬰|⬲|⬳|⬴|⬵|⬶|⬷|⬸|⬹|⬺|⬻|⬼|⬽|⬾|⬿|⭀|⭁|⭂|⭃|⭄|⭇|⭈|⭉|⭊|⭋|⭌|￩|￫|⇜|⇝|↜|↝|↩|↪|↫|↬|↼|↽|⇀|⇁|⇄|⇆|⇇|⇉|⇋|⇌|⇚|⇛|⇠|⇢|↷|↶|↺|↻|-->|<--|<-->")]
    Arrow,
    #[token("||")]
    Or,
    #[token("&&")]
    And,
    #[regex(r"in|isa|>|<|>=|≥|<=|≤|==|===|≡|!=|≠|!==|≢|∈|∉|∋|∌|⊆|⊈|⊂|⊄|⊊|∝|∊|∍|∥|∦|∷|∺|∻|∽|∾|≁|≃|≂|≄|≅|≆|≇|≈|≉|≊|≋|≌|≍|≎|≐|≑|≒|≓|≖|≗|≘|≙|≚|≛|≜|≝|≞|≟|≣|≦|≧|≨|≩|≪|≫|≬|≭|≮|≯|≰|≱|≲|≳|≴|≵|≶|≷|≸|≹|≺|≻|≼|≽|≾|≿|⊀|⊁|⊃|⊅|⊇|⊉|⊋|⊏|⊐|⊑|⊒|⊜|⊩|⊬|⊮|⊰|⊱|⊲|⊳|⊴|⊵|⊶|⊷|⋍|⋐|⋑|⋕|⋖|⋗|⋘|⋙|⋚|⋛|⋜|⋝|⋞|⋟|⋠|⋡|⋢|⋣|⋤|⋥|⋦|⋧|⋨|⋩|⋪|⋫|⋬|⋭|⋲|⋳|⋴|⋵|⋶|⋷|⋸|⋹|⋺|⋻|⋼|⋽|⋾|⋿|⟈|⟉|⟒|⦷|⧀|⧁|⧡|⧣|⧤|⧥|⩦|⩧|⩪|⩫|⩬|⩭|⩮|⩯|⩰|⩱|⩲|⩳|⩵|⩶|⩷|⩸|⩹|⩺|⩻|⩼|⩽|⩾|⩿|⪀|⪁|⪂|⪃|⪄|⪅|⪆|⪇|⪈|⪉|⪊|⪋|⪌|⪍|⪎|⪏|⪐|⪑|⪒|⪓|⪔|⪕|⪖|⪗|⪘|⪙|⪚|⪛|⪜|⪝|⪞|⪟|⪠|⪡|⪢|⪣|⪤|⪥|⪦|⪧|⪨|⪩|⪪|⪫|⪬|⪭|⪮|⪯|⪰|⪱|⪲|⪳|⪴|⪵|⪶|⪷|⪸|⪹|⪺|⪻|⪼|⪽|⪾|⪿|⫀|⫁|⫂|⫃|⫄|⫅|⫆|⫇|⫈|⫉|⫊|⫋|⫌|⫍|⫎|⫏|⫐|⫑|⫒|⫓|⫔|⫕|⫖|⫗|⫘|⫙|⫷|⫸|⫹|⫺|⊢|⊣|⟂|⫪|⫫")]
    Comparison,
    #[token("<|")]
    PipeL,
    #[token("|>")]
    PipeR,
    #[regex(r":|\.\.|…|⁝|⋮|⋱|⋰|⋯")]
    Colon,
    #[regex(r"\+|-|−|¦|\||⊕|⊖|⊞|⊟|\+\+|∪|∨|⊔|±|∓|∔|∸|≏|⊎|⊻|⊽|⋎|⋓|⧺|⧻|⨈|⨢|⨣|⨤|⨥|⨦|⨧|⨨|⨩|⨪|⨫|⨬|⨭|⨮|⨹|⨺|⩁|⩂|⩅|⩊|⩌|⩏|⩐|⩒|⩔|⩖|⩗|⩛|⩝|⩡|⩢|⩣")]
    Plus,
    #[regex(r"\*|/|⌿|÷|%|&|·|·|⋅|∘|×|\\|∩|∧|⊗|⊘|⊙|⊚|⊛|⊠|⊡|⊓|∗|∙|∤|⅋|≀|⊼|⋄|⋆|⋇|⋉|⋊|⋋|⋌|⋏|⋒|⟑|⦸|⦼|⦾|⦿|⧶|⧷|⨇|⨰|⨱|⨲|⨳|⨴|⨵|⨶|⨷|⨸|⨻|⨼|⨽|⩀|⩃|⩄|⩋|⩍|⩎|⩑|⩓|⩕|⩘|⩚|⩜|⩞|⩟|⩠|⫛|⊍|▷|⨝|⟕|⟖|⟗|⨟")]
    Times,
    #[token("//")]
    Rational,
    #[regex(r"<<|>>|>>>")]
    Bitshift,
    #[regex(r"\^|↑|↓|⇵|⟰|⟱|⤈|⤉|⤊|⤋|⤒|⤓|⥉|⥌|⥍|⥏|⥑|⥔|⥕|⥘|⥙|⥜|⥝|⥠|⥡|⥣|⥥|⥮|⥯|￪|￬")]
    Power,
    #[token("::")]
    Decl,
    #[token("->")]
    LambdaArrow,
    #[token("<:")]
    Subtype,
    #[token(">:")]
    Supertype,
    #[token("!")]
    Not,
    #[token("√")]
    Sqrt,
    // Other Literals
    #[regex(r#"'(\\.|\\[0-7]+|\\x[0-9a-fA-F]|\\[uU][0-9a-fA-F]+)'"#)]
    EscapedCharacter,
    // Kill me - this is broken when we look at the tick operator
    #[regex(r"'[\u{21}-\u{10FFFF}]'")]
    LiteralCharacter,
    #[regex(r#""(?s:[^"\\]|\\.)*""#)]
    String,
    // Hacky, the regex version doesn't work
    // r#""""(?s:.*)[^"]?""""#
    #[token(r#"""""#, |lex| {
        let len = lex.remainder().find(r#"""""#)?;
        lex.bump(len+3);
        Some(())
    })]
    TripleString,
    #[token(",")]
    Comma,
    #[token("@")]
    At,
    #[regex(r#"`(?s:[^`\\]|\\.)*`"#)]
    Cmd,
    // Cursed edge cases
    #[token(")'")]
    RParenTick,
    #[token("}'")]
    RBraceTick,
    #[token("]'")]
    RBracketTick,
    #[regex(r"(?&id_start_char)((?&id_start_char)|(?&id_extras))*'")]
    IdentifierTick,
    #[token("'")]
    Tick,
}

// We need to wrap the lexer to make it play nice with rowan
pub struct Lexer<'a> {
    pub inner: logos::Lexer<'a, RawToken>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: RawToken::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();
        Some(Self::Item { kind, text })
    }
}

#[derive(Debug, PartialEq)]
pub struct Lexeme<'a> {
    pub kind: RawToken,
    pub text: &'a str,
}
