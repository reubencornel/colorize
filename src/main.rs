use std::u8;
use std::fmt;
use std::collections::HashMap;
use std::io::{self, Read};

extern crate clap;
use clap::{Arg, App};
   

#[derive(Debug)]
struct RGBColor {
    red: u8,
    green: u8,
    blue: u8
}

const COLORS: &[&str; 1926]  = &[
    "absolutezero","#0048BA",
    "acid","#B0BF1A",
    "acidgreen","#B0BF1A",
    "aero","#7CB9E8",
    "aeroblue","#C9FFE5",
    "airsuperiorityblue","#72A0C1",
    "alabamacrimson","#AF002A",
    "alabaster","#EDEAE0",
    "aliceblue","#F0F8FF",
    "alienarmpit","#84DE02",
    "alloyorange","#C46210",
    "almond","#EFDECD",
    "amazon","#3B7A57",
    "amber","#FFBF00",
    "amethyst","#9966CC",
    "anti-flashwhite","#F2F3F4",
    "antiquebrass","#CD9575",
    "antiquebronze","#665D1E",
    "antiquefuchsia","#915C83",
    "antiqueruby","#841B2D",
    "antiquewhite","#FAEBD7",
    "ao-English","#008000",
    "applegreen","#8DB600",
    "apricot","#FBCEB1",
    "aqua","#00FFFF",
    "aquamarine","#7FFFD4",
    "arcticlime","#D0FF14",
    "armygreen","#4B5320",
    "artichoke","#8F9779",
    "arylideyellow","#E9D66B",
    "ashgray","#B2BEB5",
    "asparagus","#87A96B",
    "atomictangerine","#FF9966",
    "auburn","#A52A2A",
    "aureolin","#FDEE00",
    "avocado","#568203",
    "azure","#007FFF",
    "azuremist","#F0FFFF",
    "babyblue","#89CFF0",
    "babyblueeyes","#A1CAF1",
    "babypink","#F4C2C2",
    "babypowder","#FEFEFA",
    "baker-Millerpink","#FF91AF",
    "bananamania","#FAE7B5",
    "barbiepink","#E94196",
    "barbiepink-Pantone","#E0218A",
    "barnred","#7C0A02",
    "battleshipgrey","#848482",
    "beaublue","#BCD4E6",
    "beaver","#9F8170",
    "beige","#F5F5DC",
    "b'dazzledblue","#2E5894",
    "bigdipo’ruby","#9C2542",
    "bigfootfeet","#E88E5A",
    "bisque","#FFE4C4",
    "bistre","#3D2B1F",
    "bistrebrown","#967117",
    "bitterlemon","#CAE00D",
    "bitterlime","#BFFF00",
    "bittersweet","#FE6F5E",
    "bittersweetshimmer","#BF4F51",
    "black","#000000",
    "blackbean","#3D0C02",
    "blackchocolate","#1B1811",
    "blackcoffee","#3B2F2F",
    "blackcoral","#54626F",
    "blackolive","#3B3C36",
    "blackshadows","#BFAFB2",
    "blanchedalmond","#FFEBCD",
    "blast-offbronze","#A57164",
    "bleudefrance","#318CE7",
    "blizzardblue","#ACE5EE",
    "blond","#FAF0BE",
    "bloodred","#660000",
    "blue","#0000FF",
    "blue-crayola","#1F75FE",
    "blue-munsell","#0093AF",
    "blue-ncs","#0087BD",
    "blue-pantone","#0018A8",
    "blue-pigment","#333399",
    "blue-ryb","#0247FE",
    "bluebell","#A2A2D0",
    "blue-gray","#6699CC",
    "blue-green","#0D98BA",
    "blue-green-colorwheel","#064E40",
    "bluejeans","#5DADEC",
    "bluesapphire","#126180",
    "blue-violet","#8A2BE2",
    "blue-violet-crayola","#7366BD",
    "blue-violet-colorwheel","#4D1A7F",
    "blueyonder","#5072A7",
    "bluetiful","#3C69E7",
    "blush","#DE5D83",
    "bole","#79443B",
    "bondiblue","#0095B6",
    "bone","#E3DAC9",
    "bottlegreen","#006A4E",
    "brandy","#87413F",
    "brickred","#CB4154",
    "brightgreen","#66FF00",
    "brightlilac","#D891EF",
    "brightmaroon","#C32148",
    "brightnavyblue","#1974D2",
    "brightyellow-Crayola","#FFAA1D",
    "brilliantrose","#FF55A3",
    "brinkpink","#FB607F",
    "britishracinggreen","#004225",
    "bronze","#CD7F32",
    "brown","#88540B",
    "brownsugar","#AF6E4D",
    "brunswickgreen","#1B4D3E",
    "bubbles","#E7FEFF",
    "budgreen","#7BB661",
    "buff","#F0DC82",
    "burgundy","#800020",
    "burlywood","#DEB887",
    "burnishedbrown","#A17A74",
    "burntorange","#CC5500",
    "burntsienna","#E97451",
    "burntumber","#8A3324",
    "byzantine","#BD33A4",
    "byzantium","#702963",
    "cadet","#536872",
    "cadetblue","#5F9EA0",
    "cadetblue-crayola","#A9B2C3",
    "cadetgrey","#91A3B0",
    "cadmiumgreen","#006B3C",
    "cadmiumorange","#ED872D",
    "cadmiumred","#E30022",
    "cadmiumyellow","#FFF600",
    "cafeaulait","#A67B5B",
    "cafenoir","#4B3621",
    "cambridgeblue","#A3C1AD",
    "camel","#C19A6B",
    "cameopink","#EFBBCC",
    "canary","#FFFF99",
    "canaryyellow","#FFEF00",
    "candyapplered","#FF0800",
    "candypink","#E4717A",
    "capri","#00BFFF",
    "caputmortuum","#592720",
    "cardinal","#C41E3A",
    "caribbeangreen","#00CC99",
    "carmine","#960018",
    "carmine-M&P","#D70040",
    "carnationpink","#FFA6C9",
    "carnelian","#B31B1B",
    "carolinablue","#56A0D3",
    "carrotorange","#ED9121",
    "castletongreen","#00563F",
    "catawba","#703642",
    "cedarchest","#C95A49",
    "celadon","#ACE1AF",
    "celadonblue","#007BA7",
    "celadongreen","#2F847C",
    "celeste","#B2FFFF",
    "celticblue","#246BCE",
    "cerise","#DE3163",
    "cerulean","#007BA7",
    "ceruleanblue","#2A52BE",
    "ceruleanfrost","#6D9BC3",
    "cerulean-crayola","#1DACD6",
    "cgblue","#007AA5",
    "cgred","#E03C31",
    "champagne","#F7E7CE",
    "champagnepink","#F1DDCF",
    "charcoal","#36454F",
    "charlestongreen","#232B2B",
    "charmpink","#E68FAC",
    "chartreuse-traditional","#DFFF00",
    "chartreuse-web","#7FFF00",
    "cherryblossompink","#FFB7C5",
    "chestnut","#954535",
    "chinapink","#DE6FA1",
    "chinarose","#A8516E",
    "chinesered","#AA381E",
    "chineseviolet","#856088",
    "chineseyellow","#FFB200",
    "chocolate-traditional","#7B3F00",
    "chocolate-web","#D2691E",
    "chromeyellow","#FFA700",
    "cinereous","#98817B",
    "cinnabar","#E34234",
    "cinnamonsatin","#CD607E",
    "citrine","#E4D00A",
    "citron","#9FA91F",
    "claret","#7F1734",
    "cobaltblue","#0047AB",
    "cocoabrown","#D2691E",
    "coffee","#6F4E37",
    "columbiablue","#B9D9EB",
    "congopink","#F88379",
    "coolgrey","#8C92AC",
    "copper","#B87333",
    "copper-crayola","#DA8A67",
    "copperpenny","#AD6F69",
    "copperred","#CB6D51",
    "copperrose","#996666",
    "coquelicot","#FF3800",
    "coral","#FF7F50",
    "coralpink","#F88379",
    "cordovan","#893F45",
    "corn","#FBEC5D",
    "cornflowerblue","#6495ED",
    "cornsilk","#FFF8DC",
    "cosmiccobalt","#2E2D88",
    "cosmiclatte","#FFF8E7",
    "cosmospink","#FEBCFF",
    "coyotebrown","#81613C",
    "cottoncandy","#FFBCD9",
    "cream","#FFFDD0",
    "crimson","#DC143C",
    "cultured","#F5F5F5",
    "cyan","#00FFFF",
    "cyan-process","#00B7EB",
    "cybergrape","#58427C",
    "cyberyellow","#FFD300",
    "cyclamen","#F56FA1",
    "darkblue-gray","#666699",
    "darkbrown","#654321",
    "darkbyzantium","#5D3954",
    "darkcornflowerblue","#26428B",
    "darkcyan","#008B8B",
    "darkelectricblue","#536878",
    "darkgoldenrod","#B8860B",
    "darkgreen","#013220",
    "darkgreen-x11","#006400",
    "darkjunglegreen","#1A2421",
    "darkkhaki","#BDB76B",
    "darklava","#483C32",
    "darkliver","#534B4F",
    "darkliver-horses","#543D37",
    "darkmagenta","#8B008B",
    "darkmossgreen","#4A5D23",
    "darkolivegreen","#556B2F",
    "darkorange","#FF8C00",
    "darkorchid","#9932CC",
    "darkpastelgreen","#03C03C",
    "darkpurple","#301934",
    "darkred","#8B0000",
    "darksalmon","#E9967A",
    "darkseagreen","#8FBC8F",
    "darksienna","#3C1414",
    "darkskyblue","#8CBED6",
    "darkslateblue","#483D8B",
    "darkslategray","#2F4F4F",
    "darkspringgreen","#177245",
    "darkturquoise","#00CED1",
    "darkviolet","#9400D3",
    "dartmouthgreen","#00703C",
    "davy'sgrey","#555555",
    "deepcerise","#DA3287",
    "deepchampagne","#FAD6A5",
    "deepchestnut","#B94E48",
    "deepjunglegreen","#004B49",
    "deeppink","#FF1493",
    "deepsaffron","#FF9933",
    "deepskyblue","#00BFFF",
    "deepspacesparkle","#4A646C",
    "deeptaupe","#7E5E60",
    "denim","#1560BD",
    "denimblue","#2243B6",
    "desert","#C19A6B",
    "desertsand","#EDC9AF",
    "dimgray","#696969",
    "dingydungeon","#C53151",
    "dodgerblue","#1E90FF",
    "dogwoodrose","#D71868",
    "drab","#967117",
    "dukeblue","#00009C",
    "dutchwhite","#EFDFBB",
    "earthyellow","#E1A95F",
    "ebony","#555D50",
    "ecru","#C2B280",
    "eerieblack","#1B1B1B",
    "eggplant","#614051",
    "eggshell","#F0EAD6",
    "egyptianblue","#1034A6",
    "electricblue","#7DF9FF",
    "electricgreen","#00FF00",
    "electricindigo","#6F00FF",
    "electriclime","#CCFF00",
    "electricpurple","#BF00FF",
    "electricviolet","#8F00FF",
    "emerald","#50C878",
    "eminence","#6C3082",
    "englishgreen","#1B4D3E",
    "englishlavender","#B48395",
    "englishred","#AB4B52",
    "englishvermillion","#CC474B",
    "englishviolet","#563C5C",
    "erin","#00FF40",
    "etonblue","#96C8A2",
    "fallow","#C19A6B",
    "falured","#801818",
    "fandango","#B53389",
    "fandangopink","#DE5285",
    "fashionfuchsia","#F400A1",
    "fawn","#E5AA70",
    "feldgrau","#4D5D53",
    "ferngreen","#4F7942",
    "fielddrab","#6C541E",
    "fieryrose","#FF5470",
    "firebrick","#B22222",
    "fireenginered","#CE2029",
    "fireopal","#E95C4B",
    "flame","#E25822",
    "flax","#EEDC82",
    "flesh","#FFE9D1",
    "flickrblue","#216BD86",
    "flickrpink","#FB0081",
    "flirt","#A2006D",
    "floralwhite","#FFFAF0",
    "fluorescentblue","#15F4EE",
    "forestgreen-crayola","#5FA777",
    "forestgreen-traditional","#014421",
    "forestgreen-web","#228B22",
    "frenchbeige","#A67B5B",
    "frenchbistre","#856D4D",
    "frenchblue","#0072BB",
    "frenchfuchsia","#FD3F92",
    "frenchlilac","#86608E",
    "frenchlime","#9EFD38",
    "frenchmauve","#D473D4",
    "frenchpink","#FD6C9E",
    "frenchraspberry","#C72C48",
    "frenchrose","#F64A8A",
    "frenchskyblue","#77B5FE",
    "frenchviolet","#8806CE",
    "frostbite","#E936A7",
    "fuchsia","#FF00FF",
    "fuchsia-crayola","#C154C1",
    "fuchsiapurple","#CC397B",
    "fuchsiarose","#C74375",
    "fulvous","#E48400",
    "fuzzywuzzy","#CC6666",
    "gainsboro","#DCDCDC",
    "gamboge","#E49B0F",
    "gambogeorange-brown","#996600",
    "gargoylegas","#FFDF46",
    "genericviridian","#007F66",
    "ghostwhite","#F8F8FF",
    "glaucous","#6082B6",
    "glossygrape","#AB92B3",
    "gogreen","#00AB66",
    "gold","#A57C00",
    "gold-metallic","#D4AF37",
    "gold-web-golden","#FFD700",
    "gold-crayola","#E6BE8A",
    "goldfusion","#85754E",
    "goldenbrown","#996515",
    "goldenpoppy","#FCC200",
    "goldenyellow","#FFDF00",
    "goldenrod","#DAA520",
    "granitegray","#676767",
    "grannysmithapple","#A8E4A0",
    "gray-html-cssgray","#808080",
    "gray-x11gray","#BEBEBE",
    "green","#00FF00",
    "green-x11colorwheel","#00FF00",
    "green-crayola","#1CAC78",
    "green-html-csscolor","#008000",
    "green-munsell","#00A877",
    "green-ncs","#009F6B",
    "green-pantone","#00AD43",
    "green-pigment","#00A550",
    "green-ryb","#66B032",
    "green-blue","#1164B4",
    "green-blue-crayola","#2887C8",
    "green-cyan","#009966",
    "greenlizard","#A7F432",
    "greensheen","#6EAEA1",
    "green-yellow","#ADFF2F",
    "green-yellow-crayola","#F0E891",
    "grullo","#A99A86",
    "gunmetal","#2a3439",
    "hanblue","#446CCF",
    "hanpurple","#5218FA",
    "hansayellow","#E9D66B",
    "harlequin","#3FFF00",
    "harvestgold","#DA9100",
    "heatwave","#FF7A00",
    "heliotrope","#DF73FF",
    "heliotropegray","#AA98A9",
    "hollywoodcerise","#F400A1",
    "honeydew","#F0FFF0",
    "honolulublue","#006DB0",
    "hooker'sgreen","#49796B",
    "hotmagenta","#FF1DCE",
    "hotpink","#FF69B4",
    "huntergreen","#355E3B",
    "iceberg","#71A6D2",
    "icterine","#FCF75E",
    "illuminatingemerald","#319177",
    "imperialred","#ED2939",
    "inchworm","#B2EC5D",
    "independence","#4C516D",
    "indiagreen","#138808",
    "indianred","#CD5C5C",
    "indianyellow","#E3A857",
    "indigo","#4B0082",
    "indigoblue","#00416A",
    "indigodye","#091F92",
    "internationalkleinblue","#002FA7",
    "internationalorange-aerospace","#FF4F00",
    "internationalorange-engineering","#BA160C",
    "internationalorange-goldengatebridge","#C0362C",
    "iris","#5A4FCF",
    "irresistible","#B3446C",
    "isabelline","#F4F0EC",
    "islamicgreen","#009000",
    "italianskyblue","#B2FFFF",
    "ivory","#FFFFF0",
    "jade","#00A86B",
    "jasmine","#F8DE7E",
    "jazzberryjam","#A50B5E",
    "jet","#343434",
    "jonquil","#F4CA16",
    "junebud","#BDDA57",
    "junglegreen","#29AB87",
    "kellygreen","#4CBB17",
    "keppel","#3AB09E",
    "keylime","#E8F48C",
    "khaki-html-css-khaki","#C3B091",
    "khaki-x11-lightkhaki","#F0E68C",
    "kobe","#882D17",
    "kobi","#E79FC4",
    "kobicha","#6B4423",
    "kombugreen","#354230",
    "ksupurple","#512888",
    "languidlavender","#D6CADD",
    "lapislazuli","#26619C",
    "laserlemon","#FFFF66",
    "laurelgreen","#A9BA9D",
    "lava","#CF1020",
    "lavender-floral","#B57EDC",
    "lavender-web","#E6E6FA",
    "lavenderblue","#CCCCFF",
    "lavenderblush","#FFF0F5",
    "lavendergray","#C4C3D0",
    "lavendermagenta","#EE82EE",
    "lawngreen","#7CFC00",
    "lemon","#FFF700",
    "lemonchiffon","#FFFACD",
    "lemoncurry","#CCA01D",
    "lemonglacier","#FDFF00",
    "lemonmeringue","#F6EABE",
    "lemonyellow","#FFF44F",
    "lemonyellow-crayola","#FFFF9F",
    "liberty","#545AA7",
    "lightblue","#ADD8E6",
    "lightcoral","#F08080",
    "lightcornflowerblue","#93CCEA",
    "lightcyan","#E0FFFF",
    "lightfrenchbeige","#C8AD7F",
    "lightgoldenrodyellow","#FAFAD2",
    "lightgray","#D3D3D3",
    "lightgreen","#90EE90",
    "lightorange","#FED8B1",
    "lightperiwinkle","#C5CBE1",
    "lightpink","#FFB6C1",
    "lightsalmon","#FFA07A",
    "lightseagreen","#20B2AA",
    "lightskyblue","#87CEFA",
    "lightslategray","#778899",
    "lightsteelblue","#B0C4DE",
    "lightyellow","#FFFFE0",
    "lilac","#C8A2C8",
    "lilacluster","#AE98AA",
    "lime-colorwheel","#BFFF00",
    "lime-web-x11green","#00FF00",
    "limegreen","#32CD32",
    "lincolngreen","#195905",
    "linen","#FAF0E6",
    "lion","#C19A6B",
    "liseranpurple","#DE6FA1",
    "littleboyblue","#6CA0DC",
    "liver","#674C47",
    "liver-dogs","#B86D29",
    "liver-organ","#6C2E1F",
    "liverchestnut","#987456",
    "livid","#6699CC",
    "macaroniandcheese","#FFBD88",
    "madderlake","#CC3336",
    "magenta","#FF00FF",
    "magenta-crayola","#FF55A3",
    "magenta-dye","#CA1F7B",
    "magenta-pantone","#D0417E",
    "magenta-process","#FF0090",
    "magentahaze","#9F4576",
    "magicmint","#AAF0D1",
    "magnolia","#F8F4FF",
    "mahogany","#C04000",
    "maize","#FBEC5D",
    "maize-crayola","#F2C649",
    "majorelleblue","#6050DC",
    "malachite","#0BDA51",
    "manatee","#979AAA",
    "mandarin","#F37A48",
    "mango","#FDBE02",
    "mangotango","#FF8243",
    "mantis","#74C365",
    "mardigras","#880085",
    "marigold","#EAA221",
    "maroon-crayola","#C32148",
    "maroon-html-CSS","#800000",
    "maroon-x11","#B03060",
    "mauve","#E0B0FF",
    "mauvetaupe","#915F6D",
    "mauvelous","#EF98AA",
    "maximumblue","#47ABCC",
    "maximumbluegreen","#30BFBF",
    "maximumbluepurple","#ACACE6",
    "maximumgreen","#5E8C31",
    "maximumgreenyellow","#D9E650",
    "maximumpurple","#733380",
    "maximumred","#D92121",
    "maximumyellow","#FAFA37",
    "maximumyellowred","#F2BA49",
    "maygreen","#4C9141",
    "mayablue","#73C2FB",
    "mediumaquamarine","#66DDAA",
    "mediumblue","#0000CD",
    "mediumcandyapplered","#E2062C",
    "mediumcarmine","#AF4035",
    "mediumchampagne","#F3E5AB",
    "mediumorchid","#BA55D3",
    "mediumpurple","#9370DB",
    "mediumseagreen","#3CB371",
    "mediumslateblue","#7B68EE",
    "mediumspringgreen","#00FA9A",
    "mediumturquoise","#48D1CC",
    "mediumviolet-red","#C71585",
    "mellowapricot","#F8B878",
    "mellowyellow","#F8DE7E",
    "melancholy","#FDBCB4",
    "melon","#FEBAAD",
    "metallicgold","#D3AF37",
    "metallicseaweed","#0A7E8C",
    "metallicsunburst","#9C7C38",
    "mexicanpink","#E4007C",
    "middleblue","#7ED4E6",
    "middlebluegreen","#8DD9CC",
    "middlebluepurple","#8B72BE",
    "middlegrey","#8B8680",
    "middlegreen","#4D8C57",
    "middlegreenyellow","#ACBF60",
    "middlepurple","#D982B5",
    "middlered","#E58E73",
    "middleredpurple","#A55353",
    "middleyellow","#FFEB00",
    "middleyellowred","#ECB176",
    "midnight","#702670",
    "midnightblue","#191970",
    "midnightgreen-eaglegreen","#004953",
    "mikadoyellow","#FFC40C",
    "mimipink","#FFDAE9",
    "mindaro","#E3F988",
    "ming","#36747D",
    "minionyellow","#F5E050",
    "mint","#3EB489",
    "mintcream","#F5FFFA",
    "mintgreen","#98FF98",
    "mistymoss","#BBB477",
    "mistyrose","#FFE4E1",
    "modebeige","#967117",
    "morningblue","#8DA399",
    "mossgreen","#8A9A5B",
    "mountainmeadow","#30BA8F",
    "mountbattenpink","#997A8D",
    "msugreen","#18453B",
    "mulberry","#C54B8C",
    "mulberry-crayola","#C8509B",
    "mustard","#FFDB58",
    "myrtlegreen","#317873",
    "mystic","#D65282",
    "mysticmaroon","#AD4379",
    "nadeshikopink","#F6ADC6",
    "naplesyellow","#FADA5E",
    "navajowhite","#FFDEAD",
    "navyblue","#000080",
    "navyblue-crayola","#1974D2",
    "neonblue","#4666FF",
    "neongreen","#39FF14",
    "newyorkpink","#D7837F",
    "nickel","#727472",
    "nintendored","#E4000F",
    "non-photoblue","#A4DDED",
    "nyanza","#E9FFDB",
    "oceanblue","#4F42B5",
    "oceangreen","#48BF91",
    "ochre","#CC7722",
    "oldburgundy","#43302E",
    "oldgold","#CFB53B",
    "oldlace","#FDF5E6",
    "oldlavender","#796878",
    "oldmauve","#673147",
    "oldrose","#C08081",
    "oldsilver","#848482",
    "olive","#808000",
    "olivegreen","#B5B35C",
    "olivine","#9AB973",
    "onyx","#353839",
    "opal","#A8C3BC",
    "operamauve","#B784A7",
    "orange","#FF6600",
    "orange-colorwheel","#FF7F00",
    "orange-crayola","#FF7538",
    "orange-pantone","#FF5800",
    "orange-ryb","#FB9902",
    "orange-web","#FFA500",
    "orangepeel","#FF9F00",
    "orange-red","#FF681F",
    "orange-red-crayola","#FF5349",
    "orangesoda","#FA5B3D",
    "orange-yellow","#F5BD1F",
    "orange-yellow-crayola","#F8D568",
    "orchid","#DA70D6",
    "orchidpink","#F2BDCD",
    "orchid-crayola","#E29CD2",
    "outerspace-crayola","#2D383A",
    "outrageousorange","#FF6E4A",
    "oxblood","#800020",
    "oxfordblue","#002147",
    "oucrimsonred","#841617",
    "pacificblue","#1CA9C9",
    "pakistangreen","#006600",
    "palatinatepurple","#682860",
    "paleaqua","#BCD4E6",
    "palecerulean","#9BC4E2",
    "palepink","#FADADD",
    "palepurple-pantone","#FAE6FA",
    "palesilver","#C9C0BB",
    "palespringbud","#ECEBBD",
    "pansypurple","#78184A",
    "paoloveronesegreen","#009B7D",
    "papayawhip","#FFEFD5",
    "paradisepink","#E63E62",
    "parisgreen","#50C878",
    "pastelpink","#DEA5A4",
    "patriarch","#800080",
    "payne'sgrey","#536878",
    "peach","#FFE5B4",
    "peach","#FFCBA4",
    "peachpuff","#FFDAB9",
    "pear","#D1E231",
    "pearlypurple","#B768A2",
    "periwinkle","#CCCCFF",
    "periwinkle-crayola","#C3CDE6",
    "permanentgeraniumlake","#E12C2C",
    "persianblue","#1C39BB",
    "persiangreen","#00A693",
    "persianindigo","#32127A",
    "persianorange","#D99058",
    "persianpink","#F77FBE",
    "persianplum","#701C1C",
    "persianred","#CC3333",
    "persianrose","#FE28A2",
    "persimmon","#EC5800",
    "pewterblue","#8BA8B7",
    "phlox","#DF00FF",
    "phthaloblue","#000F89",
    "phthalogreen","#123524",
    "picoteeblue","#2E2787",
    "pictorialcarmine","#C30B4E",
    "piggypink","#FDDDE6",
    "pinegreen","#01796F",
    "pinetree","#2A2F23",
    "pink","#FFC0CB",
    "pink-pantone","#D74894",
    "pinkflamingo","#FC74FD",
    "pinklace","#FFDDF4",
    "pinklavender","#D8B2D1",
    "pinksherbet","#F78FA7",
    "pistachio","#93C572",
    "platinum","#E5E4E2",
    "plum","#8E4585",
    "plum-web","#DDA0DD",
    "plumppurple","#5946B2",
    "polishedpine","#5DA493",
    "pompandpower","#86608E",
    "popstar","#BE4F62",
    "portlandorange","#FF5A36",
    "powderblue","#B0E0E6",
    "princessperfume","#FF85CF",
    "princetonorange","#F58025",
    "prune","#701C1C",
    "prussianblue","#003153",
    "psychedelicpurple","#DF00FF",
    "puce","#CC8899",
    "pullmanbrown-upsbrown","#644117",
    "pumpkin","#FF7518",
    "purple","#6A0DAD",
    "purple-html","#800080",
    "purple-munsell","#9F00C5",
    "purple-x11","#A020F0",
    "purplemountainmajesty","#9678B6",
    "purplenavy","#4E5180",
    "purplepizzazz","#FE4EDA",
    "purpleplum","#9C51B6",
    "purpureus","#9A4EAE",
    "queenblue","#436B95",
    "queenpink","#E8CCD7",
    "quicksilver","#A6A6A6",
    "quinacridonemagenta","#8E3A59",
    "radicalred","#FF355E",
    "raisinblack","#242124",
    "rajah","#FBAB60",
    "raspberry","#E30B5D",
    "raspberryglace","#915F6D",
    "raspberryrose","#B3446C",
    "rawsienna","#D68A59",
    "rawumber","#826644",
    "razzledazzlerose","#FF33CC",
    "razzmatazz","#E3256B",
    "razzmicberry","#8D4E85",
    "rebeccapurple","#663399",
    "red","#FF0000",
    "red-crayola","#EE204D",
    "red-munsell","#F2003C",
    "red-ncs","#C40233",
    "red-pantone","#ED2939",
    "red-pigment","#ED1C24",
    "red-ryb","#FE2712",
    "red-orange","#FF5349",
    "red-orange-crayola","#FF681F",
    "red-orange-colorwheel","#FF4500",
    "red-purple","#E40078",
    "redsalsa","#FD3A4A",
    "red-violet","#C71585",
    "red-violet-crayola","#C0448F",
    "red-violet-colorwheel","#922B3E",
    "redwood","#A45A52",
    "resolutionblue","#002387",
    "rhythm","#777696",
    "richblack","#004040",
    "richblack-fogra29","#010B13",
    "richblack-fogra39","#010203",
    "riflegreen","#444C38",
    "robineggblue","#00CCCC",
    "rocketmetallic","#8A7F80",
    "romansilver","#838996",
    "rose","#FF007F",
    "rosebonbon","#F9429E",
    "rosedust","#9E5E6F",
    "roseebony","#674846",
    "rosemadder","#E32636",
    "rosepink","#FF66CC",
    "rosequartz","#AA98A9",
    "rosered","#C21E56",
    "rosetaupe","#905D5D",
    "rosevale","#AB4E52",
    "rosewood","#65000B",
    "rossocorsa","#D40000",
    "rosybrown","#BC8F8F",
    "royalblue-dark","#002366",
    "royalblue-light","#4169E1",
    "royalpurple","#7851A9",
    "royalyellow","#FADA5E",
    "ruber","#CE4676",
    "rubinered","#D10056",
    "ruby","#E0115F",
    "rubyred","#9B111E",
    "rufous","#A81C07",
    "russet","#80461B",
    "russiangreen","#679267",
    "russianviolet","#32174D",
    "rust","#B7410E",
    "rustyred","#DA2C43",
    "sacramentostategreen","#043927",
    "saddlebrown","#8B4513",
    "safetyorange","#FF7800",
    "safetyorange-blazeorange","#FF6700",
    "safetyyellow","#EED202",
    "saffron","#F4C430",
    "sage","#BCB88A",
    "st.Patrick'sblue","#23297A",
    "salmon","#FA8072",
    "salmonpink","#FF91A4",
    "sand","#C2B280",
    "sanddune","#967117",
    "sandybrown","#F4A460",
    "sapgreen","#507D2A",
    "sapphire","#0F52BA",
    "sapphireblue","#0067A5",
    "sapphire-crayola","#0067A5",
    "satinsheengold","#CBA135",
    "scarlet","#FF2400",
    "schausspink","#FF91AF",
    "schoolbusyellow","#FFD800",
    "screamin'Green","#66FF66",
    "seagreen","#2E8B57",
    "seagreen-crayola","#00FFCD",
    "sealbrown","#59260B",
    "seashell","#FFF5EE",
    "selectiveyellow","#FFBA00",
    "sepia","#704214",
    "shadow","#8A795D",
    "shadowblue","#778BA5",
    "shamrockgreen","#009E60",
    "sheengreen","#8FD400",
    "shimmeringblush","#D98695",
    "shinyshamrock","#5FA778",
    "shockingpink","#FC0FC0",
    "shockingpink-crayola","#FF6FFF",
    "sienna","#882D17",
    "silver","#C0C0C0",
    "silver-crayola","#C9C0BB",
    "silver-metallic","#AAA9AD",
    "silverchalice","#ACACAC",
    "silverpink","#C4AEAD",
    "silversand","#BFC1C2",
    "sinopia","#CB410B",
    "sizzlingred","#FF3855",
    "sizzlingsunrise","#FFDB00",
    "skobeloff","#007474",
    "skyblue","#87CEEB",
    "skyblue-crayola","#76D7EA",
    "skymagenta","#CF71AF",
    "slateblue","#6A5ACD",
    "slategray","#708090",
    "slimygreen","#299617",
    "smitten","#C84186",
    "smokyblack","#100C08",
    "snow","#FFFAFA",
    "solidpink","#893843",
    "sonicsilver","#757575",
    "spacecadet","#1D2951",
    "spanishbistre","#807532",
    "spanishblue","#0070B8",
    "spanishcarmine","#D10047",
    "spanishgray","#989898",
    "spanishgreen","#009150",
    "spanishorange","#E86100",
    "spanishpink","#F7BFBE",
    "spanishred","#E60026",
    "spanishskyblue","#00FFFF",
    "spanishviolet","#4C2882",
    "spanishviridian","#007F5C",
    "springbud","#A7FC00",
    "springfrost","#87FF2A",
    "springgreen","#00FF7F",
    "springgreen-crayola","#ECEBBD",
    "starcommandblue","#007BB8",
    "steelblue","#4682B4",
    "steelpink","#CC33CC",
    "steelteal","#5F8A8B",
    "stildegrainyellow","#FADA5E",
    "straw","#E4D96F",
    "sugarplum","#914E75",
    "sunglow","#FFCC33",
    "sunray","#E3AB57",
    "sunset","#FAD6A5",
    "superpink","#CF6BA9",
    "sweetbrown","#A83731",
    "tan","#D2B48C",
    "tan-crayola","#D99A6C",
    "tangerine","#F28500",
    "tangopink","#E4717A",
    "tartorange","#FB4D46",
    "taupe","#483C32",
    "taupegray","#8B8589",
    "teagreen","#D0F0C0",
    "tearose","#F88379",
    "tearose","#F4C2C2",
    "teal","#008080",
    "tealblue","#367588",
    "telemagenta","#CF3476",
    "tenne-tawny","#CD5700",
    "terracotta","#E2725B",
    "thistle","#D8BFD8",
    "thulianpink","#DE6FA1",
    "ticklemepink","#FC89AC",
    "tiffanyblue","#0ABAB5",
    "timberwolf","#DBD7D2",
    "titaniumyellow","#EEE600",
    "tomato","#FF6347",
    "tropicalrainforest","#00755E",
    "trueblue","#2D68C4",
    "tuftsblue","#3E8EDE",
    "tumbleweed","#DEAA88",
    "turquoise","#40E0D0",
    "turquoiseblue","#00FFEF",
    "turquoisegreen","#A0D6B4",
    "turtlegreen","#8A9A5B",
    "tuscan","#FAD6A5",
    "tuscanbrown","#6F4E37",
    "tuscanred","#7C4848",
    "tuscantan","#A67B5B",
    "tuscany","#C09999",
    "twilightlavender","#8A496B",
    "tyrianpurple","#66023C",
    "uablue","#0033AA",
    "uared","#D9004C",
    "ultramarine","#3F00FF",
    "ultramarineblue","#4166F5",
    "ultrapink","#FF6FFF",
    "ultrared","#FC6C85",
    "umber","#635147",
    "unbleachedsilk","#FFDDCA",
    "unitednationsblue","#5B92E5",
    "unmellowyellow","#FFFF66",
    "upforestgreen","#014421",
    "upmaroon","#7B1113",
    "upsdellred","#AE2029",
    "uranianblue","#AFDBF5",
    "usafablue","#004F98",
    "vandykebrown","#664228",
    "vanilla","#F3E5AB",
    "vanillaice","#F38FA9",
    "vegasgold","#C5B358",
    "venetianred","#C80815",
    "verdigris","#43B3AE",
    "vermilion","#E34234",
    "vermilion","#D9381E",
    "veronica","#A020F0",
    "violet","#8F00FF",
    "violet-colorwheel","#7F00FF",
    "violet-crayola","#963D7F",
    "violet-ryb","#8601AF",
    "violet-web","#EE82EE",
    "violet-blue","#324AB2",
    "violet-blue-crayola","#766EC8",
    "violet-red","#F75394",
    "violet-red","#891446",
    "viridian","#40826D",
    "viridiangreen","#009698",
    "vividburgundy","#9F1D35",
    "vividskyblue","#00CCFF",
    "vividtangerine","#FFA089",
    "vividviolet","#9F00FF",
    "volt","#CEFF00",
    "warmblack","#004242",
    "wheat","#F5DEB3",
    "white","#FFFFFF",
    "wildblueyonder","#A2ADD0",
    "wildorchid","#D470A2",
    "wildstrawberry","#FF43A4",
    "wildwatermelon","#FC6C85",
    "windsortan","#A75502",
    "wine","#722F37",
    "winedregs","#673147",
    "wintersky","#FF007C",
    "wintergreendream","#56887D",
    "wisteria","#C9A0DC",
    "woodbrown","#C19A6B",
    "yaleblue","#0F4D92",
    "yellow","#FFFF00",
    "yellow-crayola","#FCE883",
    "yellow-munsell","#EFCC00",
    "yellow-ncs","#FFD300",
    "yellow-pantone","#FEDF00",
    "yellow-process","#FFEF00",
    "yellow-ryb","#FEFE33",
    "yellow-green","#9ACD32",
    "yellow-green-crayola","#C5E384",
    "yellow-green-colorwheel","#30B21A",
    "yelloworange","#FFAE42",
    "yelloworange-colorwheel","#FF9505",
    "yellowsunshine","#FFF700",
    "yinmnblue","#2E5090",
    "zaffre","#0014A8",
    "zomp","#39A78E"];

const escape_key_code: [u8; 1] = [0x1b as u8];


impl RGBColor{
    fn new(red: u8, green: u8, blue: u8) -> RGBColor {
        RGBColor{
            red: red,
            green: green,
            blue: blue
        }
    }

    fn new_from_hex_str(input_str: &str) -> Result<RGBColor, String> {      
        RGBColor::new_from_str(&input_str[1..=2], &input_str[3..=4], &input_str[5..=6])
    }

    fn new_from_str(red: &str, green: &str, blue: &str) -> Result<RGBColor, String> {
        RGBColor::parse_int(red, "red")
            .and_then(|r| {RGBColor::parse_int(green, "green")
                           .and_then(|g| {RGBColor::parse_int(blue, "blue")
                                          .and_then(|b| {Ok(RGBColor::new(r, g, b))})})})
    }

    fn parse_int(input_str: &str, color: &str) -> Result<u8, String> {
        u8::from_str_radix(input_str, 16)
            .or_else(|err| Err(String::from(format!("Error parsing {}: {}", color, err))))
    }

    fn foreground_color_code(&self) -> String {
        let escape_key_string:String = String::from_utf8_lossy(&escape_key_code).into_owned();
        format!("{}[38;2;{};{};{}m", escape_key_string, self.red, self.green, self.blue)
    }

    fn background_color_code(&self) -> String {
        let escape_key_string:String = String::from_utf8_lossy(&escape_key_code).into_owned();
        format!("{}[48;2;{};{};{}m", escape_key_string, self.red, self.green, self.blue)
    }
}

fn term_reset_string() -> String {
    let escape_key_string:String = String::from_utf8_lossy(&escape_key_code).into_owned();
    format!("{}[0m", escape_key_string)
}

impl fmt::Display for RGBColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.red, self.green, self.blue)
    }
}

fn main () {
    let matches = App::new("My Colorizer Program")
        .version("0.1")
        .author("Reuben Cornel")
        .about("Colorizes regular expressions")
        .arg(Arg::with_name("foregroundColor")
             .short("f")
             .long("foregroundColor")
             .value_name("FOREGROUND_COLOR")
             .help("Sets the foreground color")
        ).arg(Arg::with_name("backgroundColor")
             .short("b")
             .long("backgroundColor")
             .value_name("BACKGROUND_COLOR")
             .help("Sets the background color")
        ).arg(Arg::with_name("inputString")
             .short("i")
             .long("inputString")
             .value_name("INPUT_STRING")
             .help("Sets the string to be colorized. If this is not set, the program tries to read a string from stdin")
             ).
        get_matches();
    
    let mut count = 0;
    let mut color_name: &str = "";
    let mut color_map = HashMap::new();
    
    for color in COLORS.iter() {
        count = count + 1;
        if count % 2 == 1 {
            color_name = color;
        }else {
            color_map.insert(color_name.to_string(), RGBColor::new_from_hex_str(color).unwrap());
        }
    }

    let input_fg_color = matches.value_of("foregroundColor").unwrap_or("");
    let fg_color = color_map.get(input_fg_color)
        .map(|x| x.foreground_color_code())
        .unwrap_or(String::from(""));
    
    let input_bg_color = matches.value_of("backgroundColor").unwrap_or("");
    let bg_color = color_map.get(input_bg_color)
        .map(|x| x.background_color_code())
        .unwrap_or(String::from(""));


    let mut buffer = String::new();
    let input_string = matches.value_of("inputString")
        .or_else(||  {
            let read_result = io::stdin().read_to_string(&mut buffer);
            if read_result.is_ok() {
                Some(&buffer)
            } else {
                None
            }
        }).unwrap_or("");

    let reset_str = term_reset_string();
    
    print!("{}{}{}{}",fg_color, bg_color, input_string, reset_str);
}
