use yuaidb::Database; // Наш корабль с добычей!
use yuairender::{YuaiRender, RenderOutput};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new("data", "config.toml").await?;
    let query = db.select("pirates")
        .fields(vec!["name", "treasures", "active"])
        .where_eq("gold", "100");

    // HTML с пользовательским шаблоном
    let html_renderer = YuaiRender::new("html", Some("templates/pirate.html"))?;
    let result = query.clone().execute(&db).await?;
    match html_renderer.render(result)? {
        RenderOutput::Rendered(html) => println!("HTML добыча:\n{}", html),
        _ => unreachable!(),
    }

    // Markdown с дефолтным шаблоном
    let md_renderer = YuaiRender::new("markdown", None)?;
    let result = query.clone().execute(&db).await?;
    match md_renderer.render(result)? {
        RenderOutput::Rendered(md) => println!("Markdown добыча:\n{}", md),
        _ => unreachable!(),
    }

    // JSON без шаблона
    let json_renderer = YuaiRender::new("json", None)?;
    let result = query.clone().execute(&db).await?;
    match json_renderer.render(result)? {
        RenderOutput::Rendered(json) => println!("JSON добыча:\n{}", json),
        _ => unreachable!(),
    }

    // Сырые данные
    let raw_renderer = YuaiRender::new("plaintext", None)?; // Просто для примера
    let result = query.execute(&db).await?;
    match raw_renderer.render(result)? {
        RenderOutput::Raw(data) => {
            if let Some(rows) = data {
                for row in rows {
                    println!("Сырая добыча: {:?}", row);
                }
            } else {
                println!("Йо-хо-хо! Трюм пуст!");
            }
        }
        _ => println!("Йо-хо-хо! Добыча в сундуке!"),
    }

    Ok(())
}
