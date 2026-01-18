#![allow(dead_code)]
use serde::{Deserialize, Serialize};

// ============================================================================
// Structs matching the XSD schema (resources/localidades.xsd)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct Root {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: Option<f32>,
    pub origen: Origen,
    pub elaborado: String,
    pub nombre: String,
    pub provincia: String,
    pub prediccion: Prediccion,
}

#[derive(Debug, Deserialize)]
pub struct Origen {
    pub productor: String,
    pub web: String,
    pub enlace: String,
    pub language: String,
    pub copyright: String,
    pub nota_legal: String,
}

#[derive(Debug, Deserialize)]
pub struct Prediccion {
    pub dia: Vec<Dia>,
}

#[derive(Debug, Deserialize)]
pub struct Dia {
    #[serde(rename = "@fecha")]
    pub fecha: Option<String>,
    pub prob_precipitacion: Vec<ProbPrecipitacion>,
    pub cota_nieve_prov: Vec<CotaNieveProv>,
    pub estado_cielo: Vec<EstadoCielo>,
    pub viento: Vec<Viento>,
    pub racha_max: Vec<RachaMax>,
    pub temperatura: Temperatura,
    pub sens_termica: SensTermica,
    pub humedad_relativa: HumedadRelativa,
    #[serde(default)]
    pub uv_max: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProbPrecipitacion {
    #[serde(rename = "@periodo")]
    pub periodo: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CotaNieveProv {
    #[serde(rename = "@periodo")]
    pub periodo: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EstadoCielo {
    #[serde(rename = "@periodo")]
    pub periodo: Option<String>,
    #[serde(rename = "@descripcion")]
    pub descripcion: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Viento {
    #[serde(rename = "@periodo")]
    pub periodo: Option<String>,
    pub direccion: String,
    pub velocidad: String,
}

#[derive(Debug, Deserialize)]
pub struct RachaMax {
    #[serde(rename = "@periodo")]
    pub periodo: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Temperatura {
    pub maxima: i8,
    pub minima: i8,
    #[serde(default)]
    pub dato: Vec<Dato>,
}

#[derive(Debug, Deserialize)]
pub struct SensTermica {
    pub maxima: i8,
    pub minima: i8,
    #[serde(default)]
    pub dato: Vec<Dato>,
}

#[derive(Debug, Deserialize)]
pub struct HumedadRelativa {
    pub maxima: i8,
    pub minima: i8,
    #[serde(default)]
    pub dato: Vec<Dato>,
}

#[derive(Debug, Deserialize)]
pub struct Dato {
    #[serde(rename = "@hora")]
    pub hora: Option<i8>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

// ============================================================================
// Mail request body struct
// ============================================================================

#[derive(Debug, Serialize)]
pub struct MailRequest {
    pub mail: Mail,
}

#[derive(Debug, Serialize)]
pub struct Mail {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub text: String,
    pub encoding: String,
}
