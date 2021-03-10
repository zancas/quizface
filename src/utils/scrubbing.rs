macro_rules! getaddressdeltas {
    ($result_data:expr) => {
        $result_data
            .split(r#"(or, if chainInfo is true):"#)
            .collect::<Vec<&str>>()[1]
            .trim()
            .to_string()
            .replace(r#""deltas":"#, r#""alsoStandalone<deltas>":"#)
            .replace(
                r#"        "satoshis"    (number) The difference of zatoshis
        "txid"        (string) The related txid
        "index"       (number) The related input or output index
        "height"      (number) The block height
        "address"     (string)  The address base58check encoded"#,
                r#"        "satoshis":   (numeric) The difference of zatoshis
        "txid":       (string) The related txid
        "index":      (numeric) The related input or output index
        "height":     (numeric) The block height
        "address":    (string)  The address base58check encoded"#,
            )
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

macro_rules! getblockchaininfo{
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

macro_rules! getchaintips{
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

macro_rules! gettransaction{
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
    ($result_data:expr) => {
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

macro_rules! submitblock {
    ($result_data:expr) => {
        $result_data.replace(r#"duplicate" - node already has valid copy of block
"duplicate-invalid" - node already has block, but it is invalid
"duplicate-inconclusive" - node already has block but has not validated it
"inconclusive" - node has not validated the block, it may not be on the node's current best chain
"rejected" - block was rejected as invalid
For more information on submitblock parameters and results, see: https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki#block-submission"#,
r#"duplicate": (boolean) node already has valid copy of block
"duplicate-invalid": (boolean) node already has block, but it is invalid
"duplicate-inconclusive": (boolean) node already has block but has not validated it
"inconclusive": (boolean)node has not validated the block, it may not be on the node's current best chain
"rejected": (boolean) block was rejected as invalid"#)
    }
}

macro_rules! z_getoperationresult {
    ($result_data:expr) => {
        $result_data.replace(
            r#"(array) A list of JSON objects"#,
            r#"(INSUFFICIENT) A list of JSON objects"#,
        )
    };
}

macro_rules! z_getoperationstatus {
    ($result_data:expr) => {
        $result_data.replace(
            r#"(array) A list of JSON objects"#,
            r#"(INSUFFICIENT) A list of JSON objects"#,
        )
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

macro_rules! dotdotdot {
    ($result_data:expr) => {
        $result_data
            .replace(r#", ..."#, r#""#)
            .replace(r#",..."#, r#""#)
    };
}

pub(crate) fn scrub(cmd_name: String, result_data: String) -> String {
    if cmd_name == "getaddressdeltas".to_string() {
        getaddressdeltas!(result_data)
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
    } else {
        dotdotdot!(result_data)
    }
}

#[cfg(test)]
mod test {
    macro_rules! build_cmd_replace_map {
        [$($cmd:literal| $($replacement:literal),*);*] => {
            $(
                dbg!($cmd);
            )*;
        }
    }
    #[test]
    fn run_dm() {
        build_cmd_replace_map!["f" | "b"; "a" | "q"];
    }
}
