# YUAIRENDER - Your Ultimate Adaptable Intelligence Renderer 🚀✨

**Врываемся с космическим настроением!** 
Привет, гении кода, мастера дизайна и властелины данных! `YUAIRENDER` — это не просто рендер, это мир, где данные превращаются в шедевры быстрее, чем вы скажете "Вау!". HTML с шаблонами из FIGMA? JSON для машин? Protobuf для молний? RAW? У нас есть всё — хватайте и творите Rust магию!

## Что это за чудо?
`YUAIRENDER` — это библиотека на Rust, которая имеет:
- **Гибкость уровня "вау"**: HTML, JSON, XML, CSV, PlainText, Markdown, Protobuf — выбирайте формат и вперёд!
- **Шаблоны мечты**: HTML и Markdown с циклами `{% for %}` и условиями `{% if %}` — создавайте шедевры без лишних движений!
- **Дружелюбность**: Нет данных? Плейсхолдер пропал? Пфф, пропускаем и летим дальше!
- **Универсальность**: Берёт `Vec<HashMap<String, String>>` из любого уголка мультивселенной — кидайте свои данные, мы разберёмся!

## Зачем вам этот космос?
- **HTML-шаблоны для Figma-гуру**: Рисуете в Figma? Экспортируйте в HTML, вставляйте `{{ variable }}` — и ваши макеты оживают! Для дизайнеров — это как кисть и холст: просто, красиво, идеально.
- **SSR, CSR, гидрация — звёздный уровень**:
  - **Server-Side Rendering (SSR)**: HTML рендерится на сервере со скоростью света — поисковики в восторге, пользователи видят страницу мгновенно!
  - **Client-Side Rendering (CSR)**: Сырые данные и шаблоны летят к клиенту — рендерьте в JS и добавляйте динамику на лету!
  - **Гидрация**: Сервер выдаёт HTML, клиент оживляет его с данными — плавно, как танец звёзд, без мигания и перезагрузок!
- **JSON для машин**: Компактный и чёткий — идеален для API и серверов, где каждая миллисекунда на счету.
- **XML для классиков**: Надёжный, как старый друг, — для систем, где традиции важнее трендов.
- **CSV для гениальных аналитиков**: Таблицы готовы к Excel или скриптам — считайте и анализируйте без суеты!
- **PlainText для минималистов**: Просто текст — для логов, консолей и тех, кто любит всё по-простому!
- **Markdown для поэтов данных**: Красивый и читаемый — для документации, блогов и историй, которые вдохновляют!
- **Protobuf для космической скорости**: Бинарный формат — компактный, быстрый, для тех, кто живёт на опережение!

## Как запустить эту ракету?
1. **Грузим в проект**:
   ```toml
   [dependencies]
   yuairender = { git = "https://github.com/cthvlab/yuairender" }
   yuaidb = "0.1" # Укажи свою версию базы
   tokio = { version = "1.0", features = ["full"] } # Для асинхронных приключений
   ```

2. **Пример эпичного старта с гидрацией**:
   ```rust
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
   ```
  
3. **Шаблон `templates/pirates.html`**:
   ```html
    <h1>Корабль {{ name }}</h1>
    {% for treasure in treasures %}
      <p>Сокровище: {{ treasure }}</p>
    {% endfor %}
    {% if active %}
      <footer>Капитан на борту!</footer>
    {% else %}
      <footer>Капитан спит!</footer>
    {% endif %}

   ```

4. **Клиентский JS для гидрации (`hydrate.js`)**:
   ```javascript
   // Хватаем данные из космоса
   const initialData = window.__INITIAL_DATA__[0] || {};

   // Оживляем страницу!
   const app = document.getElementById("app");
   app.querySelectorAll(".hobby-item").forEach((item, index) => {
       item.addEventListener("click", () => {
           alert(`Космическое хобби: ${initialData.hobbies.split(",")[index]}!`);
       });
   });

   const footer = app.querySelector(".status");
   if (initialData.active === "true") {
       footer.style.color = "green";
       footer.textContent += " (Кликни для привета!)";
       footer.addEventListener("click", () => alert(`Привет от ${initialData.name}!`));
   } else {
       footer.style.color = "gray";
   }
   ```

## Что у нас в арсенале?
- **Форматы**: HTML, JSON, XML, CSV, PlainText, Markdown, Protobuf — полный набор для любой миссии!
- **Шаблоны**: HTML и Markdown с циклами `{% for %}` и условиями `{% if %}` — пропускаем всё, что не нашли, и летим дальше!
- **Простота**: Формат, шаблон (если надо), данные — и готово, никаких сложностей!
- **Гибкость**: Хотите сырые данные? `RenderOutput::Raw` — ваш лучший друг!

## Как это работает?
1. Выбираете формат и шаблон через `YuaiRender::new`.
2. Кидаете данные из `yuaidb` или откуда угодно — `Option<Vec<HashMap<String, String>>>`.
3. Получаете `RenderOutput` — либо готовую строку, либо сырые данные для своих космических планов!

## Почему это лучшее для SSR, CSR и гидрации?
- **SSR**: HTML рендерится на сервере мгновенно — для SEO и скорости загрузки, как ракета на старте!
- **CSR**: Передавайте `RenderOutput::Raw` и шаблон клиенту — рендерьте в JS и добавляйте динамику, как звёздный танец!
- **Гидрация**: Сервер отдаёт HTML с JSON, клиент оживляет его — плавно, без рывков, как полёт через гиперпространство!

## Вперёд к бесконечности!
Готовы покорять мультивселенную? Клонируйте, форкайте, кидайте свои шаблоны и идеи! Нашли баг или мечтаете о новом формате? Открывайте issues или шлите PR — мы ждём героев со всех уголков галактики!

## Лицензия 📜
MIT — берите, используйте, веселитесь!

## Авторы
Разработано сообществом ЮАИ [yuai.ru](https://yuai.ru) 
