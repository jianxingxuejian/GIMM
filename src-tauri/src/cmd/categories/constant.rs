use tauri::utils::assets::phf;

pub const CHARACTER_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "AlbedoMod" => "albedo",
  "AloyMod" => "aloy",
  "AmberMod" => "amber",
  "AmberCNMod" => "ambercn",
  "AyakaMod" => "ayaka",
  "AyatoMod" => "ayato",
  "BarbaraMod" => "barbara",
  "BarbaraSummertimeMod" => "barbarasummertime",
  "BeidouMod" => "beidou",
  "BennettMod" => "bennett",
  "CandaceMod" => "candace",
  "ChildeMod" => "childe",
  "ChongyunMod" => "chongyun",
  "ColleiMod" => "collei",
  "CynoMod" => "cyno",
  "DilucMod" => "diluc",
  "DilucFlammeMod" => "dilucflamme",
  "DionaMod" => "diona",
  "DoriMod" => "dori",
  "EulaMod" => "eula",
  "FischlMod" => "fischl",
  "FischlHighnessMod" => "fischlhighness",
  "GanyuMod" => "ganyu",
  "GorouMod" => "gorou",
  "HeizouMod" => "heizou",
  "HuTaoMod" => "hutao",
  "IttoMod" => "itto",
  "JeanMod" => "jean",
  "JeanCNMod" => "jeancn",
  "JeanSeaMod" => "jeansea",
  "KaeyaMod" => "kaeya",
  "KazuhaMod" => "kazuha",
  "KeqingMod" => "keqing",
  "KeqingOpulentMod" => "keqingopulent",
  "KleeMod" => "klee",
  "KokomiMod" => "kokomi",
  "KujouSaraMod" => "kujousara",
  "LaylaMod" => "layla",
  "LisaMod" => "lisa",
  "MonaMod" => "mona",
  "MonaCNMod" => "monacn",
  "NahidaMod" => "nahida",
  "NilouMod" => "nilou",
  "NingguangMod" => "ningguang",
  "NingguangOrchidMod" => "ningguangorchid",
  "NoelleMod" => "noelle",
  "QiqiMod" => "qiqi",
  "RaidenShogunMod" => "raidenshogun",
  "RazorMod" => "razor",
  "RosariaMod" => "rosaria",
  "RosariaCNMod" => "rosariacn",
  "SayuMod" => "sayu",
  "ShenheMod" => "shenhe",
  "ShinobuMod" => "shinobu",
  "SucroseMod" => "sucrose",
  "ThomaMod" => "thoma",
  "TighnariMod" => "tighnari",
  "TravelerBoyMod" => "travelerboy",
  "TravelerGirlMod" => "travelergirl",
  "VentiMod" => "venti",
  "XianglingMod" => "xiangling",
  "XiaoMod" => "xiao",
  "XingqiuMod" => "xingqiu",
  "XinyanMod" => "xinyan",
  "YaeMod" => "yae",
  "YanfeiMod" => "yanfei",
  "YelanMod" => "yelan",
  "YoimiyaMod" => "yoimiya",
  "YunJinMod" => "yunjin",
  "ZhongliMod" => "zhongli",
};

pub const NPC_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "AsmodayMod" => "asmoday",
  "BaizhuMod" => "baizhu",
  "BaizhuFaceMod" => "baizhuface",
  "BaronBunnyMod" => "baronbunny",
  "ChubbyMod" => "chubby",
  "DainsleifMod" => "dainsleif",
  "DainsleifFaceMod" => "dainsleifface",
  "DottoreMod" => "dottore",
  "GuobaMod" => "guoba",
  "KatheryneMod" => "katheryne",
  "LaSignoraMod" => "lasignora",
  "LaSignoraFaceMod" => "lasignoraface",
  "LoliNahidaMod" => "lolinahida",
  "OzMod" => "oz",
  "PaimonMod" => "paimon",
  "PaimonFaceMod" => "paimonface",
  "RexLapisMod" => "rexlapis",
  "ScaramoucheMod" => "scaramouche",
  "ScaramoucheFaceMod" => "scaramoucheface",
  "UshiMod" => "ushi",
};

pub const ENEMY_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "AbyssHeraldMod" => "abyssherald",
  "AbyssLectorElectroMod" => "abysslectorelectro",
  "AbyssLectorPyroMod" => "abysslectorpyro",
  "AndriusMod" => "andrius",
  "AzhdahaMod" => "azhdaha",
  "BigSlimeDendroMod" => "bigslimedendro",
  "BlackSerpentWindcutterMod" => "blackserpentwindcutter",
  "CryoHypostasisMod" => "cryohypostasis",
  "DvalinMod" => "dvalin",
  "ElectroHypostasisMod" => "electrohypostasis",
  "ElectroRegisvineMod" => "electroregisvine",
  "FatuiElectroCicinMageMod" => "fatuielectrocicinmage",
  "FatuiMirrorMaidenMod" => "fatuimirrormaiden",
  "HilichurlMod" => "hilichurl",
  "KairagiFieryMod" => "kairagifiery",
  "KairagiThunderMod" => "kairagithunder",
  "LaSignoraHarbingersMod" => "lasignoraharbingers",
  "LawachurlGeoMod" => "lawachurlgeo",
  "MagatsuMitakeNarukaminoMikotoMod" => "magatsumitakenarukaminomikoto",
  "MitachurlMod" => "mitachurl",
  "RuinGuardMod" => "ruinguard",
  "RuinHunterMod" => "ruinhunter",
  "TartagliaDelusionMod" => "tartagliadelusion",
  "TartagliaHarbingersMod" => "tartagliaharbingers",
};

pub const WEAPON_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "RavenBowMod" => "ravenbow",
  "RecurveBowMod" => "recurvebow",
  "SharpshooterOathMod" => "sharpshooteroath",
  "SlingshotMod" => "slingshot",
  "FadingTwilightMod" => "fadingtwilight",
  "FavoniusWarbowMod" => "favoniuswarbow",
  "SacrificialBowMod" => "sacrificialbow",
  "TheStringlessMod" => "thestringless",
  "AquaSimulacraMod" => "aquasimulacra",
  "ElegyForTheEndMod" => "elegyfortheend",
  "PolarStarMod" => "polarstar",
  "ThunderingPulseMod" => "thunderingpulse",
  "FrostbearerMod" => "frostbearer",
  "OathswornEyeMod" => "oathsworneye",
  "WidsithMod" => "widsith",
  "KagurasVerityMod" => "kagurasverity",
  "LostPrayerMod" => "lostprayer",
  "MemoryOfDustMod" => "memoryofdust",
  "Tullaytullah'sRemembranceMod" => "tullaytullah'sremembrance",
  "WasterGreatswordMod" => "wastergreatsword",
  "OldMercsPalMod" => "oldmercspal",
  "BloodtaintedGreatswordMod" => "bloodtaintedgreatsword",
  "DebateClubMod" => "debateclub",
  "FerrousShadowMod" => "ferrousshadow",
  "QuartzMod" => "quartz",
  "SkyriderGreatswordMod" => "skyridergreatsword",
  "WhiteIronGreatswordMod" => "whiteirongreatsword",
  "AkuoumaruMod" => "akuoumaru",
  "BellMod" => "bell",
  "BlackcliffSlasherMod" => "blackcliffslasher",
  "FavoniusGreatswordMod" => "favoniusgreatsword",
  "KatsuragikiriMod" => "katsuragikiri",
  "LithicBladeMod" => "lithicblade",
  "PrototypeArchaicMod" => "prototypearchaic",
  "RainslasherMod" => "rainslasher",
  "RoyalGreatswordMod" => "royalgreatsword",
  "SacrificialGreatswordMod" => "sacrificialgreatsword",
  "SeaLordMod" => "sealord",
  "SerpentSpineMod" => "serpentspine",
  "SnowTombedMod" => "snowtombed",
  "WhiteblindMod" => "whiteblind",
  "RedhornMod" => "redhorn",
  "SkywardPrideMod" => "skywardpride",
  "SongOfBrokenPinesMod" => "songofbrokenpines",
  "UnforgedMod" => "unforged",
  "WolfsGravestoneMod" => "wolfsgravestone",
  "IttoTaurusClaymoreMod" => "ittotaurusclaymore",
  "KairagiFierySwordMod" => "kairagifierysword",
  "KairagiThunderSwordMod" => "kairagithundersword",
  "LineBreakerMod" => "linebreaker",
  "MitaAxeMod" => "mitaaxe",
  "PolarStarsMod" => "polarstars",
  "RagingTideHarbingersMod" => "ragingtideharbingers",
  "StandardBearerMod" => "standardbearer",
  "WindcutterMod" => "windcutter",
  "BeginnersProtectorMod" => "beginnersprotector",
  "IronPointMod" => "ironpoint",
  "BlackTasselMod" => "blacktassel",
  "HalberdMod" => "halberd",
  "WhiteTasselMod" => "whitetassel",
  "BlackcliffPoleMod" => "blackcliffpole",
  "CrescentPikeMod" => "crescentpike",
  "DeathmatchMod" => "deathmatch",
  "DragonsBaneMod" => "dragonsbane",
  "DragonspineSpearMod" => "dragonspinespear",
  "FavoniusLanceMod" => "favoniuslance",
  "KitainMod" => "kitain",
  "LithicSpearMod" => "lithicspear",
  "PrototypeStarglitterMod" => "prototypestarglitter",
  "RoyalSpearMod" => "royalspear",
  "TheCatchMod" => "thecatch",
  "WavebreakersFinMod" => "wavebreakersfin",
  "CalamityQuellerMod" => "calamityqueller",
  "EngulfingLightningMod" => "engulfinglightning",
  "JadeWingedSpearMod" => "jadewingedspear",
  "SkywardSpineMod" => "skywardspine",
  "StaffOfHomaMod" => "staffofhoma",
  "VortexVanquisherMod" => "vortexvanquisher",
  "MoraxVortexMod" => "moraxvortex",
  "DullBladeMod" => "dullblade",
  "SilverSwordMod" => "silversword",
  "CoolSteelMod" => "coolsteel",
  "DarkIronSwordMod" => "darkironsword",
  "FilletBladeMod" => "filletblade",
  "HarbingerOfDawnMod" => "harbingerofdawn",
  "SkyriderSwordMod" => "skyridersword",
  "TravelersHandySwordMod" => "travelershandysword",
  "AlleyFlashMod" => "alleyflash",
  "AmenomaKageuchiMod" => "amenomakageuchi",
  "BlackcliffLongswordMod" => "blackclifflongsword",
  "BlackSwordMod" => "blacksword",
  "CinnabarSpindleMod" => "cinnabarspindle",
  "FavoniusSwordMod" => "favoniussword",
  "FesteringDesireMod" => "festeringdesire",
  "FluteMod" => "flute",
  "IronStingMod" => "ironsting",
  "KagotsurubeIsshinMod" => "kagotsurubeisshin",
  "LionsRoarMod" => "lionsroar",
  "PrototypeRancourMod" => "prototyperancour",
  "RoyalLongswordMod" => "royallongsword",
  "SacrificialSwordMod" => "sacrificialsword",
  "SwordOfDescensionMod" => "swordofdescension",
  "AquilaFavoniaMod" => "aquilafavonia",
  "FreedomSwornMod" => "freedomsworn",
  "HaranGeppakuMod" => "harangeppaku",
  "JadeCutterMod" => "jadecutter",
  "MistsplitterMod" => "mistsplitter",
  "SkywardBladeMod" => "skywardblade",
  "SummitShaperMod" => "summitshaper",
  "AetherSwordMod" => "aethersword",
  "AlhaithamswordMod" => "alhaithamsword",
  "LumineSwordMod" => "luminesword",
  "RaidenEiSwordMod" => "raideneisword",
};

pub const ENTITIES_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {};

pub const OBJECT_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {};

pub const TCGCARD_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {};