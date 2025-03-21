use yuaidb::Database;
use yuairender::{YuaiRender, RenderOutput};

#[tokio::main]
async fn main() {
    // Запускаем звездолёт базы данных — полный вперёд!
    let db = match Database::new("data", "config.toml").await {
        Ok(db) => {
            println!("База данных на орбите, капитан!");
            Some(db)
        }
        Err(e) => {
            println!("Сбой в гипердвигателе базы: {}. Летим без неё, йо-хо!", e);
            None
        }
    };

    // Грузим корабли в трюм, если база на связи!
    if let Some(ref db) = db {
        let mut insert_ships = db.insert("ships"); // Новый фрегат для флота!
        insert_ships.values(vec![
            vec![("ship_id", "101"), ("name", "Чёрная Комета"), ("speed", "0.9")],
            vec![("ship_id", "102"), ("name", "Астероидный Шторм"), ("speed", "0.7")],
        ]);
        match insert_ships.execute(db).await {
            Ok(_) => println!("Флот пополнен новыми звездолётами!"),
            Err(e) => println!("Корабли затерялись в туманности: {}. Плывём дальше!", e),
        }
    } else {
        println!("База в чёрной дыре, кораблей не будет, увы!");
    }

    // Зовём пиратов на борт, если есть где их записать!
    if let Some(ref db) = db {
        let mut insert_pirates = db.insert("pirates"); // Экипаж для космического разбоя!
        insert_pirates.values(vec![
            vec![("name", "Капитан Джек Воробот"), ("ship_id", "101")],
            vec![("name", "Лихой Иван"), ("ship_id", "102")],
            vec![("name", "Морской Волк"), ("ship_id", "101")],
        ]);
        match insert_pirates.execute(db).await {
            Ok(_) => println!("Пираты поднялись на борт, ром на месте!"),
            Err(e) => println!("Пираты сбежали с корабля: {}. Идём дальше!", e),
        }
    } else {
        println!("Без базы пираты остались на астероиде!");
    }

    // Ищем добычу в звёздных архивах!
    let data = if let Some(ref db) = db {
        let mut query = db.select("pirates"); // Сканируем галактику за пиратами!
        query.alias("p"); // Даём кодовое имя "p" для шпионов!
        query.fields(vec!["p.name", "s.name", "s.speed"]); // Что у нас в сундуке?
        query.join("ships", "s", "s.ship_id", "p.ship_id"); // Соединяем флот с экипажем!
        query.order_by("p.name", true); // Сортируем по алфавиту, как в пиратском уставе!
        query.limit(10); // Не больше 10 сокровищ за раз!

        match query.execute(db).await {
            Ok(Some(rows)) => Some(rows), // Нашли сундук с данными!
            Ok(None) => {
                println!("Сундук пуст, пираты спрятали добычу!");
                None
            }
            Err(e) => {
                println!("Космическая буря в запросе: {}. Летим без груза!", e);
                None
            }
        }
    } else {
        println!("Без базы — без добычи, рендерим пустой космос!");
        None
    };

    // Рисуем карту сокровищ в HTML!
    let html = match YuaiRender::new("html", Some("templates/pirates.html")) {
        Ok(html_renderer) => match html_renderer.render(data.clone()) {
            Ok(RenderOutput::Rendered(html)) => html, // Карта готова, капитан!
            Ok(RenderOutput::Raw(_)) => unreachable!(), // Секретный код, сюда не попадём!
            Err(e) => {
                println!("Шторм в рендере HTML: {}. Кидаем заглушку!", e);
                "<p>Ошибка: звёзды скрыты!</p>".to_string()
            }
        },
        Err(e) => {
            println!("Картограф сбежал с шаблоном: {}. Ставим метку!", e);
            "<p>Ошибка: карта потеряна!</p>".to_string()
        }
    };

    // Пакуем добычу в JSON для космических шпионов!
    let json = match YuaiRender::new("json", None) {
        Ok(json_renderer) => match json_renderer.render(data) {
            Ok(RenderOutput::Rendered(json)) => json, // Данные в сундуке JSON!
            Ok(RenderOutput::Raw(_)) => unreachable!(), // Тайный ход, не для нас!
            Err(e) => {
                println!("Ошибка в упаковке JSON: {}. Пустой сундук!", e);
                "{}".to_string()
            }
        },
        Err(e) => {
            println!("JSON-машина сломалась: {}. Берём пустышку!", e);
            "{}".to_string()
        }
    };

    // Собираем звёздный корабль HTML с добычей!
    let response = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Звёздный пират</title>
        </head>
        <body>
            <div id="app">{}</div>
            <script type="text/javascript">
                window.__INITIAL_DATA__ = {};
            </script>
            <script src="/hydrate.js"></script>
        </body>
        </html>
        "#,
        html, json
    );

    // Показываем карту галактической добычи!
    println!("Готовый HTML с гидрацией:\n{}", response);
}
