use std::collections::HashMap;
#[derive(Debug)]
struct Replacements(HashMap<String, Vec<(String, String)>>);
impl Replacements {
    fn new() -> Self {
        Replacements(HashMap::new())
    }
}
use std::iter::FromIterator;

impl FromIterator<(String, (String, String))> for Replacements {
    fn from_iter<I: IntoIterator<Item = (String, (String, String))>>(
        iter: I,
    ) -> Self {
        let mut r = Replacements(HashMap::new());
        for i in iter {
            if !r.0.contains_key(&i.0) {
                r.0.insert(i.0, vec![i.1]);
            } else {
                r.0.get_mut(&i.0).unwrap().append(&mut vec![i.1.clone()]);
            }
        }
        r
    }
}
fn create_replacement_map() -> Replacements {
    vec![("getaddressdeltas", (r#"a"#, r#"b"#))]
        .iter()
        .map(|(x, y)| (x.to_string(), (y.0.to_string(), y.1.to_string())))
        .collect::<Vec<(String, (String, String))>>()
        .into_iter()
        .collect::<Replacements>()
}
macro_rules! getaddressdeltas {
    ($result_data:expr) => {
        $result_data
            .replace(r#"(or, if chainInfo is true):"#, "Result:")
            .replace("number", "numeric")
            .replace(", ...", "")
            .replace(
                r#"  "start":
    {
      "hash"          (string)  The start block hash
      "height"        (numeric) The height of the start block
    }"#,
                r#"  "start":
    {
      "hash":         (string)  The start block hash
      "height":       (numeric) The height of the start block
    }"#,
            )
            .replace(
                r#"  "end":
    {
      "hash"          (string)  The end block hash
      "height"        (numeric) The height of the end block
    }"#,
                r#"  "end":
    {
      "hash":         (string)  The end block hash
      "height":       (numeric) The height of the end block
    }"#,
            )
    };
}

macro_rules! getaddressmempool {
    ($result_data:expr) => {
        $result_data.replace(r#"number"#, r#"numeric"#)
    };
}

macro_rules! getblockchaininfo {
    ($result_data:expr) => {
    $result_data.replace("[0..1]", "").replace(
        "{ ... }      (object) progress toward rejecting pre-softfork blocks",
        "{
\"status\": (boolean)
\"found\": (numeric)
\"required\": (numeric)
\"window\": (numeric)
}").replace("(same fields as \"enforce\")", "").replace(", ...", "")
    };
}

macro_rules! getblockdeltas {
    ($result_data:expr) => {
        $result_data
            .replace(r#"hex string"#, r#"hexadecimal"#)
            .replace(r#"hexstring"#, r#"hexadecimal"#)
            .replace(r#", ..."#, r#""#)
    };
}

macro_rules! getblockhashes {
    ($result_data:expr) => {
        $result_data
            .replace(r#"hex string"#, r#"hexadecimal"#)
            .replace(r#"hexstring"#, r#"hexadecimal"#)
            .replace(r#", ..."#, r#""#)
    };
}

macro_rules! getchaintips {
    ($result_data:expr) => {
    $result_data.replace(
            r#"Possible values for status:
1.  "invalid"               This branch contains at least one invalid block
2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
5.  "active"                This is the tip of the active main chain, which is certainly valid"#, "")
.replace(r#""height": xxxx,
"#, r#""height": xxxx,         (numeric) height of the chain tip
"#).replace(r#""hash": "xxxx",
"#, r#""hash": "xxxx",         (string) block hash of the tip
"#)
    };
}

macro_rules! getdeprecationinfo {
    ($result_data:expr) => {
        $result_data.replace(r#"MagicBean:x.y.z[-v]"#, r#"MagicBean"#)
    };
}

macro_rules! getnetworkinfo {
    ($result_data:expr) => {
        $result_data
            .replace(r#"MagicBean:x.y.z[-v]"#, r#"MagicBean"#)
            .replace(r#",..."#, r#""#)
    };
}

macro_rules! getpeerinfo {
    ($result_data:expr) => {
        $result_data
            .replace(r#"MagicBean:x.y.z[-v]"#, r#"MagicBean"#)
            .replace(r#",..."#, r#""#)
    };
}

macro_rules! getspentinfo {
    ($result_data:expr) => {
        $result_data.replace(r#"number"#, r#"numeric"#).replace(
            r#"  ,...
"#,
            r#""#,
        )
    };
}

macro_rules! gettransaction {
    ($result_data:expr) => {
    $result_data.replace(r#"      "nullifiers" : [ string, ... ]      (string) Nullifiers of input notes
      "commitments" : [ string, ... ]     (string) Note commitments for note outputs
      "macs" : [ string, ... ]            (string) Message authentication tags"#,
    r#""nullifiers": [
        "nullifier" (string)
    ],
    "commitments": [
        "commitment" (string)
    ],
    "macs": [
        "mac" (string)
    ],"#).replace(r#",..."#,r#""#).replace(r#", ..."#,r#""#)
    };
}

macro_rules! listaccounts{
    ($result_data:expr) =>  {
    $result_data.replace(r#"                      (json object where keys are account names, and values are numeric balances"#, "")
        .replace(r#"  ...
"#, "")
    };
}

macro_rules! listreceivedbyaccount {
    ($result_data:expr) => {
        $result_data.replace(r#"bool"#, "boolean").replace(
            r#"  ,...
"#,
            "",
        )
    };
}

macro_rules! listreceivedbyaddress {
    ($result_data:expr) => {
        $result_data.replace(r#"bool"#, "boolean").replace(
            r#"  ,...
"#,
            "",
        )
    };
}
macro_rules! listtransactions {
    ($result_data:expr) => {
        $result_data
            .lines()
            .filter(|l| {
                !l.starts_with("                                         ")
            })
            .fold(String::new(), |mut accumulator, new| {
                accumulator.push_str(new);
                accumulator.push_str("\n");
                accumulator
            })
    };
}

// TODO the submitblock macro returns an ad-hoc string rather
// than following a suggested pattern for amending help output
macro_rules! submitblock {
    ($result_data:expr) => {
        r#""(enum: duplicate, duplicate-invalid, duplicate-inconclusive, inconclusive, rejected)""#.to_string()
    };
}

const INSUFFICIENT: &str = r#"INSUFFICIENT_INFORMATION
Result:
"do_not_use_this": (INSUFFICIENT)

Examples:
None"#;
macro_rules! getblocktemplate {
    ($result_data:expr) => {
        INSUFFICIENT.to_string()
    };
}

macro_rules! z_getoperationresult {
    ($result_data:expr) => {
        INSUFFICIENT.to_string()
    };
}

macro_rules! z_getoperationstatus {
    ($result_data:expr) => {
        INSUFFICIENT.to_string()
    };
}

macro_rules! z_listreceivedbyaddress {
    ($result_data:expr) => {
        $result_data
            .replace(r#" (sprout) : n,"#, r#": n, <sprout> "#)
            .replace(r#" (sapling) : n,"#, r#": n, <sapling> "#)
    };
}

macro_rules! z_validateaddress {
    ($result_data:expr) => {
        $result_data
            .replace(r#"[sprout]"#, r#"<sprout>"#)
            .replace(r#"[sapling]"#, r#"<sapling>"#)
    };
}

macro_rules! getrawmempool {
    ($result_data:expr) => {
        $result_data
            .replace(r#"Result: (for verbose = false):
[                     (json array of string)
  "transactionid"     (string) The transaction id
  ,...
]

Result: (for verbose = true):
{                           (json object)
  "transactionid" : {       (json object)
    "size" : n,             (numeric) transaction size in bytes
    "fee" : n,              (numeric) transaction fee in ZEC
    "time" : n,             (numeric) local time transaction entered pool in seconds since 1 Jan 1970 GMT
    "height" : n,           (numeric) block height when transaction entered pool
    "startingpriority" : n, (numeric) priority when transaction entered pool
    "currentpriority" : n,  (numeric) transaction priority now
    "depends" : [           (array) unconfirmed transactions used as inputs for this transaction
        "transactionid",    (string) parent transaction id
       ... ]
  }, ...
}"#,

r#"Result:
[                     
  "transactionid"     (string) The transaction id
]

Result:
{
  "transactionid" : {
    "size" : n,             (numeric) transaction size in bytes
    "fee" : n,              (numeric) transaction fee in ZEC
    "time" : n,             (numeric) local time transaction entered pool in seconds since 1 Jan 1970 GMT
    "height" : n,           (numeric) block height when transaction entered pool
    "startingpriority" : n, (numeric) priority when transaction entered pool
    "currentpriority" : n,  (numeric) transaction priority now
    "depends" : [
        "transactionid",    (string) parent transaction id
        ]
  }
}"#)
    };
}

macro_rules! getblockheader {
    ($result_data:expr) => {
        $result_data.replace(r#"Result (for verbose = true):
{
  "hash" : "hash",     (string) the block hash (same as provided)
  "confirmations" : n,   (numeric) The number of confirmations, or -1 if the block is not on the main chain
  "height" : n,          (numeric) The block height or index
  "version" : n,         (numeric) The block version
  "merkleroot" : "xxxx", (string) The merkle root
  "finalsaplingroot" : "xxxx", (string) The root of the Sapling commitment tree after applying this block
  "time" : ttt,          (numeric) The block time in seconds since epoch (Jan 1 1970 GMT)
  "nonce" : n,           (numeric) The nonce
  "bits" : "1d00ffff", (string) The bits
  "difficulty" : x.xxx,  (numeric) The difficulty
  "previousblockhash" : "hash",  (string) The hash of the previous block
  "nextblockhash" : "hash"       (string) The hash of the next block
}

Result (for verbose=false):
"data"             (string) A string that is serialized, hex-encoded data for block 'hash'."#,
r#"Result:
"data"             (string) A string that is serialized, hex-encoded data for block 'hash'.

"Result:
{
  "hash" : "hash",     (string) the block hash (same as provided)
  "confirmations" : n,   (numeric) The number of confirmations, or -1 if the block is not on the main chain
  "height" : n,          (numeric) The block height or index
  "version" : n,         (numeric) The block version
  "merkleroot" : "xxxx", (string) The merkle root
  "finalsaplingroot" : "xxxx", (string) The root of the Sapling commitment tree after applying this block
  "time" : ttt,          (numeric) The block time in seconds since epoch (Jan 1 1970 GMT)
  "nonce" : n,           (numeric) The nonce
  "bits" : "1d00ffff", (string) The bits
  "difficulty" : x.xxx,  (numeric) The difficulty
  "previousblockhash" : "hash",  (string) The hash of the previous block
  "nextblockhash" : "hash"       (string) The hash of the next block
}"#)
    };
}

//TODO turn into individual scrubbers
macro_rules! dotdotdot {
    ($result_data:expr) => {
        $result_data
            .replace(", ...\n", r#""#)
            .replace(",...\n", r#""#)
    };
}

pub(crate) fn scrub(cmd_name: String, result_data: String) -> String {
    create_replacement_map();
    let result_data = result_data
        .replace(r#", ..."#, r#""#)
        .replace(r#",..."#, r#""#);
    if cmd_name == "getaddressdeltas".to_string() {
        getaddressdeltas!(result_data)
    } else if cmd_name == "getblockheader".to_string() {
        getblockheader!(result_data)
    } else if cmd_name == "getrawmempool".to_string() {
        getrawmempool!(result_data)
    } else if cmd_name == "getaddressmempool".to_string() {
        getaddressmempool!(result_data)
    } else if cmd_name == "getchaintips".to_string() {
        getchaintips!(result_data)
    } else if cmd_name == "getblockchaininfo".to_string() {
        getblockchaininfo!(result_data)
    } else if cmd_name == "getblockdeltas".to_string() {
        getblockdeltas!(result_data)
    } else if cmd_name == "getblockhashes".to_string() {
        getblockhashes!(result_data)
    } else if cmd_name == "getdeprecationinfo".to_string() {
        getdeprecationinfo!(result_data)
    } else if cmd_name == "getnetworkinfo".to_string() {
        getnetworkinfo!(result_data)
    } else if cmd_name == "getpeerinfo".to_string() {
        getpeerinfo!(result_data)
    } else if cmd_name == "getspentinfo".to_string() {
        getspentinfo!(result_data)
    } else if cmd_name == "gettransaction".to_string() {
        gettransaction!(result_data)
    } else if cmd_name == "listaccounts".to_string() {
        listaccounts!(result_data)
    } else if cmd_name == "listreceivedbyaccount".to_string() {
        listreceivedbyaccount!(result_data)
    } else if cmd_name == "listreceivedbyaddress".to_string() {
        listreceivedbyaddress!(result_data)
    } else if cmd_name == "listtransactions".to_string() {
        listtransactions!(result_data)
    } else if cmd_name == "submitblock".to_string() {
        submitblock!(result_data)
    } else if cmd_name == "z_getoperationresult".to_string() {
        z_getoperationresult!(result_data)
    } else if cmd_name == "z_getoperationstatus".to_string() {
        z_getoperationstatus!(result_data)
    } else if cmd_name == "z_listreceivedbyaddress".to_string() {
        z_listreceivedbyaddress!(result_data)
    } else if cmd_name == "z_validateaddress".to_string() {
        z_validateaddress!(result_data)
    } else if cmd_name == "getblocktemplate".to_string() {
        getblocktemplate!(result_data)
    } else {
        result_data
    }
}
