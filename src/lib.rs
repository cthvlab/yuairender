use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

// Ошибки рендера
#[derive(Debug)]
pub enum RenderError {
    UnknownFormat(String),      // Клиент запросил формат, которого нет в нашей карте!
    FileError(String),          // Не смогли открыть файл с шаблоном!
    SerializationError(String), // Ошибка при превращении добычи в строку (JSON, Protobuf)!
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::UnknownFormat(fmt) => write!(f, "Формат '{}' не опознан!", fmt),
            RenderError::FileError(msg) => write!(f, "Ошибка с файлом: {}", msg),
            RenderError::SerializationError(msg) => write!(f, "Ошибка рендера: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}

// Форматы рендера — как отдаём добычу!
#[derive(Debug, Clone, PartialEq)]
pub enum RenderFormat {
    Html,       // HTML с шаблонами для FIGMA boys & girls!
    Json,       // JSON — для машинного разбора!
    Xml,        // XML — старые свитки мореходов!
    Csv,        // CSV — счёт дублонов в строках!
    PlainText,  // Текст — просто и без рома!
    Markdown,   // Markdown с шаблонами — для легенд!
    Protobuf,   // Protobuf — байты для скорости!
}

impl FromStr for RenderFormat {
    type Err = RenderError;

    // Превращаем строку от клиента в наш формат рендера!
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "html" => Ok(RenderFormat::Html),
            "json" => Ok(RenderFormat::Json),
            "xml" => Ok(RenderFormat::Xml),
            "csv" => Ok(RenderFormat::Csv),
            "text" | "plain" => Ok(RenderFormat::PlainText),
            "markdown" | "md" => Ok(RenderFormat::Markdown),
            "protobuf" | "proto" => Ok(RenderFormat::Protobuf),
            _ => Err(RenderError::UnknownFormat(s.to_string())),
        }
    }
}

// Токены шаблона — куски, которые мы будем обрабатывать!
#[derive(Debug)]
enum TemplateToken {
    Text(String),             // Обычный текст из шаблона, без магии!
    Variable(String),         // Переменная вроде {{ name }} — берём добычу!
    ForStart(String, String), // Начало цикла {% for item in items %} — повторяем добычу!
    ForEnd,                   // Конец цикла {% endfor %} — закрываем!
    IfStart(String),          // Начало условия {% if show %} — проверяем!
    Else,                     // Альтернатива {% else %} — другой путь!
    IfEnd,                    // Конец условия {% endif %} — возвращаемся на курс!
}

// Главный рендер
pub struct YuaiRender {
    format: RenderFormat,       // Какой формат выбран?
    template: Option<String>,   // Путь к шаблону (для HTML и Markdown)!
}

impl YuaiRender {
    // Новый рендер — с форматом и шаблоном!
    pub fn new(format: &str, template: Option<&str>) -> Result<Self, RenderError> {
        let render_format = RenderFormat::from_str(format)?; // Парсим формат!
        let template_path = template.map(|t| t.to_string()); // Если шаблон указан, сохраняем путь!
        Ok(YuaiRender {
            format: render_format,
            template: template_path,
        })
    }

    // Рендерим добычу — превращаем данные в строку или оставляем сырыми!
    pub fn render(&self, data: Option<Vec<HashMap<String, String>>>) -> Result<RenderOutput, RenderError> {
        match self.format {
            // HTML с шаблоном!
            RenderFormat::Html => {
                let template_content = self.load_template("templates/default.html")?;
                let rendered = self.render_template(&template_content, data.unwrap_or_default())?;
                Ok(RenderOutput::Rendered(rendered))
            }
            // JSON!
            RenderFormat::Json => {
                let json = serde_json::to_string(&data)
                    .map_err(|e| RenderError::SerializationError(format!("Не могу закодировать в JSON: {}", e)))?;
                Ok(RenderOutput::Rendered(json))
            }
            // XML!
            RenderFormat::Xml => {
                let mut output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rows>");
                if let Some(rows) = &data {
                    if rows.is_empty() { return Ok(RenderOutput::Rendered("<rows></rows>".to_string())); }
                    for row in rows {
                        output.push_str("\n  <row>");
                        for (key, value) in row {
                            output.push_str(&format!("\n    <{}>{}</{}>", key, value, key));
                        }
                        output.push_str("\n  </row>");
                    }
                    output.push_str("\n</rows>");
                } else {
                    output.push_str("<rows></rows>");
                }
                Ok(RenderOutput::Rendered(output))
            }
            // CSV!
            RenderFormat::Csv => {
                let mut output = String::new();
                if let Some(rows) = &data {
                    if rows.is_empty() { return Ok(RenderOutput::Rendered("".to_string())); }
                    let first_row = &rows[0];
                    output.push_str(&first_row.keys().collect::<Vec<_>>().join(","));
                    output.push('\n');
                    for row in rows {
                        let values: Vec<String> = row.values().map(|v| format!("\"{}\"", v.replace("\"", "\"\""))).collect();
                        output.push_str(&values.join(","));
                        output.push('\n');
                    }
                }
                Ok(RenderOutput::Rendered(output))
            }
            // PlainText!
            RenderFormat::PlainText => {
                let mut output = String::new();
                if let Some(rows) = &data {
                    if rows.is_empty() { return Ok(RenderOutput::Rendered("Йо-хо-хо! Трюм пуст!".to_string())); }
                    for row in rows {
                        for (key, value) in row {
                            output.push_str(&format!("{}: {}\n", key, value));
                        }
                        output.push_str("---\n");
                    }
                } else {
                    output.push_str("Йо-хо-хо! Трюм пуст!");
                }
                Ok(RenderOutput::Rendered(output))
            }
            // Markdown с шаблоном!
            RenderFormat::Markdown => {
                let template_content = self.load_template("templates/default.md")?;
                let rendered = self.render_template(&template_content, data.unwrap_or_default())?;
                Ok(RenderOutput::Rendered(rendered))
            }
            // Protobuf!
            RenderFormat::Protobuf => {
                let bytes = bincode::serialize(&data)
                    .map_err(|e| RenderError::SerializationError(format!("Не могу закодировать в байты: {}", e)))?;
                let encoded = base64::encode(bytes);
                Ok(RenderOutput::Rendered(encoded))
            }
        }
    }

    // Загружаем шаблон!
    fn load_template(&self, default_path: &str) -> Result<String, RenderError> {
        let path = self.template.as_ref().map(|s| s.as_str()).unwrap_or(default_path);
        read_to_string(path)
            .map_err(|e| RenderError::FileError(format!("Не могу открыть шаблон '{}': {}", path, e)))
    }

    // Рендерим шаблон — превращаем карту в добычу!
    fn render_template(&self, template: &str, data: Vec<HashMap<String, String>>) -> Result<String, RenderError> {
        let tokens = self.parse_template(template); // Разбиваем шаблон на куски!
        let mut output = String::new();
        let mut stack = Vec::new(); // Стек для вложенных блоков (циклы, условия)!

        for token in tokens {
            match token {
                TemplateToken::Text(text) => output.push_str(&text), // Просто текст — в сундук!
                TemplateToken::Variable(var) => {
                    // Ищем переменную в первой строке данных (для простоты)!
                    if let Some(row) = data.first() {
                        if let Some(value) = row.get(&var) {
                            output.push_str(value); // Нашли добычу — добавляем!
                        }
                    }
                }
                TemplateToken::ForStart(item_name, list_name) => {
                    stack.push((item_name, list_name, output.len())); // Запоминаем начало цикла!
                }
                TemplateToken::ForEnd => {
                    if let Some((item_name, list_name, start_pos)) = stack.pop() {
                        let loop_content = &output[start_pos..]; // Берём содержимое цикла!
                        output.truncate(start_pos); // Обрезаем до начала цикла!
                        for row in &data {
                            if let Some(list) = row.get(&list_name) {
                                // Предполагаем, что список — это строка с разделителями (например, "a,b,c")!
                                for item in list.split(',') {
                                    let mut temp = loop_content.to_string();
                                    temp = temp.replace(&format!("{{ {} }}", item_name), item);
                                    output.push_str(&temp); // Повторяем для каждого элемента!
                                }
                            }
                        }
                    }
                }
                TemplateToken::IfStart(condition) => {
                    stack.push((condition, output.len())); // Запоминаем начало условия!
                }
                TemplateToken::Else => {
                    if let Some((condition, start_pos)) = stack.last_mut() {
                        let if_content = &output[*start_pos..]; // Сохраняем содержимое if!
                        output.truncate(*start_pos); // Обрезаем до начала!
                        if let Some(row) = data.first() {
                            if row.get(condition).map(|v| v == "true").unwrap_or(false) {
                                output.push_str(if_content); // Условие истинно — оставляем if!
                            }
                        }
                        *start_pos = output.len(); // Перемещаем позицию для else!
                    }
                }
                TemplateToken::IfEnd => {
                    if let Some((condition, start_pos)) = stack.pop() {
                        let else_content = &output[start_pos..]; // Берём содержимое else (если есть)!
                        output.truncate(start_pos); // Обрезаем до начала else!
                        if let Some(row) = data.first() {
                            if !row.get(&condition).map(|v| v == "true").unwrap_or(false) {
                                output.push_str(else_content); // Условие ложно — берём else!
                            }
                        }
                    }
                }
            }
        }
        Ok(output) // Отдаём готовую добычу!
    }

    // Парсим шаблон — разбиваем на куски для обработки!
    fn parse_template(&self, template: &str) -> Vec<TemplateToken> {
        let mut tokens = Vec::new();
        let mut remaining = template;
        let re = regex::Regex::new(r"(\{\{.*?\}\}|\{%.*?%\})").unwrap();

        while let Some(mat) = re.find(remaining) {
            let before = &remaining[..mat.start()]; // Текст до токена!
            if !before.is_empty() {
                tokens.push(TemplateToken::Text(before.to_string()));
            }

            let token_str = mat.as_str();
            if token_str.starts_with("{{") && token_str.ends_with("}}") {
                let var = token_str[2..token_str.len() - 2].trim().to_string();
                tokens.push(TemplateToken::Variable(var)); // Переменная!
            } else if token_str.starts_with("{%") && token_str.ends_with("%}") {
                let content = token_str[2..token_str.len() - 2].trim();
                if content.starts_with("for ") {
                    let parts: Vec<&str> = content[4..].split(" in ").collect();
                    if parts.len() == 2 {
                        let item_name = parts[0].trim().to_string();
                        let list_name = parts[1].trim().to_string();
                        tokens.push(TemplateToken::ForStart(item_name, list_name)); // Начало цикла!
                    }
                } else if content == "endfor" {
                    tokens.push(TemplateToken::ForEnd); // Конец цикла!
                } else if content.starts_with("if ") {
                    let condition = content[3..].trim().to_string();
                    tokens.push(TemplateToken::IfStart(condition)); // Начало условия!
                } else if content == "else" {
                    tokens.push(TemplateToken::Else); // Альтернатива!
                } else if content == "endif" {
                    tokens.push(TemplateToken::IfEnd); // Конец условия!
                }
            }

            remaining = &remaining[mat.end()..];
        }
        if !remaining.is_empty() {
            tokens.push(TemplateToken::Text(remaining.to_string())); // Остаток текста!
        }
        tokens
    }
}

// Выход рендера!
pub enum RenderOutput {
    Rendered(String),                  // Готовая строка!
    Raw(Option<Vec<HashMap<String, String>>>), // Сырые данные — разбирай сам!
}
