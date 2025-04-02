
#[path = "../configs/envconfig.rs"]
mod envconfig;
#[path = "./interfaceutils.rs"]
mod interfaceutils;

use envconfig::{CHAINID_MAP, ENV};
use interfaceutils::AssetInfo;

use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::time::sleep;

use std::collections::HashMap;

pub async fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

pub static SUPPORTED_PERIODS: Lazy<Vec<&'static str>> =
    Lazy::new(|| vec!["1m", "5m", "15m", "1h", "4h", "1d"]);

pub static PERIOD_MAP: Lazy<HashMap<&'static str, u64>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("1m", 60);
    map.insert("5m", 5 * 60);
    map.insert("15m", 15 * 60);
    map.insert("1h", 60 * 60);
    map.insert("4h", 4 * 60 * 60);
    map.insert("1d", 24 * 60 * 60);
    map
});

pub static SUPPORTED_TOKENS_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BTC", "ETH", "USDC", "WBTC", "WETH", "WSTETH", "AXS", "GALA", "PENDLE", "EOS", "BTT",
        "ENA", "DYDX", "BEAM", "GNO", "NEO", "ORDI", "AGIX", "EGLD", "XTZ", "RON", "SAND", "GT",
        "ENS", "WLD", "AKT", "NEXO", "ZRO", "CHZ", "SNX", "DEXE", "BNB", "SOL", "XRP", "TON",
        "DOGE", "ADA", "SHIB", "AVAX", "DOT", "LINK", "BCH", "NEAR", "UNI", "MATIC", "LEO", "LTC",
        "PEPE", "ICP", "KAS", "ETC", "APT", "RNDR", "HBAR", "ATOM", "XLM", "OKB", "FTM", "WIF",
        "ONDO", "THETA", "FLOKI", "JASMY", "NOT", "RUNE", "BONK", "BRETT", "FET", "AAVE", "TIA",
        "CORE", "PYTH", "ALGO", "SEI", "FLR", "JUP", "KCS", "FLOW", "QNT", "TRX", "BSV", "MANA",
        "XEC", "STRK", "XMR", "ARB", "MNT", "FIL", "CRO", "STX", "IMX", "MKR", "SUI", "VET", "GRT",
        "INJ", "OP", "LDO", "TAO", "AR", "BGB", "DAI",
    ]
});

pub static SUPPORTED_TOKENS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "WBTC", "WETH", "USDC", "BNB", "SOL", "XRP", "TON", "DOGE", "ADA", "TRX", "SHIB", "AVAX",
        "DOT", "LINK", "WSTETH", "BTC", "GBP", "AUD", "EUR", "NZD", "DADDY", "ORDI", "IMX", "SAND",
        "FOKI", "STX", "MERL", "CLT", "DEN", "MIA", "TPA", "MIAB", "NYC", "LAX", "SAN", "SOLB",
        "SFO", "LAS", "PIT", "PHL", "AUS", "DFW", "IAH", "ATL", "SEA", "PHX", "CHI", "BOS", "PDX",
        "WDC", "BKN", "USA", "PARIS", "LCY", "CHIR", "DENR", "USDR", "XAU", "XAG", "XPD", "XPT",
        "NG", "XBR", "ZW", "RC", "US1YTX", "US5YTX", "AU5YTRR", "GB5YTRR", "IN5YTRR",
    ]
});

pub static SUPPORTED_TOKENS_MOVE_TESTNET: Lazy<Vec<&'static str>> =
    Lazy::new(|| vec!["WETH", "ETH", "USDC", "SOL", "HYPE", "BTC", "PENDLE"]);

pub static SUPPORTED_TOKENS_BERA_TESTNET: Lazy<Vec<&'static str>> =
    Lazy::new(|| vec!["WBERA", "BERA", "WBTC", "USDC"]);

pub static SUPPORTED_TOKENS_OPTIMUS_TESTNET: Lazy<Vec<&'static str>> =
    Lazy::new(|| vec!["WEVMOS", "WBTC", "WETH", "WSTETH", "USDC"]);

pub static SUPPORTED_TOKENS: Lazy<HashMap<String, Vec<&'static str>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "arbitrum_sepolia".to_string(),
            SUPPORTED_TOKENS_ARBITRUM_SEPOLIA.clone(), 
        );
        map.insert(   
            "bitlayer_testnet".to_string(),
            SUPPORTED_TOKENS_BITLAYER_TESTNET.clone(),
        );
        map.insert("move_testnet".to_string(), SUPPORTED_TOKENS_MOVE_TESTNET.clone());
        map.insert("bera_testnet".to_string(), SUPPORTED_TOKENS_BERA_TESTNET.clone());
        map.insert(
            "optimus_testnet".to_string(),
            SUPPORTED_TOKENS_OPTIMUS_TESTNET.clone(),
        );
        map
    });

pub const PRICE_FETCH_INTERVAL: u32 = 5000;

pub static TOKEN_INFO: Lazy<HashMap<String, HashMap<String, AssetInfo>>> = Lazy::new(|| {
    let mut networks = HashMap::new();

    // Arbitrum Sepolia
    let mut arbitrum_sepolia = HashMap::new();
    //Bitlayer Testnet
    let mut bitlayer_testnet = HashMap::new();
    //Movement Testnet
    let mut move_testnet = HashMap::new();
    //Optimus Testnet
    let mut optimus_testnet = HashMap::new();
    //Bera Testnet
    let mut bera_testnet = HashMap::new();

    //Arbitrum Sepolia
    arbitrum_sepolia.insert(
        "BTC".to_string(),
        AssetInfo {
            token_address: "0x07901B404570a7f69d458d61643d34E1Cc1FfCa2".to_string(),
            token_decimals: 22,
        },
    );
    arbitrum_sepolia.insert(
        "WETH".to_string(),
        AssetInfo {
            token_address: "0x17c598522891f920D8dEdcF708644b8C0e561C61".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "USDC".to_string(),
        AssetInfo {
            token_address: "0x59d3ef0a605Cc5BA05d6D639B9A0B34675A185f2".to_string(),
            token_decimals: 24,
        },
    );
    arbitrum_sepolia.insert(
        "WBTC".to_string(),
        AssetInfo {
            token_address: "0xB52bEF69e1003781cE24D6923980c6d543a5DA8B".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ETH".to_string(),
        AssetInfo {
            token_address: "0x0000000000000000000000000000000000000000".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "WSTETH".to_string(),
        AssetInfo {
            token_address: "0x9FdCf7481E5A078e3608AfDD65D87cEA457Ae832".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "AXS".to_string(),
        AssetInfo {
            token_address: "0xf924F05d55548D89bd77ddF952bceB10e2e71803".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "GALA".to_string(),
        AssetInfo {
            token_address: "0x3C18dC18EF8a3Eea8ffC19962C1e88c1bB6AA44a".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "PENDLE".to_string(),
        AssetInfo {
            token_address: "0xa9892f03250b483F0f02A06dfF4b4c766726e0D2".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "EOS".to_string(),
        AssetInfo {
            token_address: "0x5D40d7E2e1e1Edf68e069FCFBAD966A49D3b85B4".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "BTT".to_string(),
        AssetInfo {
            token_address: "0x1FFeC6106759703203F55D806db38CD92034a166".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ENA".to_string(),
        AssetInfo {
            token_address: "0x60d19dbb55Bac6d317440e4da74632e156Ac521F".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "DYDX".to_string(),
        AssetInfo {
            token_address: "0xdFc5C995817b48Bbea1F4d1A683F764f00a2f275".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "BEAM".to_string(),
        AssetInfo {
            token_address: "0xA7Ed90816352B032fbB362A5A1A5E6F319450113".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "GNO".to_string(),
        AssetInfo {
            token_address: "0xff6E85aD22999E8ad4F5d10803ADd246cD25ac8D".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "NEO".to_string(),
        AssetInfo {
            token_address: "0x6C60410C1d8DAfdf5Be6c5ce9FaA4D296275ad26".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ORDI".to_string(),
        AssetInfo {
            token_address: "0xfF82f18e8e0850312B3eAea1f2e6B1790588A36e".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "AGIX".to_string(),
        AssetInfo {
            token_address: "0xEd92b6cCed2e53df0dd4d6Ae47b8Cb6B4520A4a8".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "EGLD".to_string(),
        AssetInfo {
            token_address: "0x6b6e64F0a3fEe6ae3f6e53AaF2dA78798FDa5C99".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "XTZ".to_string(),
        AssetInfo {
            token_address: "0x417F4B0040bbF5d1742161608ca66C3433792d97".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "RON".to_string(),
        AssetInfo {
            token_address: "0xFaa6951d669eB2227497935c02f2e27582fe3a04".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "SAND".to_string(),
        AssetInfo {
            token_address: "0x3A16A757da06cDEB900ff067f23454716f2A578F".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "GT".to_string(),
        AssetInfo {
            token_address: "0x5dAe62F22d5A5DAe3Ef9500981c2d729c34d7fE1".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ENS".to_string(),
        AssetInfo {
            token_address: "0x86e7e835DadC2dEB74E888E9B961ba06F27aEDE8".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "WLD".to_string(),
        AssetInfo {
            token_address: "0x0Deec82419B894080E2073F73C34ebEf940F1335".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "AKT".to_string(),
        AssetInfo {
            token_address: "0x1bFB8494ad2a8f80A8936C06E96922447b917443".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "NEXO".to_string(),
        AssetInfo {
            token_address: "0x0024Ae56b013aC452755641573c51BCd0A80B9D0".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ZRO".to_string(),
        AssetInfo {
            token_address: "0xD1FcC11726438979DA049846b62580618fDD9BfF".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "CHZ".to_string(),
        AssetInfo {
            token_address: "0x8032fb4F1057D7DcbF9E14f304Fec57A57477d18".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "SNX".to_string(),
        AssetInfo {
            token_address: "0xC4f5913cb1121F00AE9b3634084e8AE1c1Ac8505".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "DEXE".to_string(),
        AssetInfo {
            token_address: "0xbAd31db9E947f3913386b8d8d0E8DA90aaa0fbc0".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "BNB".to_string(),
        AssetInfo {
            token_address: "0x61D093cB9DDADaE5d5594e6B274F5C794698a909".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "SOL".to_string(),
        AssetInfo {
            token_address: "0x56748ABA4AF984E9e26194d87d6E1DeEFB461843".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "XRP".to_string(),
        AssetInfo {
            token_address: "0xa426Ac377E11FD0c98c9f75139eFbEfe25Bf2B92".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "TON".to_string(),
        AssetInfo {
            token_address: "0xDb07624BBC5C9Fc33be44599684f35e1e2D06D84".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "DOGE".to_string(),
        AssetInfo {
            token_address: "0x0B7424Db97bC2cfC94F5011F64113Ae95CcFb3f0".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ADA".to_string(),
        AssetInfo {
            token_address: "0x614deDB18E0A480a06A3a878AcA1dB299036E6dE".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "SHIB".to_string(),
        AssetInfo {
            token_address: "0x5c5BF6e9B9b7DDd83D133591e5E5a5F087a6eADF".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "AVAX".to_string(),
        AssetInfo {
            token_address: "0xf2212d8993341e509E089644fF3256ac7EAb5e4a".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "DOT".to_string(),
        AssetInfo {
            token_address: "0xCF7294D6f49b32fe8c45d11D5FF148994896235c".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "LINK".to_string(),
        AssetInfo {
            token_address: "0x1907Fa55cB79e5FC5725DA9F7E202b350b93729b".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "BCH".to_string(),
        AssetInfo {
            token_address: "0x5Ac8377Dc17ad5fB27914b12E9006698Ad329527".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "NEAR".to_string(),
        AssetInfo {
            token_address: "0x62D7ea42d03Df11004a8f68747766B7BaF6C2683".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "UNI".to_string(),
        AssetInfo {
            token_address: "0xc21f75211c9334a10BC7AA5b9Daab1b219758cD7".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "MATIC".to_string(),
        AssetInfo {
            token_address: "0x31dFb3c1237d69BE39cd26d44a7A05E4397DDb18".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "LEO".to_string(),
        AssetInfo {
            token_address: "0x61265b4B5E624a20D624C5CB335417b3a27eC9A2".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "LTC".to_string(),
        AssetInfo {
            token_address: "0x58f437a3557FD8C1D9810F3B5B8C179d9F8594A1".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "PEPE".to_string(),
        AssetInfo {
            token_address: "0xe88c4C3F4D0C23DD6bE06934116Bc5634b4BbC5D".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ICP".to_string(),
        AssetInfo {
            token_address: "0xEfC41915d14433b9b8c62bC965966A8F9bfe1D5C".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "KAS".to_string(),
        AssetInfo {
            token_address: "0x04D3341A2676DD4B1FFD559Ab9B9cAab44fdD8BE".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ETC".to_string(),
        AssetInfo {
            token_address: "0xD25134a484d08556153B6280ec32B028c9748F5A".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "APT".to_string(),
        AssetInfo {
            token_address: "0x381a53a0a1d08Bf7eE2A987C919e203C13e8f1EE".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "RNDR".to_string(),
        AssetInfo {
            token_address: "0xd684Ce7289eb3d45b4BD7deFF9EBf4aac69De0b3".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "HBAR".to_string(),
        AssetInfo {
            token_address: "0xD00fca9027e420F267a7B8A0e0BA668612a85ba9".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ATOM".to_string(),
        AssetInfo {
            token_address: "0x9b8C5F8Ede545d8255f68AC78aB716d3142d46F5".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "XLM".to_string(),
        AssetInfo {
            token_address: "0x17bF6b2804898b625c57FC4467b77D96c8D2b7F5".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "OKB".to_string(),
        AssetInfo {
            token_address: "0x12c45AEC05DD42155010eA97180527D39c82b6a4".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "FTM".to_string(),
        AssetInfo {
            token_address: "0x5bA03F6BCDEdFe14d5BA9C055188dc2e9494f35E".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "WIF".to_string(),
        AssetInfo {
            token_address: "0x5B01750F634cfdB0DA82A42449dfB620274A6354".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ONDO".to_string(),
        AssetInfo {
            token_address: "0x94E2CBFC42670e515D373ad00155D3Eef6c0E88B".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "THETA".to_string(),
        AssetInfo {
            token_address: "0xaD95cb41AdF3fD8C1309142Cf6C2383E2126df69".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "FLOKI".to_string(),
        AssetInfo {
            token_address: "0x755Ab39BFb4449F34d788FA800bE132a463419E4".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "JASMY".to_string(),
        AssetInfo {
            token_address: "0x27cDE14D8f7c039cb4118C2459544a882a11D2c5".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "NOT".to_string(),
        AssetInfo {
            token_address: "0x67c5B0484Aee2aae307DdF829c3A5F5B37492Cea".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "RUNE".to_string(),
        AssetInfo {
            token_address: "0x88EB2cdFF197F90e3518e71E83da34F3Fcac6525".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "BONK".to_string(),
        AssetInfo {
            token_address: "0xa9e646EEd585c3d46Df96Ed614F7E4347Ad9eb6d".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "BRETT".to_string(),
        AssetInfo {
            token_address: "0xd426c0381AC4Ef31cE5577c38C3921a70e6DA6e3".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "FET".to_string(),
        AssetInfo {
            token_address: "0xE72E5F5C3B00B9A4824f70FcbA343b8696083c45".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "AAVE".to_string(),
        AssetInfo {
            token_address: "0x9f5137c5E912Ab3B829c6A948Bd1081a199F5Bfb".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "TIA".to_string(),
        AssetInfo {
            token_address: "0x390CDc297A150834739E45c35929b9CF83d59c41".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "CORE".to_string(),
        AssetInfo {
            token_address: "0x31B953E39977eB207cd1c6ADCC9D92c2b1963d51".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "PYTH".to_string(),
        AssetInfo {
            token_address: "0x02643d8B20368643fA240dAf1616EFe0e261Ae3b".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ALGO".to_string(),
        AssetInfo {
            token_address: "0x65698624D816DAb41B90a38369FCA6C1807ca3Ad".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "SEI".to_string(),
        AssetInfo {
            token_address: "0x54a0951c293CC4195ff86c691A8797fa0908ca65".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "FLR".to_string(),
        AssetInfo {
            token_address: "0x869CB7A9deDd7c374e4Ec1dA716cA786F7B81D53".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "JUP".to_string(),
        AssetInfo {
            token_address: "0x695cAC0Bf1e1F697c451CAc8Dd7a769c3E898520".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "KCS".to_string(),
        AssetInfo {
            token_address: "0x56f425eCc334209A8d66DeD87754587477717627".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "FLOW".to_string(),
        AssetInfo {
            token_address: "0xA8f737101a8d6d5636B75dff28A7E3929a68A17b".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "QNT".to_string(),
        AssetInfo {
            token_address: "0xeA4F3cbAc79a55E34976727267cf02be154B31b3".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "TRX".to_string(),
        AssetInfo {
            token_address: "0xf6Bb954b4e22769a2179E2EC59CA0a09CcCf11b4".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "BSV".to_string(),
        AssetInfo {
            token_address: "0x64D772a029E4D0b566097b407B453Da22bFfB4A8".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "MANA".to_string(),
        AssetInfo {
            token_address: "0x6b8CC4e77B23D97614BE1b02652995f3B49b457D".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "XEC".to_string(),
        AssetInfo {
            token_address: "0x806C5A2de7EE9856bE7aDEc0Ea621373d171f8f4".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "STRK".to_string(),
        AssetInfo {
            token_address: "0xf05e109449d444eD2c3C9023B09c09e4C769766e".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "XMR".to_string(),
        AssetInfo {
            token_address: "0x5cD0001b75F118479256c35D20Ddb50C7372cF77".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "ARB".to_string(),
        AssetInfo {
            token_address: "0x1b1C63eFDBfbeAC9a635D4b6936024F60937588e".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "MNT".to_string(),
        AssetInfo {
            token_address: "0x2aDc9d2c46c6ccA1C29f25C7fd1c5fa1A3301809".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "FIL".to_string(),
        AssetInfo {
            token_address: "0x7347f6798D83ac00D66AE1d9CDBdc68d9bAd50CB".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "CRO".to_string(),
        AssetInfo {
            token_address: "0xD1978f505af8Df649EeB447Ae3385C7165490D93".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "STX".to_string(),
        AssetInfo {
            token_address: "0x1a62D0Fd4d4d546a736Fd8689299C32E165bC2EC".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "IMX".to_string(),
        AssetInfo {
            token_address: "0x21f4ABC8D401Ed7497EF80A45F9572bB6220B3E8".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "MKR".to_string(),
        AssetInfo {
            token_address: "0xeb9EFcf40c4A88c13A9f0C7A6c3367bF562dc651".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "SUI".to_string(),
        AssetInfo {
            token_address: "0x99FcC8fe0A4fD9576CB3B2654CeB7D7bA27Ac46B".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "VET".to_string(),
        AssetInfo {
            token_address: "0xaC1E8dE32d605aFCcb78dF2F45aadA47aB3F5E96".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "GRT".to_string(),
        AssetInfo {
            token_address: "0x3fF71850dBa53d3D60Da81c1B1E0e7C4537e74Ef".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "INJ".to_string(),
        AssetInfo {
            token_address: "0x7de36603cEa21e8b01973515C4bFD116B3E99e6f".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "OP".to_string(),
        AssetInfo {
            token_address: "0x1653fcBD4771201B39F25e444608841852018Ee0".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "LDO".to_string(),
        AssetInfo {
            token_address: "0x7D59122Bf941E2D9BFFc4157Db7Ee8dc5AA7A0CC".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "TAO".to_string(),
        AssetInfo {
            token_address: "0x950C34cFc01EdE4289b64e1F30Db00D1c88b25Bf".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "AR".to_string(),
        AssetInfo {
            token_address: "0x5dBB54ab6225789f25a22aD3686B1440B4119D6F".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "BGB".to_string(),
        AssetInfo {
            token_address: "0xD5d61A121bA6713d7C1a16e92565c95F42C17B18".to_string(),
            token_decimals: 12,
        },
    );
    arbitrum_sepolia.insert(
        "DAI".to_string(),
        AssetInfo {
            token_address: "0x01e1351cb07492a0495B6Cd853A665335e0e57a8".to_string(),
            token_decimals: 12,
        },
    );

    //Bitlayer Testnet
    bitlayer_testnet.insert(
        "WBTC".to_string(),
        AssetInfo {
            token_address: "0x313ea66A1f508B5F2825A626F7a09afeaBE594E1".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "WETH".to_string(),
        AssetInfo {
            token_address: "0xbB506faEA96E0329F5Bb5552182DB0b245413A2A".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "USDC".to_string(),
        AssetInfo {
            token_address: "0xAc53dF4Adf5edBE466fAF2242277E238E52D466B".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "BNB".to_string(),
        AssetInfo {
            token_address: "0x4a43089c6620b75ed2C965AB6d41c62E80E2954B".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SOL".to_string(),
        AssetInfo {
            token_address: "0x3E0911d6C0fa146C3006a2bc186c815fcF566243".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XRP".to_string(),
        AssetInfo {
            token_address: "0xe44e93ce209D44B32263e9487Bb4cDe6507Df44F".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "TON".to_string(),
        AssetInfo {
            token_address: "0x5775376a92236BDCB5018655cc12C752c00Fe0b2".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DOGE".to_string(),
        AssetInfo {
            token_address: "0xEdf90028e93aD936cC2329985e61E1a3C5Dcf0E4".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "ADA".to_string(),
        AssetInfo {
            token_address: "0x2c77158726951b5558fe0EbE314093375270c60f".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "TRX".to_string(),
        AssetInfo {
            token_address: "0xf716602066127E0a34Af03fc19531F0f6318c21A".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SHIB".to_string(),
        AssetInfo {
            token_address: "0xf51201e9BaEBBD71fE94DDAc3d4d9fF287667C60".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "AVAX".to_string(),
        AssetInfo {
            token_address: "0xfc73a37Ae52679e7cDb6554B0bE5d0E3100b03D2".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DOT".to_string(),
        AssetInfo {
            token_address: "0x363E6Ba86D5a723C0C7d7fD9B360091293bc0019".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "LINK".to_string(),
        AssetInfo {
            token_address: "0x1D43079cb02ffeF63b3e44fdc89dc7B485189711".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "WSTETH".to_string(),
        AssetInfo {
            token_address: "0xc060E5567B4F15473b646Cb5b3bbb977eAdA7BFD".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "BTC".to_string(),
        AssetInfo {
            token_address: "0x0000000000000000000000000000000000000000".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "GBP".to_string(),
        AssetInfo {
            token_address: "0x67E4ce304Bfa9A553d400Af72499c887f7c63858".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "AUD".to_string(),
        AssetInfo {
            token_address: "0xEDFB276A1df4f41FbB4E850e2B5ea92269a83d1E".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "EUR".to_string(),
        AssetInfo {
            token_address: "0x46e3F674d0E9ED964d71355175c511B2FC8B5E6F".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "NZD".to_string(),
        AssetInfo {
            token_address: "0x985CA067d5B4ab4a627658D5fC6c6f181978C24a".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "DADDY".to_string(),
        AssetInfo {
            token_address: "0x4Da72d32492c472C5EC0e2f066F570b227662D37".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "ORDI".to_string(),
        AssetInfo {
            token_address: "0xB7c1b5C1F1dcBa1c87EE0112d61e610F9696bbD7".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "IMX".to_string(),
        AssetInfo {
            token_address: "0x65efc0e6d3c468CeC23550e9C8c8C6F8DEfb7182".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "SAND".to_string(),
        AssetInfo {
            token_address: "0x02111877265f31056b9F3CC07036AD6b9C45F315".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "FOKI".to_string(),
        AssetInfo {
            token_address: "0x3a75478554Ee965bb17f004e4b6ACCc325DE7b5A".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "STX".to_string(),
        AssetInfo {
            token_address: "0xdE60936B0999aB2B0E92311Af12558ae89D675e5".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "MERL".to_string(),
        AssetInfo {
            token_address: "0xA411E9dDF2cc927bf61Af1f738ff4402c2f83c15".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "CLT".to_string(),
        AssetInfo {
            token_address: "0x595D59862c153e7885FD5Fb75eE7aDDf5f8768EB".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DEN".to_string(),
        AssetInfo {
            token_address: "0xB887E13ee67e63b9AFcC11136583E3e792C49003".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "MIA".to_string(),
        AssetInfo {
            token_address: "0xc9CB6cC9637c08509002EDBeAB0330CDD4A9D333".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "TPA".to_string(),
        AssetInfo {
            token_address: "0x253bd01F34cD1b5CdAEd6F74847eD9D13DD301fE".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "MIAB".to_string(),
        AssetInfo {
            token_address: "0xD77aDF1F36BB0422665eD11B1E09BD5CBe052004".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "NYC".to_string(),
        AssetInfo {
            token_address: "0xc9Be08D31324B7F19b9f2B4814693372F49D0adD".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "LAX".to_string(),
        AssetInfo {
            token_address: "0xb775Bc8970fDD0b140c619b27a3db6E3fefea25A".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SAN".to_string(),
        AssetInfo {
            token_address: "0x98dD36702A96FB835c4b7Dc17D88881c54eDa703".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SOLB".to_string(),
        AssetInfo {
            token_address: "0xC6767F79E9d1AC6624344E2Bb4fbE74e37F73f93".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SFO".to_string(),
        AssetInfo {
            token_address: "0xF99319EDf73D2E3589225336EB93a812d42BD321".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "LAS".to_string(),
        AssetInfo {
            token_address: "0xd9312cd0294b15daC2b1baEC59E9A36329dB308a".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PIT".to_string(),
        AssetInfo {
            token_address: "0x0e60C3462a74613a9A130fB65859dd4582D27629".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PHL".to_string(),
        AssetInfo {
            token_address: "0x029E230d0a9BfdfB90d688c951d7A315351E613b".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "AUS".to_string(),
        AssetInfo {
            token_address: "0xCACdF34c7acF35bc003be751Aed4D06143Bab4c8".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DFW".to_string(),
        AssetInfo {
            token_address: "0xe0B572D890b7E4c32CF121EFb6f917Fb4c3C0CEe".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "IAH".to_string(),
        AssetInfo {
            token_address: "0x9DFd7D76be1BF7d01201d87338dc41C4ea481550".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "ATL".to_string(),
        AssetInfo {
            token_address: "0xf82AFB93331DC5Ce40C959Bd7440cbb69785E534".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SEA".to_string(),
        AssetInfo {
            token_address: "0x8f80cdF35C50Be0Aa27C540baEd35CEE25c49F95".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PHX".to_string(),
        AssetInfo {
            token_address: "0x0489E7605186fDAc0ec80Bf99087EAA8e0382a40".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "CHI".to_string(),
        AssetInfo {
            token_address: "0x83246B9DC962605Cfb65cb64161c691B37B67065".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "BOS".to_string(),
        AssetInfo {
            token_address: "0x67f17cca1337C4bFaa844139f934908ecf984422".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PDX".to_string(),
        AssetInfo {
            token_address: "0xCbD0bcE5FE20f5683C84CbA45E645e6F2ffea5cC".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "WDC".to_string(),
        AssetInfo {
            token_address: "0x5D7Ee3e6a2465780B2a6cB0970E0Ca85D1063530".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "BKN".to_string(),
        AssetInfo {
            token_address: "0x2bc801c518931fC8B5f540d58694DC5402Ff5eAF".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "USA".to_string(),
        AssetInfo {
            token_address: "0x99e95495CBB9ddc59E1538E14F60eCAC1aAc45aD".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PARIS".to_string(),
        AssetInfo {
            token_address: "0x6aa8143ADa3A3A1273da05742Ecf5DCcC911900D".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "LCY".to_string(),
        AssetInfo {
            token_address: "0xC45e45E9e525A380047adC22794Bd79E43095D84".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "CHIR".to_string(),
        AssetInfo {
            token_address: "0x6c1061Db0FCE9435736718f3371A75da8b7d3E5E".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DENR".to_string(),
        AssetInfo {
            token_address: "0xEC141D6C7C4010846417978D8f7D4674Ec400325".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "USDR".to_string(),
        AssetInfo {
            token_address: "0x8054112cB526CF10a8f1c97977D6572a39Fe5eAE".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XAU".to_string(),
        AssetInfo {
            token_address: "0xaF2f1f3173EEC6d9530A3C2cBAA26799571E1a22".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XAG".to_string(),
        AssetInfo {
            token_address: "0x578a1EC389D490953a38AE77D1f2afcBE306d4BF".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XPD".to_string(),
        AssetInfo {
            token_address: "0xE75d46a10D91Af64Dd7978E0768d972F93105cC9".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XPT".to_string(),
        AssetInfo {
            token_address: "0xE89Cf36052d112BC022C01f3aC2b2D3BF641572f".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "NG".to_string(),
        AssetInfo {
            token_address: "0x7859b0eE14cDcfEB75B131D80BBF949b6db26640".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XBR".to_string(),
        AssetInfo {
            token_address: "0x9E0BCa368e2a4f45f94c1631762Af051EB08ebcF".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "ZW".to_string(),
        AssetInfo {
            token_address: "0x6Db75b0B23abC9bC180E97BB1cDd6c11287CC2fD".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "RC".to_string(),
        AssetInfo {
            token_address: "0xF7864154A0f619818cB0602BbaF076c7AAf09AA3".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "US1YTX".to_string(),
        AssetInfo {
            token_address: "0x43eCd0A6C256BDfb93E13364b5f56aDDB5E71305".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "US5YTX".to_string(),
        AssetInfo {
            token_address: "0x7c4a61b22cd1b4f9a8CAA4ACFad541b793796A2A".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "AU5YTRR".to_string(),
        AssetInfo {
            token_address: "0x202Fc380Af3f773a2ff70CDeAc5A23048BE0514c".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "GB5YTRR".to_string(),
        AssetInfo {
            token_address: "0x777234DC4Efd2587dD277EA8838DbaE62E4c339b".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "IN5YTRR".to_string(),
        AssetInfo {
            token_address: "0x9563Bf94a640dd9c8c6C6563e3e3a034184AC673".to_string(),
            token_decimals: 12,
        },
    );

    //Movement Testnet
    move_testnet.insert(
        "BTC".to_string(),
        AssetInfo {
            token_address: "0x369c1691F18F9A87312585b92C13Dd93048437Dc".to_string(),
            token_decimals: 12,
        },
    );
    move_testnet.insert(
        "WETH".to_string(),
        AssetInfo {
            token_address: "0x1FCC1D1ed9d73a66Bc94F6D0DE523BC5ED63DDE6".to_string(),
            token_decimals: 12,
        },
    );
    move_testnet.insert(
        "ETH".to_string(),
        AssetInfo {
            token_address: "0x0000000000000000000000000000000000000000".to_string(),
            token_decimals: 12,
        },
    );
    move_testnet.insert(
        "USDC".to_string(),
        AssetInfo {
            token_address: "0x37e99578B73975f3f5D24AD3EA39C64325D53CC3".to_string(),
            token_decimals: 24,
        },
    );
    move_testnet.insert(
        "HYPE".to_string(),
        AssetInfo {
            token_address: "0x5fa05EF3B8EB435E28Ba8e430DB211542feD25CE".to_string(),
            token_decimals: 12,
        },
    );
    move_testnet.insert(
        "SOL".to_string(),
        AssetInfo {
            token_address: "0x1179CEBDd1e60DfFBFCf56553fFb25A09FAa3acD".to_string(),
            token_decimals: 12,
        },
    );

    // Bera Testnet
    bera_testnet.insert(
        "WBERA".to_string(),
        AssetInfo {
            token_address: "0x3b626A75A78265B481B6b7Fa1e038407FE73645c".to_string(),
            token_decimals: 12,
        },
    );
    bera_testnet.insert(
        "BERA".to_string(),
        AssetInfo {
            token_address: "0x0000000000000000000000000000000000000000".to_string(),
            token_decimals: 12,
        },
    );
    bera_testnet.insert(
        "WBTC".to_string(),
        AssetInfo {
            token_address: "0xfbb8851A77Fe1491e17c3631377EB61F4cC8d54A".to_string(),
            token_decimals: 22,
        },
    );
    bera_testnet.insert(
        "USDC".to_string(),
        AssetInfo {
            token_address: "0xe452E10F80C715379CFE5c9C42af043e7F2e48A7".to_string(),
            token_decimals: 24,
        },
    );

    // Optimus Testnet
    optimus_testnet.insert(
        "WEVMOS".to_string(),
        AssetInfo {
            token_address: "0x6640954109d68da7b7F9815f97782BC9d9D397E9".to_string(),
            token_decimals: 12,
        },
    );
    optimus_testnet.insert(
        "WBTC".to_string(),
        AssetInfo {
            token_address: "0x5E6894554AFE84AeFB39f698584420adbf9035e3".to_string(),
            token_decimals: 22,
        },
    );
    optimus_testnet.insert(
        "WETH".to_string(),
        AssetInfo {
            token_address: "0x5659c1Fcc0764438072D005508EE0d33639665Ea".to_string(),
            token_decimals: 12,
        },
    );
    optimus_testnet.insert(
        "WSTETH".to_string(),
        AssetInfo {
            token_address: "0x3358d8f2Af6E8388F6248a27D89D12a09E7D060a".to_string(),
            token_decimals: 12,
        },
    );
    optimus_testnet.insert(
        "USDC".to_string(),
        AssetInfo {
            token_address: "0x592D41e64036f86131d268ccd03c91afFA5Fec9E".to_string(),
            token_decimals: 24,
        },
    );

    //INSERTING INTO NETWORK
    networks.insert("arbitrum_sepolia".to_string(), arbitrum_sepolia);
    networks.insert("bitlayer_testnet".to_string(), bitlayer_testnet);
    networks.insert("move_testnet".to_string(), move_testnet);
    networks.insert("optimus_testnet".to_string(), optimus_testnet);
    networks.insert("bera_testnet".to_string(), bera_testnet);

    networks
});

pub fn get_token_addresses() -> HashMap<String, String> {
    let mut TOKEN_ADDRESSES = HashMap::new();
    if let Some(TOKEN_INFO_CHAIN) = TOKEN_INFO.get(&ENV.NETWORK) {
        for (tokensymbol, assetinfo) in TOKEN_INFO_CHAIN.iter() {
            TOKEN_ADDRESSES.insert(tokensymbol.to_string(), assetinfo.token_address.clone());
        }
    }

    return TOKEN_ADDRESSES;
}

pub fn get_token_decimals() -> HashMap<String, u64> {
    let mut TOKEN_ADDRESSES = HashMap::new();
    if let Some(TOKEN_INFO_CHAIN) = TOKEN_INFO.get(&ENV.NETWORK) {
        for (tokensymbol, assetinfo) in TOKEN_INFO_CHAIN.iter() {
            TOKEN_ADDRESSES.insert(tokensymbol.to_string(), assetinfo.token_decimals.clone());
        }
    }
    return TOKEN_ADDRESSES;
}

#[derive(Clone)]
pub struct TokenKeyAddress {
    pub key: String,
    pub address: String,
}

pub static TOKENS_MAPPINGS: Lazy<HashMap<String, Vec<TokenKeyAddress>>> = Lazy::new(|| {
    let token_addresses: HashMap<String, String> = get_token_addresses();
    let mut network: HashMap<String, Vec<TokenKeyAddress>> = HashMap::new();
    let mut token_vec: Vec<TokenKeyAddress> = Vec::new();

    for (token_symbol, token_address) in token_addresses.iter() {
        let entry = TokenKeyAddress {
            key: token_symbol.clone(),
            address: token_address.clone(),
        };
        token_vec.push(entry);
    }
    let network_key = match ENV.NETWORK.as_str() {
        "arbitrum_sepolia" => "arbitrum_sepolia",
        "bitlayer_testnet" => "bitlayer_testnet",
        "move_testnet" => "move_testnet",
        _ => panic!("Network not found"),
    };

    network.insert(network_key.to_string(), token_vec);

    network
});

pub static SYMBOL_TO_ADDRESS_MAPPING: Lazy<HashMap<String, String>> =
    Lazy::new(|| get_token_addresses());

pub static SYMBOL_TO_DECIMAL_MAPPING: Lazy<HashMap<String, u64>> =
    Lazy::new(|| get_token_decimals());

pub static PYTH_ID_TO_TOKEN_MAPPING_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
            "BTC",
        );
        map.insert(
            "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace",
            "ETH",
        );
        map.insert(
            "6df640f3b8963d8f8358f791f352b8364513f6ab1cca5ed3f1f7b5448980e784",
            "WSTETH",
        );
        map.insert(
            "b7e3904c08ddd9c0c10c6d207d390fd19e87eb6aab96304f571ed94caebdefa0",
            "AXS",
        );
        map.insert(
            "0781209c28fda797616212b7f94d77af3a01f3e94a5d421760aef020cf2bcb51",
            "GALA",
        );
        map.insert(
            "9a4df90b25497f66b1afb012467e316e801ca3d839456db028892fe8c70c8016",
            "PENDLE",
        );
        map.insert(
            "06ade621dbc31ed0fc9255caaab984a468abe84164fb2ccc76f02a4636d97e31",
            "EOS",
        );
        map.insert(
            "097d687437374051c75160d648800f021086bc8edf469f11284491fda8192315",
            "BTT",
        );
        map.insert(
            "b7910ba7322db020416fcac28b48c01212fd9cc8fbcbaf7d30477ed8605f6bd4",
            "ENA",
        );
        map.insert(
            "6489800bb8974169adfe35937bf6736507097d13c190d760c557108c7e93a81b",
            "DYDX",
        );
        map.insert(
            "3871d0ef1cf9e26005e4bbf7822f67a8883071a9d8a4e7a0125d2484cca7671f",
            "BEAM",
        );
        map.insert(
            "c5f60d00d926ee369ded32a38a6bd5c1e0faa936f91b987a5d0dcf3c5d8afab0",
            "GNO",
        );
        map.insert(
            "193c739db502aadcef37c2589738b1e37bdb257d58cf1ab3c7ebc8e6df4e3ec0",
            "ORDI",
        );
        map.insert(
            "ee326a761a4b53629a29fc64bf47dda18cb2eea0bef22da7144dbdc130d112fc",
            "EGLD",
        );
        map.insert(
            "0affd4b8ad136a21d79bc82450a325ee12ff55a235abc242666e423b8bcffd03",
            "XTZ",
        );
        map.insert(
            "97cfe19da9153ef7d647b011c5e355142280ddb16004378573e6494e499879f3",
            "RON",
        );
        map.insert(
            "cb7a1d45139117f8d3da0a4b67264579aa905e3b124efede272634f094e1e9d1",
            "SAND",
        );
        map.insert(
            "051ee6cdd581106d0291dfd9b0ee13e6b4dde8fb251afd262c2ba5444257daa8",
            "GT",
        );
        map.insert(
            "b98ab6023650bd2edc026b983fb7c2f8fa1020286f1ba6ecf3f4322cd83b72a6",
            "ENS",
        );
        map.insert(
            "d6835ad1f773de4a378115eb6824bd0c0e42d84d1c84d9750e853fb6b6c7794a",
            "WLD",
        );
        map.insert(
            "4ea5bb4d2f5900cc2e97ba534240950740b4d3b89fe712a94a7304fd2fd92702",
            "AKT",
        );
        map.insert(
            "3bd860bea28bf982fa06bcf358118064bb114086cc03993bd76197eaab0b8018",
            "ZRO",
        );
        map.insert(
            "e799f456b358a2534aa1b45141d454ac04b444ed23b1440b778549bb758f2b5c",
            "CHZ",
        );
        map.insert(
            "39d020f60982ed892abbcd4a06a276a9f9b7bfbce003204c110b6e488f502da3",
            "SNX",
        );
        map.insert(
            "2f95862b045670cd22bee3114c39763a4a08beeb663b145d283c31d7d1101c4f",
            "BNB",
        );
        map.insert(
            "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
            "SOL",
        );
        map.insert(
            "ec5d399846a9209f3fe5881d70aae9268c94339ff9817e8d18ff19fa05eea1c8",
            "XRP",
        );
        map.insert(
            "8963217838ab4cf5cadc172203c1f0b763fbaa45f346d8ee50ba994bbcac3026",
            "TON",
        );
        map.insert(
            "2a01deaec9e51a579277b34b122399984d0bbf57e2458a7e42fecd2829867a0d",
            "ADA",
        );
        map.insert(
            "f0d57deca57b3da2fe63a493f4c25925fdfd8edf834b20f93e1f84dbd1504d4a",
            "SHIB",
        );
        map.insert(
            "93da3352f9f1d105fdfe4971cfa80e9dd777bfc5d0f683ebb6e1294b92137bb7",
            "AVAX",
        );
        map.insert(
            "ca3eed9b267293f6595901c734c7525ce8ef49adafe8284606ceb307afa2ca5b",
            "DOT",
        );
        map.insert(
            "8ac0c70fff57e9aefdf5edf44b51d62c2d433653cbb2cf5cc06bb115af04d221",
            "LINK",
        );
        map.insert(
            "3dd2b63686a450ec7290df3a1e0b583c0481f651351edfa7636f39aed55cf8a3",
            "BCH",
        );
        map.insert(
            "c415de8d2eba7db216527dff4b60e8f3a5311c740dadb233e13e12547e226750",
            "NEAR",
        );
        map.insert(
            "78d185a741d07edb3412b09008b7c5cfb9bbbd7d568bf00ba737b456ba171501",
            "UNI",
        );
        map.insert(
            "5de33a9112c2b700b8d30b8a3402c103578ccfa2765696471cc672bd5cf6ac52",
            "MATIC",
        );
        map.insert(
            "19e4e2b451406cf99311bb5127b12a948db17f30b69c323c8657d71119a58619",
            "LEO",
        );
        map.insert(
            "6e3f3fa8253588df9326580180233eb791e03b443a3ba7a1d892e73874e19a54",
            "LTC",
        );
        map.insert(
            "d69731a2e74ac1ce884fc3890f7ee324b6deb66147055249568869ed700882e4",
            "PEPE",
        );
        map.insert(
            "c9907d786c5821547777780a1e4f89484f3417cb14dd244f2b0a34ea7a554d67",
            "ICP",
        );
        map.insert(
            "dfd3cb51a9d39fde35a3ff6177b426def03ed48d45008248f22827d8bf50cab4",
            "KAS",
        );
        map.insert(
            "7f5cc8d963fc5b3d2ae41fe5685ada89fd4f14b435f8050f28c7fd409f40c2d8",
            "ETC",
        );
        map.insert(
            "ab7347771135fc733f8f38db462ba085ed3309955f42554a14fa13e855ac0e2f",
            "RNDR",
        );
        map.insert(
            "3728e591097635310e6341af53db8b7ee42da9b3a8d918f9463ce9cca886dfbd",
            "HBAR",
        );
        map.insert(
            "b00b60f88b03a6a625a8d1c048c3f66653edf217439983d037e7222c4e612819",
            "ATOM",
        );
        map.insert(
            "b7a8eba68a997cd0210c2e1e4ee811ad2d174b3611c22d9ebf16f4cb7e9ba850",
            "XLM",
        );
        map.insert(
            "d6f83dfeaff95d596ddec26af2ee32f391c206a183b161b7980821860eeef2f5",
            "OKB",
        );
        map.insert(
            "5c6c0d2386e3352356c3ab84434fafb5ea067ac2678a38a338c4a69ddc4bdb0c",
            "FTM",
        );
        map.insert(
            "4ca4beeca86f0d164160323817a4e42b10010a724c2217c6ee41b54cd4cc61fc",
            "WIF",
        );
        map.insert(
            "d40472610abe56d36d065a0cf889fc8f1dd9f3b7f2a478231a5fc6df07ea5ce3",
            "ONDO",
        );
        map.insert(
            "75ec6f04d4bded6afdc1440689be4402dd1e23d2ff2c21e081871eb2739ceb36",
            "NOT",
        );
        map.insert(
            "5fcf71143bb70d41af4fa9aa1287e2efd3c5911cee59f909f915c9f61baacb1e",
            "RUNE",
        );
        map.insert(
            "72b021217ca3fe68922a19aaf990109cb9d84e9ad004b4d2025ad6f529314419",
            "BONK",
        );
        map.insert(
            "b98e7ae8af2d298d2651eb21ab5b8b5738212e13efb43bd0dfbce7a74ba4b5d0",
            "FET",
        );
        map.insert(
            "2b9ab1e972a281585084148ba1389800799bd4be63b957507db1349314e47445",
            "AAVE",
        );
        map.insert(
            "09f7c1d7dfbb7df2b8fe3d3d87ee94a2259d212da4f30c1f0540d066dfa44723",
            "TIA",
        );
        map.insert(
            "9b4503710cc8c53f75c30e6e4fda1a7064680ef2e0ee97acd2e3a7c37b3c830c",
            "CORE",
        );
        map.insert(
            "0bbf28e9a841a1cc788f6a361b17ca072d0ea3098a1e5df1c3922d06719579ff",
            "PYTH",
        );
        map.insert(
            "53614f1cb0c031d4af66c04cb9c756234adad0e1cee85303795091499a4084eb",
            "SEI",
        );
        map.insert(
            "0a0408d619e9380abad35060f9192039ed5042fa6f82301d0e48bb52be830996",
            "JUP",
        );
        map.insert(
            "c8acad81438490d4ebcac23b3e93f31cdbcb893fcba746ea1c66b89684faae2f",
            "KCS",
        );
        map.insert(
            "2fb245b9a84554a0f15aa123cbb5f64cd263b59e9a87d80148cbffab50c69f30",
            "FLOW",
        );
        map.insert(
            "19ab139032007c8bd7d1fd3842ef392a5434569a72b555504a5aee47df2a0a35",
            "QNT",
        );
        map.insert(
            "67aed5a24fdad045475e7195c98a98aea119c763f272d4523f5bac93a4f33c2b",
            "TRX",
        );
        map.insert(
            "b44565b8b9b39ab2f4ba792f1c8f8aa8ef7d780e709b191637ef886d96fd1472",
            "BSV",
        );
        map.insert(
            "1dfffdcbc958d732750f53ff7f06d24bb01364b3f62abea511a390c74b8d16a5",
            "MANA",
        );
        map.insert(
            "44622616f246ce5fc46cf9ebdb879b0c0157275510744cea824ad206e48390b3",
            "XEC",
        );
        map.insert(
            "6a182399ff70ccf3e06024898942028204125a819e519a335ffa4579e66cd870",
            "STRK",
        );
        map.insert(
            "46b8cc9347f04391764a0361e0b17c3ba394b001e7c304f7650f6376e37c321d",
            "XMR",
        );
        map.insert(
            "3fa4252848f9f0a1480be62745a4629d9eb1322aebab8a791e344b3b9c1adcf5",
            "ARB",
        );
        map.insert(
            "4e3037c822d852d79af3ac80e35eb420ee3b870dca49f9344a38ef4773fb0585",
            "MNT",
        );
        map.insert(
            "150ac9b959aee0051e4091f0ef5216d941f590e1c5e7f91cf7635b5c11628c0e",
            "FIL",
        );
        map.insert(
            "23199c2bcb1303f667e733b9934db9eca5991e765b45f5ed18bc4b231415f2fe",
            "CRO",
        );
        map.insert(
            "ec7a775f46379b5e943c3526b1c8d54cd49749176b0b98e02dde68d1bd335c17",
            "STX",
        );
        map.insert(
            "941320a8989414874de5aa2fc340a75d5ed91fdff1613dd55f83844d52ea63a2",
            "IMX",
        );
        map.insert(
            "9375299e31c0deb9c6bc378e6329aab44cb48ec655552a70d4b9050346a30378",
            "MKR",
        );
        map.insert(
            "23d7315113f5b1d3ba7a83604c44b94d79f4fd69af77f804fc7f920a6dc65744",
            "SUI",
        );
        map.insert(
            "1722176f738aa1aafea170f8b27724042c5ac6d8cb9cf8ae02d692b0927e0681",
            "VET",
        );
        map.insert(
            "4d1f8dae0d96236fb98e8f47471a366ec3b1732b47041781934ca3a9bb2f35e7",
            "GRT",
        );
        map.insert(
            "7a5bc1d2b56ad029048cd63964b3ad2776eadf812edc1a43a31406cb54bff592",
            "INJ",
        );
        map.insert(
            "385f64d993f7b77d8182ed5003d97c60aa3361f3cecfe711544d2d59165e9bdf",
            "OP",
        );
        map.insert(
            "c63e2a7f37a04e5e614c07238bedb25dcc38927fba8fe890597a593c0b2fa4ad",
            "LDO",
        );
        map.insert(
            "410f41de235f2db824e562ea7ab2d3d3d4ff048316c61d629c0b93f58584e1af",
            "TAO",
        );
        map.insert(
            "f610eae82767039ffc95eef8feaeddb7bbac0673cfe7773b2fde24fd1adb0aee",
            "AR",
        );
        map.insert(
            "708bfcf418ead52a408407b039f2c33ce24ddc80d6dcb6d1cffef91c156c80fa",
            "BGB",
        );
        map.insert(
            "b0948a5e5313200c632b51bb5ca32f6de0d36e9950a942d19751e833f70dabfd",
            "DAI",
        );

        map
    });

pub static PYTH_ID_TO_TOKEN_MAPPING_MOVE_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert(
            "e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
            "BTC",
        );
        map.insert(
            "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace",
            "ETH",
        );
        map.insert(
            "eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a",
            "USDC",
        );
        map.insert(
            "9d4294bbcd1174d6f2003ec365831e64cc31d9f6f15a2b85399db8d5000960f6",
            "WETH",
        );
        map.insert(
            "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
            "SOL",
        );
        map.insert(
            "4279e31cc369bbcc2faf022b382b080e32a8e689ff20fbc530d2a603eb6cd98b",
            "HYPE",
        );
        map.insert(
            "6bf748c908767baa762a1563d454ebec2d5108f8ee36d806aadacc8f0a075b6d",
            "MOVE",
        );

        map
    });

pub static PYTH_ID_TO_TOKEN_MAPPING_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace",
            "ETH",
        );
        map.insert(
            "eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a",
            "USDC",
        );
        map.insert(
            "2f95862b045670cd22bee3114c39763a4a08beeb663b145d283c31d7d1101c4f",
            "BNB",
        );
        map.insert(
            "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
            "SOL",
        );
        map.insert(
            "ec5d399846a9209f3fe5881d70aae9268c94339ff9817e8d18ff19fa05eea1c8",
            "XRP",
        );
        map.insert(
            "8963217838ab4cf5cadc172203c1f0b763fbaa45f346d8ee50ba994bbcac3026",
            "TON",
        );
        map.insert(
            "2a01deaec9e51a579277b34b122399984d0bbf57e2458a7e42fecd2829867a0d",
            "ADA",
        );
        map.insert(
            "67aed5a24fdad045475e7195c98a98aea119c763f272d4523f5bac93a4f33c2b",
            "TRX",
        );
        map.insert(
            "f0d57deca57b3da2fe63a493f4c25925fdfd8edf834b20f93e1f84dbd1504d4a",
            "SHIB",
        );
        map.insert(
            "93da3352f9f1d105fdfe4971cfa80e9dd777bfc5d0f683ebb6e1294b92137bb7",
            "AVAX",
        );
        map.insert(
            "ca3eed9b267293f6595901c734c7525ce8ef49adafe8284606ceb307afa2ca5b",
            "DOT",
        );
        map.insert(
            "8ac0c70fff57e9aefdf5edf44b51d62c2d433653cbb2cf5cc06bb115af04d221",
            "LINK",
        );
        map.insert(
            "6df640f3b8963d8f8358f791f352b8364513f6ab1cca5ed3f1f7b5448980e784",
            "WSTETH",
        );
        map.insert(
            "e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
            "BTC",
        );
        map
    });

pub static PYTH_ID_TO_TOKEN_MAPPING: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &PYTH_ID_TO_TOKEN_MAPPING_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &PYTH_ID_TO_TOKEN_MAPPING_BITLAYER_TESTNET,
    );
    map.insert(
        "move_testnet".to_string(),
        &PYTH_ID_TO_TOKEN_MAPPING_MOVE_TESTNET,
    );
    map
});

pub static PYTH_ID_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let mut vec = Vec::new();
    vec.push("e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43");
    vec.push("ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace");
    vec.push("eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a");
    vec.push("6df640f3b8963d8f8358f791f352b8364513f6ab1cca5ed3f1f7b5448980e784");
    vec.push("b7e3904c08ddd9c0c10c6d207d390fd19e87eb6aab96304f571ed94caebdefa0");
    vec.push("0781209c28fda797616212b7f94d77af3a01f3e94a5d421760aef020cf2bcb51");
    vec.push("9a4df90b25497f66b1afb012467e316e801ca3d839456db028892fe8c70c8016");
    vec.push("06ade621dbc31ed0fc9255caaab984a468abe84164fb2ccc76f02a4636d97e31");
    vec.push("097d687437374051c75160d648800f021086bc8edf469f11284491fda8192315");
    vec.push("b7910ba7322db020416fcac28b48c01212fd9cc8fbcbaf7d30477ed8605f6bd4");
    vec.push("6489800bb8974169adfe35937bf6736507097d13c190d760c557108c7e93a81b");
    vec.push("3871d0ef1cf9e26005e4bbf7822f67a8883071a9d8a4e7a0125d2484cca7671f");
    vec.push("c5f60d00d926ee369ded32a38a6bd5c1e0faa936f91b987a5d0dcf3c5d8afab0");
    vec.push("193c739db502aadcef37c2589738b1e37bdb257d58cf1ab3c7ebc8e6df4e3ec0");
    vec.push("ee326a761a4b53629a29fc64bf47dda18cb2eea0bef22da7144dbdc130d112fc");
    vec.push("0affd4b8ad136a21d79bc82450a325ee12ff55a235abc242666e423b8bcffd03");
    vec.push("97cfe19da9153ef7d647b011c5e355142280ddb16004378573e6494e499879f3");
    vec.push("cb7a1d45139117f8d3da0a4b67264579aa905e3b124efede272634f094e1e9d1");
    vec.push("051ee6cdd581106d0291dfd9b0ee13e6b4dde8fb251afd262c2ba5444257daa8");
    vec.push("b98ab6023650bd2edc026b983fb7c2f8fa1020286f1ba6ecf3f4322cd83b72a6");
    vec.push("d6835ad1f773de4a378115eb6824bd0c0e42d84d1c84d9750e853fb6b6c7794a");
    vec.push("4ea5bb4d2f5900cc2e97ba534240950740b4d3b89fe712a94a7304fd2fd92702");
    vec.push("3bd860bea28bf982fa06bcf358118064bb114086cc03993bd76197eaab0b8018");
    vec.push("e799f456b358a2534aa1b45141d454ac04b444ed23b1440b778549bb758f2b5c");
    vec.push("39d020f60982ed892abbcd4a06a276a9f9b7bfbce003204c110b6e488f502da3");
    vec.push("2f95862b045670cd22bee3114c39763a4a08beeb663b145d283c31d7d1101c4f");
    vec.push("ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d");
    vec.push("ec5d399846a9209f3fe5881d70aae9268c94339ff9817e8d18ff19fa05eea1c8");
    vec.push("8963217838ab4cf5cadc172203c1f0b763fbaa45f346d8ee50ba994bbcac3026");
    vec.push("2a01deaec9e51a579277b34b122399984d0bbf57e2458a7e42fecd2829867a0d");
    vec.push("f0d57deca57b3da2fe63a493f4c25925fdfd8edf834b20f93e1f84dbd1504d4a");
    vec.push("93da3352f9f1d105fdfe4971cfa80e9dd777bfc5d0f683ebb6e1294b92137bb7");
    vec.push("ca3eed9b267293f6595901c734c7525ce8ef49adafe8284606ceb307afa2ca5b");
    vec.push("8ac0c70fff57e9aefdf5edf44b51d62c2d433653cbb2cf5cc06bb115af04d221");
    vec.push("3dd2b63686a450ec7290df3a1e0b583c0481f651351edfa7636f39aed55cf8a3");
    vec.push("c415de8d2eba7db216527dff4b60e8f3a5311c740dadb233e13e12547e226750");
    vec.push("78d185a741d07edb3412b09008b7c5cfb9bbbd7d568bf00ba737b456ba171501");
    vec.push("5de33a9112c2b700b8d30b8a3402c103578ccfa2765696471cc672bd5cf6ac52");
    vec.push("19e4e2b451406cf99311bb5127b12a948db17f30b69c323c8657d71119a58619");
    vec.push("6e3f3fa8253588df9326580180233eb791e03b443a3ba7a1d892e73874e19a54");
    vec.push("d69731a2e74ac1ce884fc3890f7ee324b6deb66147055249568869ed700882e4");
    vec.push("c9907d786c5821547777780a1e4f89484f3417cb14dd244f2b0a34ea7a554d67");
    vec.push("dfd3cb51a9d39fde35a3ff6177b426def03ed48d45008248f22827d8bf50cab4");
    vec.push("7f5cc8d963fc5b3d2ae41fe5685ada89fd4f14b435f8050f28c7fd409f40c2d8");
    vec.push("ab7347771135fc733f8f38db462ba085ed3309955f42554a14fa13e855ac0e2f");
    vec.push("3728e591097635310e6341af53db8b7ee42da9b3a8d918f9463ce9cca886dfbd");
    vec.push("b00b60f88b03a6a625a8d1c048c3f66653edf217439983d037e7222c4e612819");
    vec.push("b7a8eba68a997cd0210c2e1e4ee811ad2d174b3611c22d9ebf16f4cb7e9ba850");
    vec.push("d6f83dfeaff95d596ddec26af2ee32f391c206a183b161b7980821860eeef2f5");
    vec.push("5c6c0d2386e3352356c3ab84434fafb5ea067ac2678a38a338c4a69ddc4bdb0c");
    vec.push("4ca4beeca86f0d164160323817a4e42b10010a724c2217c6ee41b54cd4cc61fc");
    vec.push("d40472610abe56d36d065a0cf889fc8f1dd9f3b7f2a478231a5fc6df07ea5ce3");
    vec.push("75ec6f04d4bded6afdc1440689be4402dd1e23d2ff2c21e081871eb2739ceb36");
    vec.push("5fcf71143bb70d41af4fa9aa1287e2efd3c5911cee59f909f915c9f61baacb1e");
    vec.push("72b021217ca3fe68922a19aaf990109cb9d84e9ad004b4d2025ad6f529314419");
    vec.push("b98e7ae8af2d298d2651eb21ab5b8b5738212e13efb43bd0dfbce7a74ba4b5d0");
    vec.push("2b9ab1e972a281585084148ba1389800799bd4be63b957507db1349314e47445");
    vec.push("09f7c1d7dfbb7df2b8fe3d3d87ee94a2259d212da4f30c1f0540d066dfa44723");
    vec.push("9b4503710cc8c53f75c30e6e4fda1a7064680ef2e0ee97acd2e3a7c37b3c830c");
    vec.push("0bbf28e9a841a1cc788f6a361b17ca072d0ea3098a1e5df1c3922d06719579ff");
    vec.push("53614f1cb0c031d4af66c04cb9c756234adad0e1cee85303795091499a4084eb");
    vec.push("0a0408d619e9380abad35060f9192039ed5042fa6f82301d0e48bb52be830996");
    vec.push("c8acad81438490d4ebcac23b3e93f31cdbcb893fcba746ea1c66b89684faae2f");
    vec.push("2fb245b9a84554a0f15aa123cbb5f64cd263b59e9a87d80148cbffab50c69f30");
    vec.push("19ab139032007c8bd7d1fd3842ef392a5434569a72b555504a5aee47df2a0a35");
    vec.push("67aed5a24fdad045475e7195c98a98aea119c763f272d4523f5bac93a4f33c2b");
    vec.push("b44565b8b9b39ab2f4ba792f1c8f8aa8ef7d780e709b191637ef886d96fd1472");
    vec.push("1dfffdcbc958d732750f53ff7f06d24bb01364b3f62abea511a390c74b8d16a5");
    vec.push("44622616f246ce5fc46cf9ebdb879b0c0157275510744cea824ad206e48390b3");
    vec.push("6a182399ff70ccf3e06024898942028204125a819e519a335ffa4579e66cd870");
    vec.push("46b8cc9347f04391764a0361e0b17c3ba394b001e7c304f7650f6376e37c321d");
    vec.push("3fa4252848f9f0a1480be62745a4629d9eb1322aebab8a791e344b3b9c1adcf5");
    vec.push("4e3037c822d852d79af3ac80e35eb420ee3b870dca49f9344a38ef4773fb0585");
    vec.push("150ac9b959aee0051e4091f0ef5216d941f590e1c5e7f91cf7635b5c11628c0e");
    vec.push("23199c2bcb1303f667e733b9934db9eca5991e765b45f5ed18bc4b231415f2fe");
    vec.push("ec7a775f46379b5e943c3526b1c8d54cd49749176b0b98e02dde68d1bd335c17");
    vec.push("941320a8989414874de5aa2fc340a75d5ed91fdff1613dd55f83844d52ea63a2");
    vec.push("9375299e31c0deb9c6bc378e6329aab44cb48ec655552a70d4b9050346a30378");
    vec.push("23d7315113f5b1d3ba7a83604c44b94d79f4fd69af77f804fc7f920a6dc65744");
    vec.push("1722176f738aa1aafea170f8b27724042c5ac6d8cb9cf8ae02d692b0927e0681");
    vec.push("4d1f8dae0d96236fb98e8f47471a366ec3b1732b47041781934ca3a9bb2f35e7");
    vec.push("7a5bc1d2b56ad029048cd63964b3ad2776eadf812edc1a43a31406cb54bff592");
    vec.push("385f64d993f7b77d8182ed5003d97c60aa3361f3cecfe711544d2d59165e9bdf");
    vec.push("c63e2a7f37a04e5e614c07238bedb25dcc38927fba8fe890597a593c0b2fa4ad");
    vec.push("410f41de235f2db824e562ea7ab2d3d3d4ff048316c61d629c0b93f58584e1af");
    vec.push("f610eae82767039ffc95eef8feaeddb7bbac0673cfe7773b2fde24fd1adb0aee");
    vec.push("708bfcf418ead52a408407b039f2c33ce24ddc80d6dcb6d1cffef91c156c80fa");
    vec.push("b0948a5e5313200c632b51bb5ca32f6de0d36e9950a942d19751e833f70dabfd");
    vec
});

pub static PYTH_ID_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace",
        "eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a",
        "2f95862b045670cd22bee3114c39763a4a08beeb663b145d283c31d7d1101c4f",
        "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
        "ec5d399846a9209f3fe5881d70aae9268c94339ff9817e8d18ff19fa05eea1c8",
        "8963217838ab4cf5cadc172203c1f0b763fbaa45f346d8ee50ba994bbcac3026",
        "2a01deaec9e51a579277b34b122399984d0bbf57e2458a7e42fecd2829867a0d",
        "67aed5a24fdad045475e7195c98a98aea119c763f272d4523f5bac93a4f33c2b",
        "f0d57deca57b3da2fe63a493f4c25925fdfd8edf834b20f93e1f84dbd1504d4a",
        "93da3352f9f1d105fdfe4971cfa80e9dd777bfc5d0f683ebb6e1294b92137bb7",
        "ca3eed9b267293f6595901c734c7525ce8ef49adafe8284606ceb307afa2ca5b",
        "8ac0c70fff57e9aefdf5edf44b51d62c2d433653cbb2cf5cc06bb115af04d221",
        "6df640f3b8963d8f8358f791f352b8364513f6ab1cca5ed3f1f7b5448980e784",
        "e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
    ]
});

pub static PYTH_ID_MOVE_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
        "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace",
        "eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a",
        "c9d8b075a5c69303365ae23633d4e085199bf5c520a3b90fed1322a0342ffc33",
        "9d4294bbcd1174d6f2003ec365831e64cc31d9f6f15a2b85399db8d5000960f6",
        "6df640f3b8963d8f8358f791f352b8364513f6ab1cca5ed3f1f7b5448980e784",
        "0781209c28fda797616212b7f94d77af3a01f3e94a5d421760aef020cf2bcb51",
        "9a4df90b25497f66b1afb012467e316e801ca3d839456db028892fe8c70c8016",
        "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
        "4279e31cc369bbcc2faf022b382b080e32a8e689ff20fbc530d2a603eb6cd98b",
        "6bf748c908767baa762a1563d454ebec2d5108f8ee36d806aadacc8f0a075b6d",
        "bed3097008b9b5e3c93bec20be79cb43986b85a996475589351a21e67bae9b61",
    ]
});

pub static PYTH_ID: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("arbitrum_sepolia".to_string(), &PYTH_ID_ARBITRUM_SEPOLIA);
    map.insert("bitlayer_testnet".to_string(), &PYTH_ID_BITLAYER_TESTNET);
    map.insert("move_testnet".to_string(), &PYTH_ID_MOVE_TESTNET);
    map
});

pub static BINANCE_KEYS_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BTCUSDT",
        "ETHUSDT",
        "BNBUSDT",
        "NEOUSDT",
        "LTCUSDT",
        "ADAUSDT",
        "XRPUSDT",
        "EOSUSDT",
        "XLMUSDT",
        "TRXUSDT",
        "ETCUSDT",
        "VETUSDT",
        "USDCUSDT",
        "LINKUSDT",
        "BTTCUSDT",
        "FETUSDT",
        "XMRUSDT",
        "THETAUSDT",
        "MATICUSDT",
        "ATOMUSDT",
        "FTMUSDT",
        "ALGOUSDT",
        "DOGEUSDT",
        "CHZUSDT",
        "BEAMXUSDT",
        "XTZUSDT",
        "HBARUSDT",
        "STXUSDT",
        "BCHUSDT",
        "SOLUSDT",
        "SNXUSDT",
        "MKRUSDT",
        "MANAUSDT",
        "SANDUSDT",
        "DOTUSDT",
        "EGLDUSDT",
        "RUNEUSDT",
        "UNIUSDT",
        "AVAXUSDT",
        "AAVEUSDT",
        "NEARUSDT",
        "FILUSDT",
        "INJUSDT",
        "AXSUSDT",
        "GRTUSDT",
        "SHIBUSDT",
        "ICPUSDT",
        "ARUSDT",
        "DEXEUSDT",
        "QNTUSDT",
        "FLOWUSDT",
        "GNOUSDT",
        "XECUSDT",
        "DYDXUSDT",
        "GALAUSDT",
        "ENSUSDT",
        "JASMYUSDT",
        "RNDRUSDT",
        "IMXUSDT",
        "NEXOUSDT",
        "LDOUSDT",
        "OPUSDT",
        "APTUSDT",
        "AGIXUSDT",
        "ARBUSDT",
        "SUIUSDT",
        "PEPEUSDT",
        "FLOKIUSDT",
        "PENDLEUSDT",
        "WLDUSDT",
        "SEIUSDT",
        "TIAUSDT",
        "ORDIUSDT",
        "BONKUSDT",
        "JUPUSDT",
        "PYTHUSDT",
        "STRKUSDT",
        "WIFUSDT",
        "ENAUSDT",
        "TAOUSDT",
        "NOTUSDT",
        "ZROUSDT",
        "DAIUSDT",
    ]
});

pub static BINANCE_KEYS_MOVE_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BTCUSDT", "ETHUSDT", "SOLUSDT", "USDCUSDT", "MOVEUSDT", "PENGUSDT",
    ]
});

pub static BINANCE_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BNBUSDT", "ADAUSDT", "XRPUSDT", "TRXUSDT", "USDCUSDT", "LINKUSDT", "DOGEUSDT", "SOLUSDT",
        "DOTUSDT", "AVAXUSDT", "SHIBUSDT", "BTCUSDT", "ETHUSDT",
    ]
});

pub static BINANCE_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "arbitrum_sepolia".to_string(),
            &BINANCE_KEYS_ARBITRUM_SEPOLIA,
        );
        map.insert(
            "bitlayer_testnet".to_string(),
            &BINANCE_KEYS_BITLAYER_TESTNET,
        );
        map.insert("move_testnet".to_string(), &BINANCE_KEYS_MOVE_TESTNET);
        map
    });

pub static SYMBOL_MAP_BINANCE_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map.insert("BNBUSDT", "BNB");
        map.insert("NEOUSDT", "NEO");
        map.insert("LTCUSDT", "LTC");
        map.insert("ADAUSDT", "ADA");
        map.insert("XRPUSDT", "XRP");
        map.insert("EOSUSDT", "EOS");
        map.insert("XLMUSDT", "XLM");
        map.insert("TRXUSDT", "TRX");
        map.insert("ETCUSDT", "ETC");
        map.insert("VETUSDT", "VET");
        map.insert("USDCUSDT", "USDC");
        map.insert("LINKUSDT", "LINK");
        map.insert("BTTCUSDT", "BTT");
        map.insert("FETUSDT", "FET");
        map.insert("XMRUSDT", "XMR");
        map.insert("THETAUSDT", "THETA");
        map.insert("MATICUSDT", "MATIC");
        map.insert("ATOMUSDT", "ATOM");
        map.insert("FTMUSDT", "FTM");
        map.insert("ALGOUSDT", "ALGO");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("CHZUSDT", "CHZ");
        map.insert("BEAMXUSDT", "BEAM");
        map.insert("XTZUSDT", "XTZ");
        map.insert("HBARUSDT", "HBAR");
        map.insert("STXUSDT", "STX");
        map.insert("BCHUSDT", "BCH");
        map.insert("SOLUSDT", "SOL");
        map.insert("SNXUSDT", "SNX");
        map.insert("MKRUSDT", "MKR");
        map.insert("MANAUSDT", "MANA");
        map.insert("SANDUSDT", "SAND");
        map.insert("DOTUSDT", "DOT");
        map.insert("EGLDUSDT", "EGLD");
        map.insert("RUNEUSDT", "RUNE");
        map.insert("UNIUSDT", "UNI");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("AAVEUSDT", "AAVE");
        map.insert("NEARUSDT", "NEAR");
        map.insert("FILUSDT", "FIL");
        map.insert("INJUSDT", "INJ");
        map.insert("AXSUSDT", "AXS");
        map.insert("GRTUSDT", "GRT");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("ICPUSDT", "ICP");
        map.insert("ARUSDT", "AR");
        map.insert("DEXEUSDT", "DEXE");
        map.insert("QNTUSDT", "QNT");
        map.insert("FLOWUSDT", "FLOW");
        map.insert("GNOUSDT", "GNO");
        map.insert("XECUSDT", "XEC");
        map.insert("DYDXUSDT", "DYDX");
        map.insert("GALAUSDT", "GALA");
        map.insert("ENSUSDT", "ENS");
        map.insert("JASMYUSDT", "JASMY");
        map.insert("RNDRUSDT", "RNDR");
        map.insert("IMXUSDT", "IMX");
        map.insert("NEXOUSDT", "NEXO");
        map.insert("LDOUSDT", "LDO");
        map.insert("OPUSDT", "OP");
        map.insert("APTUSDT", "APT");
        map.insert("AGIXUSDT", "AGIX");
        map.insert("ARBUSDT", "ARB");
        map.insert("SUIUSDT", "SUI");
        map.insert("PEPEUSDT", "PEPE");
        map.insert("FLOKIUSDT", "FLOKI");
        map.insert("PENDLEUSDT", "PENDLE");
        map.insert("WLDUSDT", "WLD");
        map.insert("SEIUSDT", "SEI");
        map.insert("TIAUSDT", "TIA");
        map.insert("ORDIUSDT", "ORDI");
        map.insert("BONKUSDT", "BONK");
        map.insert("JUPUSDT", "JUP");
        map.insert("PYTHUSDT", "PYTH");
        map.insert("STRKUSDT", "STRK");
        map.insert("WIFUSDT", "WIF");
        map.insert("ENAUSDT", "ENA");
        map.insert("TAOUSDT", "TAO");
        map.insert("NOTUSDT", "NOT");
        map.insert("ZROUSDT", "ZRO");
        map.insert("DAIUSDT", "DAI");
        map
    });

pub static SYMBOL_MAP_BINANCE_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BNBUSDT", "BNB");
        map.insert("ADAUSDT", "ADA");
        map.insert("XRPUSDT", "XRP");
        map.insert("TRXUSDT", "TRX");
        map.insert("USDCUSDT", "USDC");
        map.insert("LINKUSDT", "LINK");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("SOLUSDT", "SOL");
        map.insert("DOTUSDT", "DOT");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map
    });

pub static SYMBOL_MAP_BINANCE_MOVEMENT_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map.insert("USDCUSDT", "USDC");
        map.insert("SOLUSDT", "SOL");
        map
    });

pub static BINANCE_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &SYMBOL_MAP_BINANCE_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_BINANCE_BITLAYER_TESTNET,
    );
    map.insert(
        "move_testnet".to_string(),
        &SYMBOL_MAP_BINANCE_MOVEMENT_TESTNET,
    );
    map
});

pub static KUCOIN_KEYS_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BONK-USDT",
        "XEC-USDT",
        "ETC-USDT",
        "RUNE-USDT",
        "BCH-USDT",
        "APT-USDT",
        "TRX-USDT",
        "SAND-USDT",
        "LDO-USDT",
        "TON-USDT",
        "AAVE-USDT",
        "MNT-USDT",
        "DOGE-USDT",
        "SEI-USDT",
        "ENS-USDT",
        "ARB-USDT",
        "NEO-USDT",
        "CHZ-USDT",
        "USDC-USDT",
        "DOT-USDT",
        "EOS-USDT",
        "STRK-USDT",
        "BTC-USDT",
        "TIA-USDT",
        "ADA-USDT",
        "PEPE-USDT",
        "BTT-USDT",
        "AGIX-USDT",
        "JASMY-USDT",
        "XMR-USDT",
        "FTM-USDT",
        "DYDX-USDT",
        "SNX-USDT",
        "INJ-USDT",
        "ATOM-USDT",
        "MANA-USDT",
        "ORDI-USDT",
        "ETH-USDT",
        "FLOW-USDT",
        "IMX-USDT",
        "WIF-USDT",
        "DEXE-USDT",
        "AKT-USDT",
        "AVAX-USDT",
        "STX-USDT",
        "KAS-USDT",
        "AXS-USDT",
        "CRO-USDT",
        "SOL-USDT",
        "FET-USDT",
        "MATIC-USDT",
        "ENA-USDT",
        "XRP-USDT",
        "BNB-USDT",
        "EGLD-USDT",
        "GRT-USDT",
        "NOT-USDT",
        "BRETT-USDT",
        "VET-USDT",
        "SHIB-USDT",
        "UNI-USDT",
        "THETA-USDT",
        "HBAR-USDT",
        "LTC-USDT",
        "LINK-USDT",
        "PENDLE-USDT",
        "XTZ-USDT",
        "WLD-USDT",
        "OP-USDT",
        "SUI-USDT",
        "ICP-USDT",
        "PYTH-USDT",
        "FIL-USDT",
        "ONDO-USDT",
        "JUP-USDT",
        "AR-USDT",
        "TAO-USDT",
        "XLM-USDT",
        "FLR-USDT",
        "KCS-USDT",
        "ALGO-USDT",
        "QNT-USDT",
        "NEAR-USDT",
        "MKR-USDT",
        "FLOKI-USDT",
        "RNDR-USDT",
        "ZRO-USDT",
        "USDT-DAI",
    ]
});

pub static KUCOIN_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "TRX-USDT",
        "TON-USDT",
        "DOGE-USDT",
        "USDC-USDT",
        "DOT-USDT",
        "ADA-USDT",
        "AVAX-USDT",
        "SOL-USDT",
        "XRP-USDT",
        "BNB-USDT",
        "SHIB-USDT",
        "LINK-USDT",
        "BTC-USDT",
        "ETH-USDT",
    ]
});

pub static KUCOIN_KEYS_MOVEMENT: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BONK-USDT",
        "XEC-USDT",
        "ETC-USDT",
        "RUNE-USDT",
        "BCH-USDT",
        "APT-USDT",
        "TRX-USDT",
        "SAND-USDT",
        "LDO-USDT",
        "TON-USDT",
        "AAVE-USDT",
        "MNT-USDT",
        "DOGE-USDT",
        "SEI-USDT",
        "ENS-USDT",
        "ARB-USDT",
        "NEO-USDT",
        "CHZ-USDT",
        "USDC-USDT",
        "DOT-USDT",
        "EOS-USDT",
        "STRK-USDT",
        "BTC-USDT",
        "TIA-USDT",
        "ADA-USDT",
        "PEPE-USDT",
        "BTT-USDT",
        "AGIX-USDT",
        "JASMY-USDT",
        "XMR-USDT",
        "FTM-USDT",
        "DYDX-USDT",
        "SNX-USDT",
        "INJ-USDT",
        "ATOM-USDT",
        "MANA-USDT",
        "ORDI-USDT",
        "ETH-USDT",
        "FLOW-USDT",
        "IMX-USDT",
        "WIF-USDT",
        "DEXE-USDT",
        "AKT-USDT",
        "AVAX-USDT",
        "STX-USDT",
        "KAS-USDT",
        "AXS-USDT",
        "CRO-USDT",
        "SOL-USDT",
        "FET-USDT",
        "MATIC-USDT",
        "ENA-USDT",
        "XRP-USDT",
        "BNB-USDT",
        "EGLD-USDT",
        "GRT-USDT",
        "NOT-USDT",
        "BRETT-USDT",
        "VET-USDT",
        "SHIB-USDT",
        "UNI-USDT",
        "THETA-USDT",
        "HBAR-USDT",
        "LTC-USDT",
        "LINK-USDT",
        "PENDLE-USDT",
        "XTZ-USDT",
        "WLD-USDT",
        "OP-USDT",
        "SUI-USDT",
        "ICP-USDT",
        "PYTH-USDT",
        "FIL-USDT",
        "ONDO-USDT",
        "JUP-USDT",
        "AR-USDT",
        "TAO-USDT",
        "XLM-USDT",
        "FLR-USDT",
        "KCS-USDT",
        "ALGO-USDT",
        "QNT-USDT",
        "NEAR-USDT",
        "MKR-USDT",
        "FLOKI-USDT",
        "RNDR-USDT",
        "ZRO-USDT",
        "USDT-DAI",
    ]
});

pub static KUCOIN_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &KUCOIN_KEYS_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &KUCOIN_KEYS_BITLAYER_TESTNET,
    );
    map.insert("move_testnet".to_string(), &KUCOIN_KEYS_MOVEMENT);
    map
});

// KUCOIN SYMBOL MAP

pub static SYMBOL_MAP_KUCOIN_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BONK-USDT", "BONK");
        map.insert("XEC-USDT", "XEC");
        map.insert("RUNE-USDT", "RUNE");
        map.insert("ETC-USDT", "ETC");
        map.insert("BCH-USDT", "BCH");
        map.insert("APT-USDT", "APT");
        map.insert("TRX-USDT", "TRX");
        map.insert("SAND-USDT", "SAND");
        map.insert("LDO-USDT", "LDO");
        map.insert("TON-USDT", "TON");
        map.insert("AAVE-USDT", "AAVE");
        map.insert("MNT-USDT", "MNT");
        map.insert("DOGE-USDT", "DOGE");
        map.insert("SEI-USDT", "SEI");
        map.insert("ENS-USDT", "ENS");
        map.insert("ARB-USDT", "ARB");
        map.insert("NEO-USDT", "NEO");
        map.insert("CHZ-USDT", "CHZ");
        map.insert("USDC-USDT", "USDC");
        map.insert("DOT-USDT", "DOT");
        map.insert("EOS-USDT", "EOS");
        map.insert("STRK-USDT", "STRK");
        map.insert("BTC-USDT", "BTC");
        map.insert("TIA-USDT", "TIA");
        map.insert("ADA-USDT", "ADA");
        map.insert("PEPE-USDT", "PEPE");
        map.insert("BTT-USDT", "BTT");
        map.insert("AGIX-USDT", "AGIX");
        map.insert("JASMY-USDT", "JASMY");
        map.insert("XMR-USDT", "XMR");
        map.insert("FTM-USDT", "FTM");
        map.insert("DYDX-USDT", "DYDX");
        map.insert("SNX-USDT", "SNX");
        map.insert("INJ-USDT", "INJ");
        map.insert("ATOM-USDT", "ATOM");
        map.insert("MANA-USDT", "MANA");
        map.insert("ORDI-USDT", "ORDI");
        map.insert("ETH-USDT", "ETH");
        map.insert("FLOW-USDT", "FLOW");
        map.insert("WIF-USDT", "WIF");
        map.insert("IMX-USDT", "IMX");
        map.insert("DEXE-USDT", "DEXE");
        map.insert("AKT-USDT", "AKT");
        map.insert("AVAX-USDT", "AVAX");
        map.insert("STX-USDT", "STX");
        map.insert("KAS-USDT", "KAS");
        map.insert("AXS-USDT", "AXS");
        map.insert("CRO-USDT", "CRO");
        map.insert("SOL-USDT", "SOL");
        map.insert("FET-USDT", "FET");
        map.insert("MATIC-USDT", "MATIC");
        map.insert("ENA-USDT", "ENA");
        map.insert("XRP-USDT", "XRP");
        map.insert("BNB-USDT", "BNB");
        map.insert("EGLD-USDT", "EGLD");
        map.insert("GRT-USDT", "GRT");
        map.insert("NOT-USDT", "NOT");
        map.insert("BRETT-USDT", "BRETT");
        map.insert("VET-USDT", "VET");
        map.insert("SHIB-USDT", "SHIB");
        map.insert("UNI-USDT", "UNI");
        map.insert("THETA-USDT", "THETA");
        map.insert("HBAR-USDT", "HBAR");
        map.insert("LTC-USDT", "LTC");
        map.insert("LINK-USDT", "LINK");
        map.insert("PENDLE-USDT", "PENDLE");
        map.insert("XTZ-USDT", "XTZ");
        map.insert("WLD-USDT", "WLD");
        map.insert("OP-USDT", "OP");
        map.insert("SUI-USDT", "SUI");
        map.insert("ICP-USDT", "ICP");
        map.insert("PYTH-USDT", "PYTH");
        map.insert("FIL-USDT", "FIL");
        map.insert("ONDO-USDT", "ONDO");
        map.insert("JUP-USDT", "JUP");
        map.insert("AR-USDT", "AR");
        map.insert("TAO-USDT", "TAO");
        map.insert("XLM-USDT", "XLM");
        map.insert("FLR-USDT", "FLR");
        map.insert("KCS-USDT", "KCS");
        map.insert("ALGO-USDT", "ALGO");
        map.insert("QNT-USDT", "QNT");
        map.insert("NEAR-USDT", "NEAR");
        map.insert("MKR-USDT", "MKR");
        map.insert("FLOKI-USDT", "FLOKI");
        map.insert("RNDR-USDT", "RNDR");
        map.insert("ZRO-USDT", "ZRO");
        map.insert("USDT-DAI", "DAI");
        map
    });

pub static SYMBOL_MAP_KUCOIN_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("TRX-USDT", "TRX");
        map.insert("TON-USDT", "TON");
        map.insert("DOGE-USDT", "DOGE");
        map.insert("USDC-USDT", "USDC");
        map.insert("DOT-USDT", "DOT");
        map.insert("ADA-USDT", "ADA");
        map.insert("AVAX-USDT", "AVAX");
        map.insert("SOL-USDT", "SOL");
        map.insert("XRP-USDT", "XRP");
        map.insert("BNB-USDT", "BNB");
        map.insert("SHIB-USDT", "SHIB");
        map.insert("LINK-USDT", "LINK");
        map.insert("BTC-USDT", "BTC");
        map.insert("ETH-USDT", "ETH");
        map
    });

pub static SYMBOL_MAP_KUCOIN_MOVEMENT: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BONK-USDT", "BONK");
        map.insert("XEC-USDT", "XEC");
        map.insert("RUNE-USDT", "RUNE");
        map.insert("ETC-USDT", "ETC");
        map.insert("BCH-USDT", "BCH");
        map.insert("APT-USDT", "APT");
        map.insert("TRX-USDT", "TRX");
        map.insert("SAND-USDT", "SAND");
        map.insert("LDO-USDT", "LDO");
        map.insert("TON-USDT", "TON");
        map.insert("AAVE-USDT", "AAVE");
        map.insert("MNT-USDT", "MNT");
        map.insert("DOGE-USDT", "DOGE");
        map.insert("SEI-USDT", "SEI");
        map.insert("ENS-USDT", "ENS");
        map.insert("ARB-USDT", "ARB");
        map.insert("NEO-USDT", "NEO");
        map.insert("CHZ-USDT", "CHZ");
        map.insert("USDC-USDT", "USDC");
        map.insert("DOT-USDT", "DOT");
        map.insert("EOS-USDT", "EOS");
        map.insert("STRK-USDT", "STRK");
        map.insert("BTC-USDT", "BTC");
        map.insert("TIA-USDT", "TIA");
        map.insert("ADA-USDT", "ADA");
        map.insert("PEPE-USDT", "PEPE");
        map.insert("BTT-USDT", "BTT");
        map.insert("AGIX-USDT", "AGIX");
        map.insert("JASMY-USDT", "JASMY");
        map.insert("XMR-USDT", "XMR");
        map.insert("FTM-USDT", "FTM");
        map.insert("DYDX-USDT", "DYDX");
        map.insert("SNX-USDT", "SNX");
        map.insert("INJ-USDT", "INJ");
        map.insert("ATOM-USDT", "ATOM");
        map.insert("MANA-USDT", "MANA");
        map.insert("ORDI-USDT", "ORDI");
        map.insert("ETH-USDT", "ETH");
        map.insert("FLOW-USDT", "FLOW");
        map.insert("WIF-USDT", "WIF");
        map.insert("IMX-USDT", "IMX");
        map.insert("DEXE-USDT", "DEXE");
        map.insert("AKT-USDT", "AKT");
        map.insert("AVAX-USDT", "AVAX");
        map.insert("STX-USDT", "STX");
        map.insert("KAS-USDT", "KAS");
        map.insert("AXS-USDT", "AXS");
        map.insert("CRO-USDT", "CRO");
        map.insert("SOL-USDT", "SOL");
        map.insert("FET-USDT", "FET");
        map.insert("MATIC-USDT", "MATIC");
        map.insert("ENA-USDT", "ENA");
        map.insert("XRP-USDT", "XRP");
        map.insert("BNB-USDT", "BNB");
        map.insert("EGLD-USDT", "EGLD");
        map.insert("GRT-USDT", "GRT");
        map.insert("NOT-USDT", "NOT");
        map.insert("BRETT-USDT", "BRETT");
        map.insert("VET-USDT", "VET");
        map.insert("SHIB-USDT", "SHIB");
        map.insert("UNI-USDT", "UNI");
        map.insert("THETA-USDT", "THETA");
        map.insert("HBAR-USDT", "HBAR");
        map.insert("LTC-USDT", "LTC");
        map.insert("LINK-USDT", "LINK");
        map.insert("PENDLE-USDT", "PENDLE");
        map.insert("XTZ-USDT", "XTZ");
        map.insert("WLD-USDT", "WLD");
        map.insert("OP-USDT", "OP");
        map.insert("SUI-USDT", "SUI");
        map.insert("ICP-USDT", "ICP");
        map.insert("PYTH-USDT", "PYTH");
        map.insert("FIL-USDT", "FIL");
        map.insert("ONDO-USDT", "ONDO");
        map.insert("JUP-USDT", "JUP");
        map.insert("AR-USDT", "AR");
        map.insert("TAO-USDT", "TAO");
        map.insert("XLM-USDT", "XLM");
        map.insert("FLR-USDT", "FLR");
        map.insert("KCS-USDT", "KCS");
        map.insert("ALGO-USDT", "ALGO");
        map.insert("QNT-USDT", "QNT");
        map.insert("NEAR-USDT", "NEAR");
        map.insert("MKR-USDT", "MKR");
        map.insert("FLOKI-USDT", "FLOKI");
        map.insert("RNDR-USDT", "RNDR");
        map.insert("ZRO-USDT", "ZRO");
        map.insert("USDT-DAI", "DAI");
        map
    });

pub static KUCOIN_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &SYMBOL_MAP_KUCOIN_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_KUCOIN_BITLAYER_TESTNET,
    );
    map.insert("move_testnet".to_string(), &SYMBOL_MAP_KUCOIN_MOVEMENT);
    map
});

// MEXC KEYS

pub static MEXC_KEYS_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "PYTHUSDT",
        "HBARUSDT",
        "ARUSDT",
        "XMRUSDT",
        "SOLUSDT",
        "INJUSDT",
        "ONDOUSDT",
        "PEPEUSDT",
        "JUPUSDT",
        "AVAXUSDT",
        "SUIUSDT",
        "NEXOUSDT",
        "ADAUSDT",
        "TAOUSDT",
        "STXUSDT",
        "EOSUSDT",
        "SEIUSDT",
        "DEXEUSDT",
        "LINKUSDT",
        "MANAUSDT",
        "ALGOUSDT",
        "LEOUSDT",
        "CROUSDT",
        "TRXUSDT",
        "CHZUSDT",
        "ICPUSDT",
        "XECUSDT",
        "DOGEUSDT",
        "STRKUSDT",
        "BCHUSDT",
        "XRPUSDT",
        "SANDUSDT",
        "DOTUSDT",
        "ENAUSDT",
        "APTUSDT",
        "COREUSDT",
        "MNTUSDT",
        "AGIXUSDT",
        "XTZUSDT",
        "DYDXUSDT",
        "RUNEUSDT",
        "ORDIUSDT",
        "NOTUSDT",
        "ARBUSDT",
        "BNBUSDT",
        "ZROUSDT",
        "FLRUSDT",
        "MKRUSDT",
        "ENSUSDT",
        "ATOMUSDT",
        "TONUSDT",
        "VETUSDT",
        "ETCUSDT",
        "GALAUSDT",
        "ETHUSDT",
        "WLDUSDT",
        "RNDRUSDT",
        "XLMUSDT",
        "AAVEUSDT",
        "OPUSDT",
        "KASUSDT",
        "JASMYUSDT",
        "NEARUSDT",
        "GRTUSDT",
        "RONUSDT",
        "BEAMXUSDT",
        "BONKUSDT",
        "FLOWUSDT",
        "BSVUSDT",
        "SNXUSDT",
        "USDCUSDT",
        "GNOUSDT",
        "SHIBUSDT",
        "OKBUSDT",
        "QNTUSDT",
        "AXSUSDT",
        "PENDLEUSDT",
        "FETUSDT",
        "WIFUSDT",
        "IMXUSDT",
        "KCSUSDT",
        "FTMUSDT",
        "FILUSDT",
        "UNIUSDT",
        "BGBUSDT",
        "BTCUSDT",
        "TIAUSDT",
        "LDOUSDT",
        "FLOKIUSDT",
        "NEOUSDT",
        "EGLDUSDT",
        "BTTUSDT",
        "LTCUSDT",
        "MATICUSDT",
        "DAIUSDT",
    ]
});

pub static MEXC_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "SOLUSDT", "AVAXUSDT", "ADAUSDT", "LINKUSDT", "TRXUSDT", "DOGEUSDT", "XRPUSDT", "DOTUSDT",
        "BNBUSDT", "TONUSDT", "USDCUSDT", "SHIBUSDT", "BTCUSDT", "ETHUSDT",
    ]
});

pub static MEXC_KEYS_MOVEMENT_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "SOLUSDT", "AVAXUSDT", "ADAUSDT", "LINKUSDT", "TRXUSDT", "DOGEUSDT", "XRPUSDT", "DOTUSDT",
        "BNBUSDT", "TONUSDT", "USDCUSDT", "SHIBUSDT", "BTCUSDT", "ETHUSDT",
    ]
});

pub static MEXC_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("arbitrum_sepolia".to_string(), &MEXC_KEYS_ARBITRUM_SEPOLIA);
    map.insert("bitlayer_testnet".to_string(), &MEXC_KEYS_BITLAYER_TESTNET);
    map.insert("move_testnet".to_string(), &MEXC_KEYS_MOVEMENT_TESTNET);
    map
});

// SYMBOL MAP MEXC

pub static SYMBOL_MAP_MEXC_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("PYTHUSDT", "PYTH");
        map.insert("HBARUSDT", "HBAR");
        map.insert("ARUSDT", "AR");
        map.insert("XMRUSDT", "XMR");
        map.insert("SOLUSDT", "SOL");
        map.insert("INJUSDT", "INJ");
        map.insert("ONDOUSDT", "ONDO");
        map.insert("PEPEUSDT", "PEPE");
        map.insert("JUPUSDT", "JUP");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("SUIUSDT", "SUI");
        map.insert("NEXOUSDT", "NEXO");
        map.insert("ADAUSDT", "ADA");
        map.insert("TAOUSDT", "TAO");
        map.insert("STXUSDT", "STX");
        map.insert("EOSUSDT", "EOS");
        map.insert("SEIUSDT", "SEI");
        map.insert("DEXEUSDT", "DEXE");
        map.insert("LINKUSDT", "LINK");
        map.insert("MANAUSDT", "MANA");
        map.insert("ALGOUSDT", "ALGO");
        map.insert("LEOUSDT", "LEO");
        map.insert("CROUSDT", "CRO");
        map.insert("TRXUSDT", "TRX");
        map.insert("CHZUSDT", "CHZ");
        map.insert("ICPUSDT", "ICP");
        map.insert("XECUSDT", "XEC");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("STRKUSDT", "STRK");
        map.insert("BCHUSDT", "BCH");
        map.insert("XRPUSDT", "XRP");
        map.insert("SANDUSDT", "SAND");
        map.insert("DOTUSDT", "DOT");
        map.insert("ENAUSDT", "ENA");
        map.insert("APTUSDT", "APT");
        map.insert("COREUSDT", "CORE");
        map.insert("MNTUSDT", "MNT");
        map.insert("AGIXUSDT", "AGIX");
        map.insert("XTZUSDT", "XTZ");
        map.insert("DYDXUSDT", "DYDX");
        map.insert("RUNEUSDT", "RUNE");
        map.insert("ORDIUSDT", "ORDI");
        map.insert("NOTUSDT", "NOT");
        map.insert("ARBUSDT", "ARB");
        map.insert("BNBUSDT", "BNB");
        map.insert("ZROUSDT", "ZRO");
        map.insert("FLRUSDT", "FLR");
        map.insert("MKRUSDT", "MKR");
        map.insert("ENSUSDT", "ENS");
        map.insert("ATOMUSDT", "ATOM");
        map.insert("TONUSDT", "TON");
        map.insert("VETUSDT", "VET");
        map.insert("ETCUSDT", "ETC");
        map.insert("GALAUSDT", "GALA");
        map.insert("ETHUSDT", "ETH");
        map.insert("WLDUSDT", "WLD");
        map.insert("RNDRUSDT", "RNDR");
        map.insert("XLMUSDT", "XLM");
        map.insert("AAVEUSDT", "AAVE");
        map.insert("OPUSDT", "OP");
        map.insert("KASUSDT", "KAS");
        map.insert("JASMYUSDT", "JASMY");
        map.insert("NEARUSDT", "NEAR");
        map.insert("GRTUSDT", "GRT");
        map.insert("RONUSDT", "RON");
        map.insert("BEAMXUSDT", "BEAM");
        map.insert("BONKUSDT", "BONK");
        map.insert("FLOWUSDT", "FLOW");
        map.insert("BSVUSDT", "BSV");
        map.insert("SNXUSDT", "SNX");
        map.insert("USDCUSDT", "USDC");
        map.insert("GNOUSDT", "GNO");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("OKBUSDT", "OKB");
        map.insert("QNTUSDT", "QNT");
        map.insert("AXSUSDT", "AXS");
        map.insert("PENDLEUSDT", "PENDLE");
        map.insert("FETUSDT", "FET");
        map.insert("WIFUSDT", "WIF");
        map.insert("IMXUSDT", "IMX");
        map.insert("KCSUSDT", "KCS");
        map.insert("FTMUSDT", "FTM");
        map.insert("FILUSDT", "FIL");
        map.insert("UNIUSDT", "UNI");
        map.insert("BGBUSDT", "BGB");
        map.insert("BTCUSDT", "BTC");
        map.insert("TIAUSDT", "TIA");
        map.insert("LDOUSDT", "LDO");
        map.insert("FLOKIUSDT", "FLOKI");
        map.insert("NEOUSDT", "NEO");
        map.insert("EGLDUSDT", "EGLD");
        map.insert("BTTUSDT", "BTT");
        map.insert("LTCUSDT", "LTC");
        map.insert("MATICUSDT", "MATIC");
        map.insert("DAIUSDT", "DAI");
        map
    });

pub static SYMBOL_MAP_MEXC_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("SOLUSDT", "SOL");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("ADAUSDT", "ADA");
        map.insert("LINKUSDT", "LINK");
        map.insert("TRXUSDT", "TRX");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("XRPUSDT", "XRP");
        map.insert("DOTUSDT", "DOT");
        map.insert("BNBUSDT", "BNB");
        map.insert("TONUSDT", "TON");
        map.insert("USDCUSDT", "USDC");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map
    });

pub static SYMBOL_MAP_MOVEMENT_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("SOLUSDT", "SOL");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("ADAUSDT", "ADA");
        map.insert("LINKUSDT", "LINK");
        map.insert("TRXUSDT", "TRX");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("XRPUSDT", "XRP");
        map.insert("DOTUSDT", "DOT");
        map.insert("BNBUSDT", "BNB");
        map.insert("TONUSDT", "TON");
        map.insert("USDCUSDT", "USDC");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map
    });

pub static MEXC_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &SYMBOL_MAP_MEXC_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_MEXC_BITLAYER_TESTNET,
    );
    map.insert("move_testnet".to_string(), &SYMBOL_MAP_MOVEMENT_TESTNET);
    map
});

pub static GATE_KEYS_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ORDI_USDT",
        "FLOKI_USDT",
        "BSV_USDT",
        "WLD_USDT",
        "DOT_USDT",
        "ETH_USDT",
        "XEC_USDT",
        "ICP_USDT",
        "UNI_USDT",
        "BTC_USDT",
        "DEXE_USDT",
        "EOS_USDT",
        "STX_USDT",
        "THETA_USDT",
        "SEI_USDT",
        "TRX_USDT",
        "XMR_USDT",
        "XRP_USDT",
        "NEO_USDT",
        "OKB_USDT",
        "MKR_USDT",
        "DYDX_USDT",
        "AR_USDT",
        "GNO_USDT",
        "TAO_USDT",
        "PEPE_USDT",
        "GT_USDT",
        "BRETT_USDT",
        "FLOW_USDT",
        "AGIX_USDT",
        "QNT_USDT",
        "FET_USDT",
        "NEXO_USDT",
        "ENA_USDT",
        "JASMY_USDT",
        "ATOM_USDT",
        "PYTH_USDT",
        "ARB_USDT",
        "ALGO_USDT",
        "VET_USDT",
        "NOT_USDT",
        "LEO_USDT",
        "ENS_USDT",
        "ZRO_USDT",
        "AVAX_USDT",
        "CHZ_USDT",
        "KAS_USDT",
        "BNB_USDT",
        "NEAR_USDT",
        "JUP_USDT",
        "EGLD_USDT",
        "MATIC_USDT",
        "LTC_USDT",
        "ADA_USDT",
        "SOL_USDT",
        "RUNE_USDT",
        "OP_USDT",
        "HBAR_USDT",
        "XLM_USDT",
        "SNX_USDT",
        "AAVE_USDT",
        "CRO_USDT",
        "INJ_USDT",
        "SAND_USDT",
        "FLR_USDT",
        "MNT_USDT",
        "AKT_USDT",
        "ETC_USDT",
        "MANA_USDT",
        "GALA_USDT",
        "DOGE_USDT",
        "RNDR_USDT",
        "PENDLE_USDT",
        "FTM_USDT",
        "ONDO_USDT",
        "APT_USDT",
        "XTZ_USDT",
        "RON_USDT",
        "BEAMX_USDT",
        "SHIB_USDT",
        "BTT_USDT",
        "WIF_USDT",
        "BCH_USDT",
        "GRT_USDT",
        "USDC_USDT",
        "SUI_USDT",
        "TON_USDT",
        "FIL_USDT",
        "IMX_USDT",
        "STRK_USDT",
        "AXS_USDT",
        "LDO_USDT",
        "TIA_USDT",
        "CORE_USDT",
        "LINK_USDT",
        "BONK_USDT",
        "DAI_USDT",
    ]
});

pub static GATE_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "SOL_USDT",
        "TRX_USDT",
        "SHIB_USDT",
        "DOT_USDT",
        "ADA_USDT",
        "TON_USDT",
        "XRP_USDT",
        "BNB_USDT",
        "DOGE_USDT",
        "LINK_USDT",
        "USDC_USDT",
        "AVAX_USDT",
        "BTC_USDT",
        "ETH_USDT",
    ]
});

pub static GATE_KEYS_MOVEMENT_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "SOL_USDT",
        "TRX_USDT",
        "SHIB_USDT",
        "DOT_USDT",
        "ADA_USDT",
        "TON_USDT",
        "XRP_USDT",
        "BNB_USDT",
        "DOGE_USDT",
        "LINK_USDT",
        "USDC_USDT",
        "AVAX_USDT",
        "BTC_USDT",
        "ETH_USDT",
    ]
});

pub static GATE_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("arbitrum_sepolia".to_string(), &GATE_KEYS_ARBITRUM_SEPOLIA);
    map.insert("bitlayer_testnet".to_string(), &GATE_KEYS_BITLAYER_TESTNET);
    map.insert("move_testnet".to_string(), &GATE_KEYS_MOVEMENT_TESTNET);
    map
});

// GATE SYMBOL MAP

pub static SYMBOL_MAP_GATE_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("ORDI_USDT", "ORDI");
        map.insert("FLOKI_USDT", "FLOKI");
        map.insert("BSV_USDT", "BSV");
        map.insert("WLD_USDT", "WLD");
        map.insert("DOT_USDT", "DOT");
        map.insert("ETH_USDT", "ETH");
        map.insert("XEC_USDT", "XEC");
        map.insert("ICP_USDT", "ICP");
        map.insert("UNI_USDT", "UNI");
        map.insert("BTC_USDT", "BTC");
        map.insert("DEXE_USDT", "DEXE");
        map.insert("EOS_USDT", "EOS");
        map.insert("STX_USDT", "STX");
        map.insert("THETA_USDT", "THETA");
        map.insert("SEI_USDT", "SEI");
        map.insert("TRX_USDT", "TRX");
        map.insert("XMR_USDT", "XMR");
        map.insert("XRP_USDT", "XRP");
        map.insert("NEO_USDT", "NEO");
        map.insert("OKB_USDT", "OKB");
        map.insert("MKR_USDT", "MKR");
        map.insert("DYDX_USDT", "DYDX");
        map.insert("AR_USDT", "AR");
        map.insert("GNO_USDT", "GNO");
        map.insert("TAO_USDT", "TAO");
        map.insert("PEPE_USDT", "PEPE");
        map.insert("GT_USDT", "GT");
        map.insert("BRETT_USDT", "BRETT");
        map.insert("FLOW_USDT", "FLOW");
        map.insert("AGIX_USDT", "AGIX");
        map.insert("QNT_USDT", "QNT");
        map.insert("FET_USDT", "FET");
        map.insert("NEXO_USDT", "NEXO");
        map.insert("ENA_USDT", "ENA");
        map.insert("JASMY_USDT", "JASMY");
        map.insert("ATOM_USDT", "ATOM");
        map.insert("PYTH_USDT", "PYTH");
        map.insert("ARB_USDT", "ARB");
        map.insert("ALGO_USDT", "ALGO");
        map.insert("VET_USDT", "VET");
        map.insert("NOT_USDT", "NOT");
        map.insert("LEO_USDT", "LEO");
        map.insert("ENS_USDT", "ENS");
        map.insert("ZRO_USDT", "ZRO");
        map.insert("AVAX_USDT", "AVAX");
        map.insert("CHZ_USDT", "CHZ");
        map.insert("KAS_USDT", "KAS");
        map.insert("BNB_USDT", "BNB");
        map.insert("NEAR_USDT", "NEAR");
        map.insert("JUP_USDT", "JUP");
        map.insert("EGLD_USDT", "EGLD");
        map.insert("MATIC_USDT", "MATIC");
        map.insert("LTC_USDT", "LTC");
        map.insert("ADA_USDT", "ADA");
        map.insert("SOL_USDT", "SOL");
        map.insert("RUNE_USDT", "RUNE");
        map.insert("OP_USDT", "OP");
        map.insert("HBAR_USDT", "HBAR");
        map.insert("XLM_USDT", "XLM");
        map.insert("SNX_USDT", "SNX");
        map.insert("AAVE_USDT", "AAVE");
        map.insert("CRO_USDT", "CRO");
        map.insert("INJ_USDT", "INJ");
        map.insert("SAND_USDT", "SAND");
        map.insert("FLR_USDT", "FLR");
        map.insert("MNT_USDT", "MNT");
        map.insert("AKT_USDT", "AKT");
        map.insert("ETC_USDT", "ETC");
        map.insert("MANA_USDT", "MANA");
        map.insert("GALA_USDT", "GALA");
        map.insert("DOGE_USDT", "DOGE");
        map.insert("RNDR_USDT", "RNDR");
        map.insert("PENDLE_USDT", "PENDLE");
        map.insert("FTM_USDT", "FTM");
        map.insert("ONDO_USDT", "ONDO");
        map.insert("APT_USDT", "APT");
        map.insert("XTZ_USDT", "XTZ");
        map.insert("RON_USDT", "RON");
        map.insert("BEAMX_USDT", "BEAM");
        map.insert("SHIB_USDT", "SHIB");
        map.insert("BTT_USDT", "BTT");
        map.insert("WIF_USDT", "WIF");
        map.insert("BCH_USDT", "BCH");
        map.insert("GRT_USDT", "GRT");
        map.insert("USDC_USDT", "USDC");
        map.insert("SUI_USDT", "SUI");
        map.insert("TON_USDT", "TON");
        map.insert("FIL_USDT", "FIL");
        map.insert("IMX_USDT", "IMX");
        map.insert("STRK_USDT", "STRK");
        map.insert("AXS_USDT", "AXS");
        map.insert("LDO_USDT", "LDO");
        map.insert("TIA_USDT", "TIA");
        map.insert("CORE_USDT", "CORE");
        map.insert("LINK_USDT", "LINK");
        map.insert("BONK_USDT", "BONK");
        map.insert("DAI_USDT", "DAI");

        map
    });

pub static SYMBOL_MAP_GATE_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert("SOL_USDT", "SOL");
        map.insert("TRX_USDT", "TRX");
        map.insert("SHIB_USDT", "SHIB");
        map.insert("DOT_USDT", "DOT");
        map.insert("ADA_USDT", "ADA");
        map.insert("TON_USDT", "TON");
        map.insert("XRP_USDT", "XRP");
        map.insert("BNB_USDT", "BNB");
        map.insert("DOGE_USDT", "DOGE");
        map.insert("LINK_USDT", "LINK");
        map.insert("USDC_USDT", "USDC");
        map.insert("AVAX_USDT", "AVAX");
        map.insert("BTC_USDT", "BTC");
        map.insert("ETH_USDT", "ETH");

        map
    });

pub static SYMBOL_MAP_GATE_MOVEMENT_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("SOL_USDT", "SOL");
        map.insert("TRX_USDT", "TRX");
        map.insert("SHIB_USDT", "SHIB");
        map.insert("DOT_USDT", "DOT");
        map.insert("ADA_USDT", "ADA");
        map.insert("TON_USDT", "TON");
        map.insert("XRP_USDT", "XRP");
        map.insert("BNB_USDT", "BNB");
        map.insert("DOGE_USDT", "DOGE");
        map.insert("LINK_USDT", "LINK");
        map.insert("USDC_USDT", "USDC");
        map.insert("AVAX_USDT", "AVAX");
        map.insert("BTC_USDT", "BTC");
        map.insert("ETH_USDT", "ETH");
        map
    });

pub static GATE_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &SYMBOL_MAP_GATE_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_GATE_BITLAYER_TESTNET,
    );
    map.insert(
        "move_testnet".to_string(),
        &SYMBOL_MAP_GATE_MOVEMENT_TESTNET,
    );
    map
});

pub static BYBIT_KEYS_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BTCUSDT",
        "ETHUSDT",
        "USDCUSDT",
        "AXSUSDT",
        "GALAUSDT",
        "PENDLEUSDT",
        "EOSUSDT",
        "BTTUSDT",
        "ENAUSDT",
        "DYDXUSDT",
        "BEAMUSDT",
        "GNOUSDT",
        "NEOUSDT",
        "ORDIUSDT",
        "AGIXUSDT",
        "EGLDUSDT",
        "XTZUSDT",
        "RONUSDT",
        "SANDUSDT",
        "GTUSDT",
        "ENSUSDT",
        "WLDUSDT",
        "AKTUSDT",
        "NEXOUSDT",
        "ZROUSDT",
        "CHZUSDT",
        "SNXUSDT",
        "DEXEUSDT",
        "BNBUSDT",
        "SOLUSDT",
        "XRPUSDT",
        "TONUSDT",
        "DOGEUSDT",
        "ADAUSDT",
        "SHIBUSDT",
        "AVAXUSDT",
        "DOTUSDT",
        "LINKUSDT",
        "BCHUSDT",
        "NEARUSDT",
        "UNIUSDT",
        "MATICUSDT",
        "LEOUSDT",
        "LTCUSDT",
        "PEPEUSDT",
        "ICPUSDT",
        "KASUSDT",
        "ETCUSDT",
        "APTUSDT",
        "RNDRUSDT",
        "HBARUSDT",
        "ATOMUSDT",
        "XLMUSDT",
        "OKBUSDT",
        "FTMUSDT",
        "WIFUSDT",
        "ONDOUSDT",
        "THETAUSDT",
        "FLOKIUSDT",
        "JASMYUSDT",
        "NOTUSDT",
        "RUNEUSDT",
        "BONKUSDT",
        "BRETTUSDT",
        "FETUSDT",
        "AAVEUSDT",
        "TIAUSDT",
        "COREUSDT",
        "PYTHUSDT",
        "ALGOUSDT",
        "SEIUSDT",
        "FLRUSDT",
        "JUPUSDT",
        "KCSUSDT",
        "FLOWUSDT",
        "QNTUSDT",
        "TRXUSDT",
        "BSVUSDT",
        "MANAUSDT",
        "XECUSDT",
        "STRKUSDT",
        "XMRUSDT",
        "ARBUSDT",
        "MNTUSDT",
        "FILUSDT",
        "CROUSDT",
        "STXUSDT",
        "IMXUSDT",
        "MKRUSDT",
        "SUIUSDT",
        "VETUSDT",
        "GRTUSDT",
        "INJUSDT",
        "OPUSDT",
        "LDOUSDT",
        "TAOUSDT",
        "ARUSDT",
        "BGBUSDT",
        "DAIUSDT",
    ]
});

pub static BYBIT_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ETHUSDT", "USDCUSDT", "BNBUSDT", "SOLUSDT", "XRPUSDT", "TONUSDT", "DOGEUSDT", "ADAUSDT",
        "TRXUSDT", "SHIBUSDT", "AVAXUSDT", "DOTUSDT", "LINKUSDT", "BTCUSDT",
    ]
});

pub static BYBIT_KEYS_MOV_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ETHUSDT", "USDCUSDT", "BNBUSDT", "SOLUSDT", "XRPUSDT", "TONUSDT", "DOGEUSDT", "ADAUSDT",
        "TRXUSDT", "SHIBUSDT", "AVAXUSDT", "DOTUSDT", "LINKUSDT", "BTCUSDT",
    ]
});

pub static BYBIT_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("arbitrum_sepolia".to_string(), &BYBIT_KEYS_ARBITRUM_SEPOLIA);
    map.insert("bitlayer_testnet".to_string(), &BYBIT_KEYS_BITLAYER_TESTNET);
    map.insert("move_testnet".to_string(), &BYBIT_KEYS_MOV_TESTNET);
    map
});

pub static SYMBOL_MAP_BYBIT_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BTCUSDT", "BTC");
        map.insert("SANDUSDT", "SAND");
        map.insert("ARUSDT", "AR");
        map.insert("GALAUSDT", "GALA");
        map.insert("USDCUSDT", "USDC");
        map.insert("NEARUSDT", "NEAR");
        map.insert("FTMUSDT", "FTM");
        map.insert("ARBUSDT", "ARB");
        map.insert("BNBUSDT", "BNB");
        map.insert("XTZUSDT", "XTZ");
        map.insert("COREUSDT", "CORE");
        map.insert("APTUSDT", "APT");
        map.insert("UNIUSDT", "UNI");
        map.insert("ENSUSDT", "ENS");
        map.insert("MKRUSDT", "MKR");
        map.insert("NOTUSDT", "NOT");
        map.insert("SUIUSDT", "SUI");
        map.insert("EOSUSDT", "EOS");
        map.insert("THETAUSDT", "THETA");
        map.insert("AXSUSDT", "AXS");
        map.insert("FLOKIUSDT", "FLOKI");
        map.insert("LINKUSDT", "LINK");
        map.insert("KASUSDT", "KAS");
        map.insert("JASMYUSDT", "JASMY");
        map.insert("PEPEUSDT", "PEPE");
        map.insert("DAIUSDT", "DAI");
        map.insert("ICPUSDT", "ICP");
        map.insert("NEXOUSDT", "NEXO");
        map.insert("KCSUSDT", "KCS");
        map.insert("ETHUSDT", "ETH");
        map.insert("WLDUSDT", "WLD");
        map.insert("ETCUSDT", "ETC");
        map.insert("DOTUSDT", "DOT");
        map.insert("MNTUSDT", "MNT");
        map.insert("RUNEUSDT", "RUNE");
        map.insert("LDOUSDT", "LDO");
        map.insert("LTCUSDT", "LTC");
        map.insert("AAVEUSDT", "AAVE");
        map.insert("BEAMUSDT", "BEAM");
        map.insert("SEIUSDT", "SEI");
        map.insert("WIFUSDT", "WIF");
        map.insert("QNTUSDT", "QNT");
        map.insert("PYTHUSDT", "PYTH");
        map.insert("ATOMUSDT", "ATOM");
        map.insert("BONKUSDT", "BONK");
        map.insert("MANAUSDT", "MANA");
        map.insert("ALGOUSDT", "ALGO");
        map.insert("BCHUSDT", "BCH");
        map.insert("BRETTUSDT", "BRETT");
        map.insert("SOLUSDT", "SOL");
        map.insert("STRKUSDT", "STRK");
        map.insert("FLRUSDT", "FLR");
        map.insert("JUPUSDT", "JUP");
        map.insert("TIAUSDT", "TIA");
        map.insert("PENDLEUSDT", "PENDLE");
        map.insert("EGLDUSDT", "EGLD");
        map.insert("AGIXUSDT", "AGIX");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("TONUSDT", "TON");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("TRXUSDT", "TRX");
        map.insert("CHZUSDT", "CHZ");
        map.insert("XLMUSDT", "XLM");
        map.insert("FLOWUSDT", "FLOW");
        map.insert("GRTUSDT", "GRT");
        map.insert("MATICUSDT", "MATIC");
        map.insert("SNXUSDT", "SNX");
        map.insert("BTTUSDT", "BTT");
        map.insert("FILUSDT", "FIL");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("INJUSDT", "INJ");
        map.insert("ORDIUSDT", "ORDI");
        map.insert("ENAUSDT", "ENA");
        map.insert("HBARUSDT", "HBAR");
        map.insert("IMXUSDT", "IMX");
        map.insert("ONDOUSDT", "ONDO");
        map.insert("XECUSDT", "XEC");
        map.insert("ZROUSDT", "ZRO");
        map.insert("STXUSDT", "STX");
        map.insert("OPUSDT", "OP");
        map.insert("RNDRUSDT", "RNDR");
        map.insert("ADAUSDT", "ADA");
        map.insert("FETUSDT", "FET");
        map.insert("DYDXUSDT", "DYDX");
        map.insert("XRPUSDT", "XRP");
        map
    });

pub static SYMBOL_MAP_BYBIT_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("LINKUSDT", "LINK");
        map.insert("BNBUSDT", "BNB");
        map.insert("USDCUSDT", "USDC");
        map.insert("XRPUSDT", "XRP");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("DOTUSDT", "DOT");
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map.insert("SOLUSDT", "SOL");
        map.insert("TONUSDT", "TON");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("ADAUSDT", "ADA");
        map.insert("TRXUSDT", "TRX");
        map.insert("SHIBUSDT", "SHIB");
        map
    });

pub static SYMBOL_MAP_BYBIT_MOVE_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BTCUSDT", "BTC");
        map.insert("SANDUSDT", "SAND");
        map.insert("ARUSDT", "AR");
        map.insert("GALAUSDT", "GALA");
        map.insert("USDCUSDT", "USDC");
        map.insert("NEARUSDT", "NEAR");
        map.insert("FTMUSDT", "FTM");
        map.insert("ARBUSDT", "ARB");
        map.insert("BNBUSDT", "BNB");
        map.insert("XTZUSDT", "XTZ");
        map.insert("COREUSDT", "CORE");
        map.insert("APTUSDT", "APT");
        map.insert("UNIUSDT", "UNI");
        map.insert("ENSUSDT", "ENS");
        map.insert("MKRUSDT", "MKR");
        map.insert("NOTUSDT", "NOT");
        map.insert("SUIUSDT", "SUI");
        map.insert("EOSUSDT", "EOS");
        map.insert("THETAUSDT", "THETA");
        map.insert("AXSUSDT", "AXS");
        map.insert("FLOKIUSDT", "FLOKI");
        map.insert("LINKUSDT", "LINK");
        map.insert("KASUSDT", "KAS");
        map.insert("JASMYUSDT", "JASMY");
        map.insert("PEPEUSDT", "PEPE");
        map.insert("DAIUSDT", "DAI");
        map.insert("ICPUSDT", "ICP");
        map.insert("NEXOUSDT", "NEXO");
        map.insert("KCSUSDT", "KCS");
        map.insert("ETHUSDT", "ETH");
        map.insert("WLDUSDT", "WLD");
        map.insert("ETCUSDT", "ETC");
        map.insert("DOTUSDT", "DOT");
        map.insert("MNTUSDT", "MNT");
        map.insert("RUNEUSDT", "RUNE");
        map.insert("LDOUSDT", "LDO");
        map.insert("LTCUSDT", "LTC");
        map.insert("AAVEUSDT", "AAVE");
        map.insert("BEAMUSDT", "BEAM");
        map.insert("SEIUSDT", "SEI");
        map.insert("WIFUSDT", "WIF");
        map.insert("QNTUSDT", "QNT");
        map.insert("PYTHUSDT", "PYTH");
        map.insert("ATOMUSDT", "ATOM");
        map.insert("BONKUSDT", "BONK");
        map.insert("MANAUSDT", "MANA");
        map.insert("ALGOUSDT", "ALGO");
        map.insert("BCHUSDT", "BCH");
        map.insert("BRETTUSDT", "BRETT");
        map.insert("SOLUSDT", "SOL");
        map.insert("STRKUSDT", "STRK");
        map.insert("FLRUSDT", "FLR");
        map.insert("JUPUSDT", "JUP");
        map.insert("TIAUSDT", "TIA");
        map.insert("PENDLEUSDT", "PENDLE");
        map.insert("EGLDUSDT", "EGLD");
        map.insert("AGIXUSDT", "AGIX");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("TONUSDT", "TON");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("TRXUSDT", "TRX");
        map.insert("CHZUSDT", "CHZ");
        map.insert("XLMUSDT", "XLM");
        map.insert("FLOWUSDT", "FLOW");
        map.insert("GRTUSDT", "GRT");
        map.insert("MATICUSDT", "MATIC");
        map.insert("SNXUSDT", "SNX");
        map.insert("BTTUSDT", "BTT");
        map.insert("FILUSDT", "FIL");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("INJUSDT", "INJ");
        map.insert("ORDIUSDT", "ORDI");
        map.insert("ENAUSDT", "ENA");
        map.insert("HBARUSDT", "HBAR");
        map.insert("IMXUSDT", "IMX");
        map.insert("ONDOUSDT", "ONDO");
        map.insert("XECUSDT", "XEC");
        map.insert("ZROUSDT", "ZRO");
        map.insert("STXUSDT", "STX");
        map.insert("OPUSDT", "OP");
        map.insert("RNDRUSDT", "RNDR");
        map.insert("ADAUSDT", "ADA");
        map.insert("FETUSDT", "FET");
        map.insert("DYDXUSDT", "DYDX");
        map.insert("XRPUSDT", "XRP");
        map
    });

pub static BYBIT_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &SYMBOL_MAP_BYBIT_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_BYBIT_BITLAYER_TESTNET,
    );
    map.insert("move_testnet".to_string(), &SYMBOL_MAP_BYBIT_MOVE_SEPOLIA);
    map
});

pub static OKX_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        // "WBTC-USD",
        // "WETH-USD",
        "USDC-USD", "BNB-USD", "SOL-USD", "XRP-USD", "TON-USD", "DOGE-USD", "ADA-USD", "TRX-USD",
        "SHIB-USD", "AVAX-USD", "DOT-USD", "LINK-USD", // "WSTETH-USD",
        "BTC-USD", "ETH-USD",
    ]
});

pub static OKX_KEYS_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BTC-USD",
        "ETH-USD",
        "USDC-USD",
        // "WBTC-USD",
        // "WETH-USD",
        // "WSTETH-USD",
        "AXS-USD",
        "GALA-USD",
        "PENDLE-USD",
        "EOS-USD",
        "BTT-USD",
        "ENA-USD",
        "DYDX-USD",
        "BEAM-USD",
        "GNO-USD",
        "NEO-USD",
        "ORDI-USD",
        "AGIX-USD",
        "EGLD-USD",
        "XTZ-USD",
        "RON-USD",
        "SAND-USD",
        "GT-USD",
        "ENS-USD",
        "WLD-USD",
        "AKT-USD",
        "NEXO-USD",
        "ZRO-USD",
        "CHZ-USD",
        "SNX-USD",
        "DEXE-USD",
        "BNB-USD",
        "SOL-USD",
        "XRP-USD",
        "TON-USD",
        "DOGE-USD",
        "ADA-USD",
        "SHIB-USD",
        "AVAX-USD",
        "DOT-USD",
        "LINK-USD",
        "BCH-USD",
        "NEAR-USD",
        "UNI-USD",
        "MATIC-USD",
        "LEO-USD",
        "LTC-USD",
        "PEPE-USD",
        "ICP-USD",
        "KAS-USD",
        "ETC-USD",
        "APT-USD",
        "RNDR-USD",
        "HBAR-USD",
        "ATOM-USD",
        "XLM-USD",
        "OKB-USD",
        "FTM-USD",
        "WIF-USD",
        "ONDO-USD",
        "THETA-USD",
        "FLOKI-USD",
        "JASMY-USD",
        "NOT-USD",
        "RUNE-USD",
        "BONK-USD",
        "BRETT-USD",
        "FET-USD",
        "AAVE-USD",
        "TIA-USD",
        "CORE-USD",
        "PYTH-USD",
        "ALGO-USD",
        "SEI-USD",
        "FLR-USD",
        "JUP-USD",
        "KCS-USD",
        "FLOW-USD",
        "QNT-USD",
        "TRX-USD",
        "BSV-USD",
        "MANA-USD",
        "XEC-USD",
        "STRK-USD",
        "XMR-USD",
        "ARB-USD",
        "MNT-USD",
        "FIL-USD",
        "CRO-USD",
        "STX-USD",
        "IMX-USD",
        "MKR-USD",
        "SUI-USD",
        "VET-USD",
        "GRT-USD",
        "INJ-USD",
        "OP-USD",
        "LDO-USD",
        "TAO-USD",
        "AR-USD",
        "BGB-USD",
        "DAI-USD",
    ]
});

pub static OKX_KEYS_MVe_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BTC-USD",
        "ETH-USD",
        "USDC-USD",
        // "WBTC-USD",
        // "WETH-USD",
        // "WSTETH-USD",
        "AXS-USD",
        "GALA-USD",
        "PENDLE-USD",
        "EOS-USD",
        "BTT-USD",
        "ENA-USD",
        "DYDX-USD",
        "BEAM-USD",
        "GNO-USD",
        "NEO-USD",
        "ORDI-USD",
        "AGIX-USD",
        "EGLD-USD",
        "XTZ-USD",
        "RON-USD",
        "SAND-USD",
        "GT-USD",
        "ENS-USD",
        "WLD-USD",
        "AKT-USD",
        "NEXO-USD",
        "ZRO-USD",
        "CHZ-USD",
        "SNX-USD",
        "DEXE-USD",
        "BNB-USD",
        "SOL-USD",
        "XRP-USD",
        "TON-USD",
        "DOGE-USD",
        "ADA-USD",
        "SHIB-USD",
        "AVAX-USD",
        "DOT-USD",
        "LINK-USD",
        "BCH-USD",
        "NEAR-USD",
        "UNI-USD",
        "MATIC-USD",
        "LEO-USD",
        "LTC-USD",
        "PEPE-USD",
        "ICP-USD",
        "KAS-USD",
        "ETC-USD",
        "APT-USD",
        "RNDR-USD",
        "HBAR-USD",
        "ATOM-USD",
        "XLM-USD",
        "OKB-USD",
        "FTM-USD",
        "WIF-USD",
        "ONDO-USD",
        "THETA-USD",
        "FLOKI-USD",
        "JASMY-USD",
        "NOT-USD",
        "RUNE-USD",
        "BONK-USD",
        "BRETT-USD",
        "FET-USD",
        "AAVE-USD",
        "TIA-USD",
        "CORE-USD",
        "PYTH-USD",
        "ALGO-USD",
        "SEI-USD",
        "FLR-USD",
        "JUP-USD",
        "KCS-USD",
        "FLOW-USD",
        "QNT-USD",
        "TRX-USD",
        "BSV-USD",
        "MANA-USD",
        "XEC-USD",
        "STRK-USD",
        "XMR-USD",
        "ARB-USD",
        "MNT-USD",
        "FIL-USD",
        "CRO-USD",
        "STX-USD",
        "IMX-USD",
        "MKR-USD",
        "SUI-USD",
        "VET-USD",
        "GRT-USD",
        "INJ-USD",
        "OP-USD",
        "LDO-USD",
        "TAO-USD",
        "AR-USD",
        "BGB-USD",
        "DAI-USD",
    ]
});

pub static OKX_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("arbitrum_sepolia".to_string(), &OKX_KEYS_ARBITRUM_SEPOLIA);
    map.insert("bitlayer_testnet".to_string(), &OKX_KEYS_BITLAYER_TESTNET);
    map.insert("move_testnet".to_string(), &OKX_KEYS_MVe_SEPOLIA);
    map
});

pub static SYMBOL_MAP_OKX_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("XRP-USD", "XRP");
        map.insert("BNB-USD", "BNB");
        map.insert("LINK-USD", "LINK");
        map.insert("BTC-USD", "BTC");
        map.insert("ETH-USD", "ETH");
        map.insert("ADA-USD", "ADA");
        map.insert("AVAX-USD", "AVAX");
        map.insert("SOL-USD", "SOL");
        map.insert("SHIB-USD", "SHIB");
        map.insert("DOT-USD", "DOT");
        map.insert("TRX-USD", "TRX");
        map.insert("USDC-USD", "USDC");
        map.insert("TON-USD", "TON");
        map.insert("DOGE-USD", "DOGE");
        map
    });

pub static SYMBOL_MAP_OKX_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert("CHZ-USD", "CHZ");
        map.insert("RNDR-USD", "RNDR");
        map.insert("PYTH-USD", "PYTH");
        map.insert("XRP-USD", "XRP");
        map.insert("AAVE-USD", "AAVE");
        map.insert("OP-USD", "OP");
        map.insert("ATOM-USD", "ATOM");
        map.insert("XLM-USD", "XLM");
        map.insert("SNX-USD", "SNX");
        map.insert("BSV-USD", "BSV");
        map.insert("XTZ-USD", "XTZ");
        map.insert("LDO-USD", "LDO");
        map.insert("BNB-USD", "BNB");
        map.insert("ETC-USD", "ETC");
        map.insert("STRK-USD", "STRK");
        map.insert("LINK-USD", "LINK");
        map.insert("ALGO-USD", "ALGO");
        map.insert("CORE-USD", "CORE");
        map.insert("WIF-USD", "WIF");
        map.insert("MANA-USD", "MANA");
        map.insert("GALA-USD", "GALA");
        map.insert("FTM-USD", "FTM");
        map.insert("DYDX-USD", "DYDX");
        map.insert("FLR-USD", "FLR");
        map.insert("WLD-USD", "WLD");
        map.insert("HBAR-USD", "HBAR");
        map.insert("BTC-USD", "BTC");
        map.insert("INJ-USD", "INJ");
        map.insert("ADA-USD", "ADA");
        map.insert("ARB-USD", "ARB");
        map.insert("ICP-USD", "ICP");
        map.insert("SAND-USD", "SAND");
        map.insert("AVAX-USD", "AVAX");
        map.insert("GRT-USD", "GRT");
        map.insert("ENS-USD", "ENS");
        map.insert("APT-USD", "APT");
        map.insert("SOL-USD", "SOL");
        map.insert("IMX-USD", "IMX");
        map.insert("OKB-USD", "OKB");
        map.insert("FIL-USD", "FIL");
        map.insert("SHIB-USD", "SHIB");
        map.insert("AR-USD", "AR");
        map.insert("AXS-USD", "AXS");
        map.insert("STX-USD", "STX");
        map.insert("ETH-USD", "ETH");
        map.insert("FET-USD", "FET");
        map.insert("NEAR-USD", "NEAR");
        map.insert("DOT-USD", "DOT");
        map.insert("FLOKI-USD", "FLOKI");
        map.insert("TRX-USD", "TRX");
        map.insert("NOT-USD", "NOT");
        map.insert("EOS-USD", "EOS");
        map.insert("SUI-USD", "SUI");
        map.insert("USDC-USD", "USDC");
        map.insert("PEPE-USD", "PEPE");
        map.insert("LTC-USD", "LTC");
        map.insert("THETA-USD", "THETA");
        map.insert("TIA-USD", "TIA");
        map.insert("CRO-USD", "CRO");
        map.insert("UNI-USD", "UNI");
        map.insert("MKR-USD", "MKR");
        map.insert("TON-USD", "TON");
        map.insert("BCH-USD", "BCH");
        map.insert("BONK-USD", "BONK");
        map.insert("ORDI-USD", "ORDI");
        map.insert("DOGE-USD", "DOGE");
        map.insert("RON-USD", "RON");
        map.insert("EGLD-USD", "EGLD");
        map.insert("NEO-USD", "NEO");
        map.insert("MATIC-USD", "MATIC");
        map.insert("FLOW-USD", "FLOW");
        map.insert("JUP-USD", "JUP");
        map.insert("ZRO-USD", "ZRO");

        map
    });

pub static MOV_SEPLOIA: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert("CHZ-USD", "CHZ");
    map.insert("RNDR-USD", "RNDR");
    map.insert("PYTH-USD", "PYTH");
    map.insert("XRP-USD", "XRP");
    map.insert("AAVE-USD", "AAVE");
    map.insert("OP-USD", "OP");
    map.insert("ATOM-USD", "ATOM");
    map.insert("XLM-USD", "XLM");
    map.insert("SNX-USD", "SNX");
    map.insert("BSV-USD", "BSV");
    map.insert("XTZ-USD", "XTZ");
    map.insert("LDO-USD", "LDO");
    map.insert("BNB-USD", "BNB");
    map.insert("ETC-USD", "ETC");
    map.insert("STRK-USD", "STRK");
    map.insert("LINK-USD", "LINK");
    map.insert("ALGO-USD", "ALGO");
    map.insert("CORE-USD", "CORE");
    map.insert("WIF-USD", "WIF");
    map.insert("MANA-USD", "MANA");
    map.insert("GALA-USD", "GALA");
    map.insert("FTM-USD", "FTM");
    map.insert("DYDX-USD", "DYDX");
    map.insert("FLR-USD", "FLR");
    map.insert("WLD-USD", "WLD");
    map.insert("HBAR-USD", "HBAR");
    map.insert("BTC-USD", "BTC");
    map.insert("INJ-USD", "INJ");
    map.insert("ADA-USD", "ADA");
    map.insert("ARB-USD", "ARB");
    map.insert("ICP-USD", "ICP");
    map.insert("SAND-USD", "SAND");
    map.insert("AVAX-USD", "AVAX");
    map.insert("GRT-USD", "GRT");
    map.insert("ENS-USD", "ENS");
    map.insert("APT-USD", "APT");
    map.insert("SOL-USD", "SOL");
    map.insert("IMX-USD", "IMX");
    map.insert("OKB-USD", "OKB");
    map.insert("FIL-USD", "FIL");
    map.insert("SHIB-USD", "SHIB");
    map.insert("AR-USD", "AR");
    map.insert("AXS-USD", "AXS");
    map.insert("STX-USD", "STX");
    map.insert("ETH-USD", "ETH");
    map.insert("FET-USD", "FET");
    map.insert("NEAR-USD", "NEAR");
    map.insert("DOT-USD", "DOT");
    map.insert("FLOKI-USD", "FLOKI");
    map.insert("TRX-USD", "TRX");
    map.insert("NOT-USD", "NOT");
    map.insert("EOS-USD", "EOS");
    map.insert("SUI-USD", "SUI");
    map.insert("USDC-USD", "USDC");
    map.insert("PEPE-USD", "PEPE");
    map.insert("LTC-USD", "LTC");
    map.insert("THETA-USD", "THETA");
    map.insert("TIA-USD", "TIA");
    map.insert("CRO-USD", "CRO");
    map.insert("UNI-USD", "UNI");
    map.insert("MKR-USD", "MKR");
    map.insert("TON-USD", "TON");
    map.insert("BCH-USD", "BCH");
    map.insert("BONK-USD", "BONK");
    map.insert("ORDI-USD", "ORDI");
    map.insert("DOGE-USD", "DOGE");
    map.insert("RON-USD", "RON");
    map.insert("EGLD-USD", "EGLD");
    map.insert("NEO-USD", "NEO");
    map.insert("MATIC-USD", "MATIC");
    map.insert("FLOW-USD", "FLOW");
    map.insert("JUP-USD", "JUP");
    map.insert("ZRO-USD", "ZRO");

    map
});

pub static OKX_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &SYMBOL_MAP_OKX_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_OKX_BITLAYER_TESTNET,
    );
    map.insert("move_testnet".to_string(), &MOV_SEPLOIA);
    map
});

pub static KRAKEN_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "USDCUSDT", "SOLUSDT", "XRPUSDT", "ADAUSDT", "SHIBUSDT", "AVAXUSDT", "DOTUSDT", "LINKUSDT",
        "XBTUSDT", "ETHUSDT", "TRXUSD",
    ]
});

pub static KRAKEN_KEYS_ARBITRUM_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ADAUSDT",
        "ALGOUSDT",
        "ATOMUSDT",
        "AVAXUSDT",
        "BCHUSDT",
        "DAIUSDT",
        "DOTUSDT",
        "EOSUSDT",
        "ETHUSDT",
        "LINKUSDT",
        "LTCUSDT",
        "MANAUSDT",
        "MATICUSDT",
        "SHIBUSDT",
        "SOLUSDT",
        "USDCUSDT",
        "XMRUSDT",
        "XRPUSDT",
        "XTZUSDT",
        "XBTUSDT",
        "AAVEUSD",
        "AKTUSD",
        "APTUSD",
        "ARBUSD",
        "AXSUSD",
        "BEAMUSD",
        "BONKUSD",
        "BTTUSD",
        "CHZUSD",
        "DYDXUSD",
        "EGLDUSD",
        "ENAUSD",
        "ENSUSD",
        "FETUSD",
        "FILUSD",
        "FLOKIUSD",
        "FLOWUSD",
        "FLRUSD",
        "FTMUSD",
        "GALAUSD",
        "GNOUSD",
        "GRTUSD",
        "ICPUSD",
        "IMXUSD",
        "INJUSD",
        "JASMYUSD",
        "JUPUSD",
        "LDOUSD",
        "MKRUSD",
        "MNTUSD",
        "NEARUSD",
        "ONDOUSD",
        "OPUSD",
        "PENDLEUSD",
        "PEPEUSD",
        "PYTHUSD",
        "QNTUSD",
        "RNDRUSD",
        "RONUSD",
        "RUNEUSD",
        "SANDUSD",
        "SEIUSD",
        "SNXUSD",
        "STRKUSD",
        "STXUSD",
        "SUIUSD",
        "TAOUSD",
        "TIAUSD",
        "TRXUSD",
        "UNIUSD",
        "WIFUSD",
        "ZROUSD",
    ]
});

pub static KRAKEN_KEYS_MOVE_SEPOLIA: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ADAUSDT",
        "ALGOUSDT",
        "ATOMUSDT",
        "AVAXUSDT",
        "BCHUSDT",
        "DAIUSDT",
        "DOTUSDT",
        "EOSUSDT",
        "ETHUSDT",
        "LINKUSDT",
        "LTCUSDT",
        "MANAUSDT",
        "MATICUSDT",
        "SHIBUSDT",
        "SOLUSDT",
        "USDCUSDT",
        "XMRUSDT",
        "XRPUSDT",
        "XTZUSDT",
        "XBTUSDT",
        "AAVEUSD",
        "AKTUSD",
        "APTUSD",
        "ARBUSD",
        "AXSUSD",
        "BEAMUSD",
        "BONKUSD",
        "BTTUSD",
        "CHZUSD",
        "DYDXUSD",
        "EGLDUSD",
        "ENAUSD",
        "ENSUSD",
        "FETUSD",
        "FILUSD",
        "FLOKIUSD",
        "FLOWUSD",
        "FLRUSD",
        "FTMUSD",
        "GALAUSD",
        "GNOUSD",
        "GRTUSD",
        "ICPUSD",
        "IMXUSD",
        "INJUSD",
        "JASMYUSD",
        "JUPUSD",
        "LDOUSD",
        "MKRUSD",
        "MNTUSD",
        "NEARUSD",
        "ONDOUSD",
        "OPUSD",
        "PENDLEUSD",
        "PEPEUSD",
        "PYTHUSD",
        "QNTUSD",
        "RNDRUSD",
        "RONUSD",
        "RUNEUSD",
        "SANDUSD",
        "SEIUSD",
        "SNXUSD",
        "STRKUSD",
        "STXUSD",
        "SUIUSD",
        "TAOUSD",
        "TIAUSD",
        "TRXUSD",
        "UNIUSD",
        "WIFUSD",
        "ZROUSD",
    ]
});

pub static KRAKEN_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &KRAKEN_KEYS_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &KRAKEN_KEYS_BITLAYER_TESTNET,
    );
    map.insert("move_testnet".to_string(), &KRAKEN_KEYS_MOVE_SEPOLIA);
    map
});

pub static SYMBOL_MAP_KRAKEN_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("XBTUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map.insert("ADAUSDT", "ADA");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("DOTUSDT", "DOT");
        map.insert("LINKUSDT", "LINK");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("SOLUSDT", "SOL");
        map.insert("USDCUSDT", "USDC");
        map.insert("XRPUSDT", "XRP");
        map.insert("TRXUSD", "TRX");
        map
    });

pub static SYMBOL_MAP_KRAKEN_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("ADAUSDT", "ADA");
        map.insert("ALGOUSDT", "ALGO");
        map.insert("ATOMUSDT", "ATOM");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("BCHUSDT", "BCH");
        map.insert("DAIUSDT", "DAI");
        map.insert("DOTUSDT", "DOT");
        map.insert("EOSUSDT", "EOS");
        map.insert("ETHUSDT", "ETH");
        map.insert("LINKUSDT", "LINK");
        map.insert("LTCUSDT", "LTC");
        map.insert("MANAUSDT", "MANA");
        map.insert("MATICUSDT", "MATIC");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("SOLUSDT", "SOL");
        map.insert("USDCUSDT", "USDC");
        map.insert("XMRUSDT", "XMR");
        map.insert("XRPUSDT", "XRP");
        map.insert("XTZUSDT", "XTZ");
        map.insert("XBTUSDT", "BTC");
        map.insert("AAVEUSD", "AAVE");
        map.insert("AKTUSD", "AKT");
        map.insert("APTUSD", "APT");
        map.insert("ARBUSD", "ARB");
        map.insert("AXSUSD", "AXS");
        map.insert("BEAMUSD", "BEAM");
        map.insert("BONKUSD", "BONK");
        map.insert("BTTUSD", "BTT");
        map.insert("CHZUSD", "CHZ");
        map.insert("DYDXUSD", "DYDX");
        map.insert("EGLDUSD", "EGLD");
        map.insert("ENAUSD", "ENA");
        map.insert("ENSUSD", "ENS");
        map.insert("FETUSD", "FET");
        map.insert("FILUSD", "FIL");
        map.insert("FLOKIUSD", "FLOKI");
        map.insert("FLOWUSD", "FLOW");
        map.insert("FLRUSD", "FLR");
        map.insert("FTMUSD", "FTM");
        map.insert("GALAUSD", "GALA");
        map.insert("GNOUSD", "GNO");
        map.insert("GRTUSD", "GRT");
        map.insert("ICPUSD", "ICP");
        map.insert("IMXUSD", "IMX");
        map.insert("INJUSD", "INJ");
        map.insert("JASMYUSD", "JASMY");
        map.insert("JUPUSD", "JUP");
        map.insert("LDOUSD", "LDO");
        map.insert("MKRUSD", "MKR");
        map.insert("MNTUSD", "MNT");
        map.insert("NEARUSD", "NEAR");
        map.insert("ONDOUSD", "ONDO");
        map.insert("OPUSD", "OP");
        map.insert("PENDLEUSD", "PENDLE");
        map.insert("PEPEUSD", "PEPE");
        map.insert("PYTHUSD", "PYTH");
        map.insert("QNTUSD", "QNT");
        map.insert("RNDRUSD", "RNDR");
        map.insert("RONUSD", "RON");
        map.insert("RUNEUSD", "RUNE");
        map.insert("SANDUSD", "SAND");
        map.insert("SEIUSD", "SEI");
        map.insert("SNXUSD", "SNX");
        map.insert("STRKUSD", "STRK");
        map.insert("STXUSD", "STX");
        map.insert("SUIUSD", "SUI");
        map.insert("TAOUSD", "TAO");
        map.insert("TIAUSD", "TIA");
        map.insert("TRXUSD", "TRX");
        map.insert("UNIUSD", "UNI");
        map.insert("WIFUSD", "WIF");
        map.insert("ZROUSD", "ZRO");
        map
    });

pub static SYMBOL_MAP_KRAKEN_MOVE_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("XBTUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map.insert("ADAUSDT", "ADA");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("DOTUSDT", "DOT");
        map.insert("LINKUSDT", "LINK");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("SOLUSDT", "SOL");
        map.insert("USDCUSDT", "USDC");
        map.insert("XRPUSDT", "XRP");
        map.insert("TRXUSD", "TRX");
        map
    });

pub static KRAKEN_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "arbitrum_sepolia".to_string(),
        &SYMBOL_MAP_KRAKEN_ARBITRUM_SEPOLIA,
    );
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_KRAKEN_BITLAYER_TESTNET,
    );
    map.insert("move_testnet".to_string(), &SYMBOL_MAP_KRAKEN_MOVE_TESTNET);
    map
});

pub static PRICES_MAPPINGS_ARBITRUM_SEPOLIA: Lazy<HashMap<&'static str, Vec<f64>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BTC", vec![]);
        map.insert("ETH", vec![]);
        map.insert("USDC", vec![]);
        map.insert("WETH", vec![]);
        map.insert("WBTC", vec![]);
        map.insert("WSTETH", vec![]);
        map.insert("AXS", vec![]);
        map.insert("GALA", vec![]);
        map.insert("PENDLE", vec![]);
        map.insert("EOS", vec![]);
        map.insert("BTT", vec![]);
        map.insert("ENA", vec![]);
        map.insert("DYDX", vec![]);
        map.insert("BEAM", vec![]);
        map.insert("GNO", vec![]);
        map.insert("NEO", vec![]);
        map.insert("ORDI", vec![]);
        map.insert("AGIX", vec![]);
        map.insert("EGLD", vec![]);
        map.insert("XTZ", vec![]);
        map.insert("RON", vec![]);
        map.insert("SAND", vec![]);
        map.insert("GT", vec![]);
        map.insert("ENS", vec![]);
        map.insert("WLD", vec![]);
        map.insert("AKT", vec![]);
        map.insert("NEXO", vec![]);
        map.insert("ZRO", vec![]);
        map.insert("CHZ", vec![]);
        map.insert("SNX", vec![]);
        map.insert("DEXE", vec![]);
        map.insert("BNB", vec![]);
        map.insert("SOL", vec![]);
        map.insert("XRP", vec![]);
        map.insert("TON", vec![]);
        map.insert("DOGE", vec![]);
        map.insert("ADA", vec![]);
        map.insert("SHIB", vec![]);
        map.insert("AVAX", vec![]);
        map.insert("DOT", vec![]);
        map.insert("LINK", vec![]);
        map.insert("BCH", vec![]);
        map.insert("NEAR", vec![]);
        map.insert("UNI", vec![]);
        map.insert("MATIC", vec![]);
        map.insert("LEO", vec![]);
        map.insert("LTC", vec![]);
        map.insert("PEPE", vec![]);
        map.insert("ICP", vec![]);
        map.insert("KAS", vec![]);
        map.insert("ETC", vec![]);
        map.insert("APT", vec![]);
        map.insert("RNDR", vec![]);
        map.insert("HBAR", vec![]);
        map.insert("ATOM", vec![]);
        map.insert("XLM", vec![]);
        map.insert("OKB", vec![]);
        map.insert("FTM", vec![]);
        map.insert("WIF", vec![]);
        map.insert("ONDO", vec![]);
        map.insert("THETA", vec![]);
        map.insert("FLOKI", vec![]);
        map.insert("JASMY", vec![]);
        map.insert("NOT", vec![]);
        map.insert("RUNE", vec![]);
        map.insert("BONK", vec![]);
        map.insert("BRETT", vec![]);
        map.insert("FET", vec![]);
        map.insert("AAVE", vec![]);
        map.insert("TIA", vec![]);
        map.insert("CORE", vec![]);
        map.insert("PYTH", vec![]);
        map.insert("ALGO", vec![]);
        map.insert("SEI", vec![]);
        map.insert("FLR", vec![]);
        map.insert("JUP", vec![]);
        map.insert("KCS", vec![]);
        map.insert("FLOW", vec![]);
        map.insert("QNT", vec![]);
        map.insert("TRX", vec![]);
        map.insert("BSV", vec![]);
        map.insert("MANA", vec![]);
        map.insert("XEC", vec![]);
        map.insert("STRK", vec![]);
        map.insert("XMR", vec![]);
        map.insert("ARB", vec![]);
        map.insert("MNT", vec![]);
        map.insert("FIL", vec![]);
        map.insert("CRO", vec![]);
        map.insert("STX", vec![]);
        map.insert("IMX", vec![]);
        map.insert("MKR", vec![]);
        map.insert("SUI", vec![]);
        map.insert("VET", vec![]);
        map.insert("GRT", vec![]);
        map.insert("INJ", vec![]);
        map.insert("OP", vec![]);
        map.insert("LDO", vec![]);
        map.insert("TAO", vec![]);
        map.insert("AR", vec![]);
        map.insert("BGB", vec![]);
        map.insert("DAI", vec![]);
        map
    });

pub static PRICES_MAPPINGS_BITLAYER_TESTNET: Lazy<HashMap<&'static str, Vec<f64>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BTC", vec![]);
        map.insert("WBTC", vec![]);
        map.insert("WETH", vec![]);
        map.insert("USDC", vec![]);
        map.insert("BNB", vec![]);
        map.insert("SOL", vec![]);
        map.insert("XRP", vec![]);
        map.insert("TON", vec![]);
        map.insert("DOGE", vec![]);
        map.insert("ADA", vec![]);
        map.insert("TRX", vec![]);
        map.insert("SHIB", vec![]);
        map.insert("AVAX", vec![]);
        map.insert("DOT", vec![]);
        map.insert("LINK", vec![]);
        map.insert("WSTETH", vec![]);
        map
    });

pub static PRICES_MAPPINGS_MOVE_SEPOLIA: Lazy<HashMap<&'static str, Vec<f64>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("BTC", vec![]);
    map.insert("ETH", vec![]);
    map.insert("USDC", vec![]);
    map.insert("WETH", vec![]);
    map.insert("PENDLE", vec![]);
    map.insert("SOL", vec![]);
    map.insert("HYPE", vec![]);
    map
});

pub static PRICES_MAPPINGS: Lazy<HashMap<String, &'static Lazy<HashMap<&'static str, Vec<f64>>>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "arbitrum_sepolia".to_string(),
            &PRICES_MAPPINGS_ARBITRUM_SEPOLIA,
        );
        map.insert(
            "bitlayer_testnet".to_string(),
            &PRICES_MAPPINGS_BITLAYER_TESTNET,
        );
        map.insert("move_testnet".to_string(), &PRICES_MAPPINGS_MOVE_SEPOLIA);
        map
    });
