

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
    pub address: Vec<String>,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: String,
    pub address: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainTransaction {
    pub txid: String,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
}