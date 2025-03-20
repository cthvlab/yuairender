# YUAIRENDER - Your Ultimate Adaptable Intelligence Renderer 🚀✨

**Врываемся с космическим настроением!** 
Привет, гении кода, мастера дизайна и властелины данных! `YUAIRENDER` — это не просто рендер, это мир, где данные превращаются в шедевры быстрее, чем вы скажете "Вау!". HTML с шаблонами из FIGMA? JSON для машин? Protobuf для молний? RAW? У нас есть всё — хватайте и творите Rust магию!

## Что это за чудо?
`YUAIRENDER` — это библиотека на Rust, которая:
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
   use yuairender::{YuaiRender, RenderOutput};
   use yuaidb::Database;

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       // Подключаемся к базе — нашему звёздному хранилищу!
       let db = Database::new("data", "config.toml").await?;
       let query = db.select("users")
           .fields(vec!["name", "hobbies", "active"])
           .where_eq("status", "online");

       // SSR: Рендерим HTML на сервере
       let html_renderer = YuaiRender::new("html", Some("templates/user.html"))?;
       let data = query.clone().execute(&db).await?;
       let html = match html_renderer.render(data.clone())? {
           RenderOutput::Rendered(html) => html,
           _ => unreachable!(),
       };

       // JSON для гидрации: данные для клиента
       let json_renderer = YuaiRender::new("json", None)?;
       let json = match json_renderer.render(data)? {
           RenderOutput::Rendered(json) => json,
           _ => unreachable!(),
       };

       // Итоговый HTML с данными для гидрации
       let response = format!(
           r#"
           <!DOCTYPE html>
           <html>
           <head>
               <title>Галактический привет!</title>
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

       println!("Космический HTML с гидрацией:\n{}", response);
       Ok(())
   }
   ```

3. **Шаблон `templates/user.html`**:
   ```html
   <h1>Привет, {{ name }}!</h1>
   <ul>
   {% for hobby in hobbies %}
     <li class="hobby-item" data-hobby="{{ hobby }}">Любимое дело: {{ hobby }}</li>
   {% endfor %}
   </ul>
   {% if active %}
     <footer class="status">Онлайн и сияет!</footer>
   {% else %}
     <footer class="status">Пока в спящем режиме!</footer>
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

## Зависимости
- `serde` и `serde_json` — для JSON-магии.
- `bincode` и `base64` — для Protobuf-скорости.
- `regex` — для парсинга шаблонов.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"
base64 = "0.13"
regex = "1.5"
```

## Для кого этот рендер?
- **Figma-мастера**: HTML-шаблоны — ваш космический холст! Экспортируйте макеты, добавляйте данные — и звёзды ваши!
- **API-гении**: JSON и Protobuf — для молниеносных и точных данных!
- **Аналитики**: CSV — таблицы для подсчётов на световой скорости!
- **Писатели**: Markdown — для заметок и историй, которые покоряют галактики!
- **Минималисты**: PlainText — чисто, просто, без лишнего шума!
- **Классики**: XML — надёжно, как старая звезда!

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
