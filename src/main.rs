mod dto;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use dto::{Mail, MailRequest, Root};
use encoding_rs::ISO_8859_15;
use log::{error, info};
use quick_xml::de::from_str;
use std::process::ExitCode;

// ============================================================================
// Helper functions
// ============================================================================

fn format_weather_report(root: &Root) -> String {
    let mut report = String::new();

    report.push_str(&format!("METEO FORECAST FOR {} ({})\n", root.nombre, root.provincia));
    report.push_str(&format!("Generated: {}\n", root.elaborado));
    report.push_str(&format!("Source: {}\n", root.origen.productor));
    report.push_str(&"=".repeat(60));
    report.push('\n');

    for dia in &root.prediccion.dia {
        if let Some(ref fecha) = dia.fecha {
            report.push_str(&format!("\n📅 DATE: {}\n", fecha));
            report.push_str(&"-".repeat(40));
            report.push('\n');
        }

        // Temperature
        report.push_str(&format!(
            "\n🌡️  TEMPERATURE: {}°C - {}°C\n",
            dia.temperatura.minima, dia.temperatura.maxima
        ));
        for dato in &dia.temperatura.dato {
            if let (Some(hora), Some(value)) = (dato.hora, &dato.value) {
                if !value.is_empty() {
                    report.push_str(&format!("    {:02}:00 → {}°C\n", hora, value));
                }
            }
        }

        // Thermal sensation
        report.push_str(&format!(
            "\n🌡️  THERMAL SENSATION: {}°C - {}°C\n",
            dia.sens_termica.minima, dia.sens_termica.maxima
        ));
        for dato in &dia.sens_termica.dato {
            if let (Some(hora), Some(value)) = (dato.hora, &dato.value) {
                if !value.is_empty() {
                    report.push_str(&format!("    {:02}:00 → {}°C\n", hora, value));
                }
            }
        }

        // Relative humidity
        report.push_str(&format!(
            "\n💧 RELATIVE HUMIDITY: {}% - {}%\n",
            dia.humedad_relativa.minima, dia.humedad_relativa.maxima
        ));
        for dato in &dia.humedad_relativa.dato {
            if let (Some(hora), Some(value)) = (dato.hora, &dato.value) {
                if !value.is_empty() {
                    report.push_str(&format!("    {:02}:00 → {}%\n", hora, value));
                }
            }
        }

        // Precipitation probability
        let has_precip: Vec<_> = dia
            .prob_precipitacion
            .iter()
            .filter(|p| p.value.as_ref().is_some_and(|v| !v.is_empty()))
            .collect();
        if !has_precip.is_empty() {
            report.push_str("\n🌧️  PRECIPITATION PROBABILITY:\n");
            for prob in has_precip {
                let periodo = prob.periodo.as_deref().unwrap_or("all day");
                let value = prob.value.as_deref().unwrap_or("0");
                report.push_str(&format!("    {} → {}%\n", periodo, value));
            }
        }

        // Snow level
        let has_snow: Vec<_> = dia
            .cota_nieve_prov
            .iter()
            .filter(|c| c.value.as_ref().is_some_and(|v| !v.is_empty()))
            .collect();
        if !has_snow.is_empty() {
            report.push_str("\n❄️  SNOW LEVEL:\n");
            for cota in has_snow {
                let periodo = cota.periodo.as_deref().unwrap_or("all day");
                let value = cota.value.as_deref().unwrap_or("-");
                report.push_str(&format!("    {} → {}m\n", periodo, value));
            }
        }

        // Sky condition
        let has_sky: Vec<_> = dia
            .estado_cielo
            .iter()
            .filter(|e| e.descripcion.as_ref().is_some_and(|d| !d.is_empty()))
            .collect();
        if !has_sky.is_empty() {
            report.push_str("\n☁️  SKY CONDITION:\n");
            for estado in has_sky {
                let periodo = estado.periodo.as_deref().unwrap_or("all day");
                let desc = estado.descripcion.as_deref().unwrap_or("-");
                report.push_str(&format!("    {} → {}\n", periodo, desc));
            }
        }

        // Wind
        let has_wind: Vec<_> = dia
            .viento
            .iter()
            .filter(|v| !v.direccion.is_empty() || !v.velocidad.is_empty())
            .collect();
        if !has_wind.is_empty() {
            report.push_str("\n💨 WIND:\n");
            for viento in has_wind {
                let periodo = viento.periodo.as_deref().unwrap_or("all day");
                report.push_str(&format!(
                    "    {} → {} at {} km/h\n",
                    periodo, viento.direccion, viento.velocidad
                ));
            }
        }

        // Wind gust
        let has_gust: Vec<_> = dia
            .racha_max
            .iter()
            .filter(|r| r.value.as_ref().is_some_and(|v| !v.is_empty()))
            .collect();
        if !has_gust.is_empty() {
            report.push_str("\n💨 MAX WIND GUST:\n");
            for racha in has_gust {
                let periodo = racha.periodo.as_deref().unwrap_or("all day");
                let value = racha.value.as_deref().unwrap_or("-");
                report.push_str(&format!("    {} → {} km/h\n", periodo, value));
            }
        }

        // UV index
        if let Some(ref uv) = dia.uv_max {
            if !uv.is_empty() {
                report.push_str(&format!("\n☀️  UV INDEX (max): {}\n", uv));
            }
        }

        report.push_str(&"\n");
        report.push_str(&"=".repeat(60));
        report.push('\n');
    }

    report.push_str(&format!("\n{}\n", root.origen.nota_legal));

    report
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting meteo-forecast application");

    // Download XML from AEMET
    let xml_url = std::env::var("AEMET_LOCATION").unwrap_or_default();
    if xml_url.is_empty() {
        return Err("AEMET_LOCATION environment variable is not set".into());
    }
    info!("Downloading weather data from {}", xml_url);

    let response = reqwest::blocking::get(xml_url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to download XML: HTTP {}", response.status()).into());
    }

    // AEMET uses ISO-8859-15 encoding, decode from raw bytes
    let raw_bytes = response.bytes()?;
    let (xml_content, _, _) = ISO_8859_15.decode(&raw_bytes);
    info!("XML downloaded successfully ({} bytes)", xml_content.len());

    // Parse XML into structs
    info!("Parsing XML data");
    let root: Root = from_str(&xml_content)?;
    info!(
        "Parsed weather data for {} ({})",
        root.nombre, root.provincia
    );

    // Format weather report in human-readable way
    let report = format_weather_report(&root);
    info!("Weather report generated ({} chars)", report.len());

    // Encode report in base64
    let encoded_report = BASE64.encode(report.as_bytes());

    let mail_from = std::env::var("MAIL_FROM").unwrap_or_default();
    if mail_from.is_empty() {
        return Err("MAIL_FROM environment variable is not set".into());
    }

    let mail_to = std::env::var("MAIL_TO").unwrap_or_default();
    if mail_to.is_empty() {
        return Err("MAIL_TO environment variable is not set".into());
    }
    let recipients: Vec<String> = mail_to.split(',').map(|s| s.trim().to_string()).collect();

    // Build mail request body
    let mail_request = MailRequest {
        mail: Mail {
            from: mail_from,
            to: recipients,
            subject: "Meteo forecast".to_string(),
            text: encoded_report,
            encoding: "base64".to_string(),
        },
    };

    let json_body = serde_json::to_string(&mail_request)?;

    let rustmail_url = std::env::var("RUSTMAIL_URL").unwrap_or_default();
    if rustmail_url.is_empty() {
        return Err("RUSTMAIL_URL environment variable is not set".into());
    }
    info!("Sending mail request to {}", rustmail_url);

    // Send POST request
    let client = reqwest::blocking::Client::new();
    let post_response = client
        .post(&rustmail_url)
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()?;

    if !post_response.status().is_success() {
        return Err(format!(
            "Failed to send mail request: HTTP {}",
            post_response.status()
        ).into());
    }

    info!("Mail request sent successfully");
    Ok(())
}

fn main() -> ExitCode {
    env_logger::init();

    match run() {
        Ok(()) => {
            info!("Application completed successfully");
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("Application error: {}", e);
            ExitCode::FAILURE
        }
    }
}
