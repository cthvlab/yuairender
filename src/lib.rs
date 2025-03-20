use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::str::FromStr;

// Ошибки рендера — если что-то пошло не по звёздам!
#[derive(Debug)]
pub enum RenderError {
    UnknownFormat(String),      // Формат не из нашей галактики!
    FileError(String),          // Не нашли файл в космосе!
    SerializationError(String), // Ошибка при упаковке данных!
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::UnknownFormat(fmt) => write!(f, "Ой-ой! Формат '{}' не из нашей вселенной!", fmt),
            RenderError::FileError(msg) => write!(f, "Космическая буря! Ошибка с файлом: {}", msg),
            RenderError::SerializationError(msg) => write!(f, "Телепорт сломался! Ошибка: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}

// Форматы рендера — выбираем курс!
#[derive(Debug, Clone, PartialEq)]
pub enum RenderFormat {
    Html,       // HTML с шаблонами — для звёздных дизайнов!
    Json,       // JSON — для машинных орбит!
    Xml,        // XML — для ретро-шаттлов!
    Csv,        // CSV — для звёздных таблиц!
    PlainText,  // Текст — просто и чисто!
    Markdown,   // Markdown с шаблонами — для галактических заметок!
    Protobuf,   // Protobuf — гиперскорость в байтах!
}

impl FromStr for RenderFormat {
    type Err = RenderError;

    // Парсим формат из строки — курс на звёзды!
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

// Токены шаблона — куски звёздной карты!
#[derive(Debug)]
enum TemplateToken {
    Text(String),             // Обычный текст — звёздная пыль!
    Variable(String),         // Переменная {{ name }} — данные из космоса!
    ForStart(String, String), // Начало цикла {% for item in items %} — звёздный цикл!
    ForEnd,                   // Конец цикла {% endfor %} — закрываем орбиту!
    IfStart(String),          // Начало условия {% if active %} — проверяем звёзды!
    Else,                     // Альтернатива {% else %} — другой путь!
    IfEnd,                    // Конец условия {% endif %} — возвращаемся на курс!
    Include(String),          // Включение {% include "header.html" %} — звёздный модуль!
}

// Элементы стека — для циклов и условий!
#[derive(Debug)]
enum StackItem {
    ForLoop(String, String, usize), // Для {% for %} — (item_name, list_name, start_pos)
    IfCond(String, usize),          // Для {% if %} — (condition, start_pos)
}

// Главный рендер — наш звездолёт!
pub struct YuaiRender {
    format: RenderFormat,       // Какой формат выбрали?
    template: Option<String>,   // Путь к основному шаблону (если есть)!
}

impl YuaiRender {
    // Новый рендер — готовим звездолёт к полёту!
    pub fn new(format: &str, template: Option<&str>) -> Result<Self, RenderError> {
        let render_format = RenderFormat::from_str(format)?;
        let template_path = template.map(|t| t.to_string());
        Ok(YuaiRender {
            format: render_format,
            template: template_path,
        })
    }

    // Рендерим данные — запускаем двигатели!
    pub fn render(&self, data: Option<Vec<HashMap<String, String>>>) -> Result<RenderOutput, RenderError> {
        match self.format {
            RenderFormat::Html => {
                let template_content = self.load_template("templates/default.html")?;
                let rendered = self.render_template(&template_content, data.unwrap_or_default(), &mut HashSet::new())?;
                Ok(RenderOutput::Rendered(rendered))
            }
            RenderFormat::Json => {
                let json = serde_json::to_string(&data)
                    .map_err(|e| RenderError::SerializationError(format!("Не могу закодировать в JSON: {}", e)))?;
                Ok(RenderOutput::Rendered(json))
            }
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
            RenderFormat::Csv => {
                let mut output = String::new();
                if let Some(rows) = &data {
                    if rows.is_empty() { return Ok(RenderOutput::Rendered("".to_string())); }
                    let first_row = &rows[0];
                    output.push_str(&first_row.keys().map(|k| k.as_str()).collect::<Vec<_>>().join(","));
                    output.push('\n');
                    for row in rows {
                        let values: Vec<String> = row.values().map(|v| format!("\"{}\"", v.replace("\"", "\"\""))).collect();
                        output.push_str(&values.join(","));
                        output.push('\n');
                    }
                }
                Ok(RenderOutput::Rendered(output))
            }
            RenderFormat::PlainText => {
                let mut output = String::new();
                if let Some(rows) = &data {
                    if rows.is_empty() { return Ok(RenderOutput::Rendered("Эй! Космос пуст!".to_string())); }
                    for row in rows {
                        for (key, value) in row {
                            output.push_str(&format!("{}: {}\n", key, value));
                        }
                        output.push_str("---\n");
                    }
                } else {
                    output.push_str("Эй! Космос пуст!");
                }
                Ok(RenderOutput::Rendered(output))
            }
            RenderFormat::Markdown => {
                let template_content = self.load_template("templates/default.md")?;
                let rendered = self.render_template(&template_content, data.unwrap_or_default(), &mut HashSet::new())?;
                Ok(RenderOutput::Rendered(rendered))
            }
            RenderFormat::Protobuf => {
                let bytes = bincode::serialize(&data)
                    .map_err(|e| RenderError::SerializationError(format!("Не могу закодировать в байты: {}", e)))?;
                let encoded = base64::encode(bytes);
                Ok(RenderOutput::Rendered(encoded))
            }
        }
    }

    // Загружаем шаблон — берём карту из звёздного архива!
    fn load_template(&self, default_path: &str) -> Result<String, RenderError> {
        let path = self.template.as_ref().map(|s| s.as_str()).unwrap_or(default_path);
        read_to_string(path)
            .map_err(|e| RenderError::FileError(format!("Не могу найти '{}': {}", path, e)))
    }

    // Рендерим шаблон — превращаем карту в звёздный путь с защитой от зацикливания!
    fn render_template(&self, template: &str, data: Vec<HashMap<String, String>>, included: &mut HashSet<String>) -> Result<String, RenderError> {
        let tokens = self.parse_template(template);
        let mut output = String::new();
        let mut stack = Vec::new(); // Стек для циклов и условий!

        for token in tokens {
            match token {
                TemplateToken::Text(text) => output.push_str(&text), // Просто текст — в космос!
                TemplateToken::Variable(var) => {
                    if let Some(row) = data.first() {
                        if let Some(value) = row.get(&var) {
                            output.push_str(value); // Нашли данные — добавляем!
                        }
                    }
                }
                TemplateToken::ForStart(item_name, list_name) => {
                    stack.push(StackItem::ForLoop(item_name, list_name, output.len())); // Запоминаем начало цикла!
                }
                TemplateToken::ForEnd => {
                    if let Some(StackItem::ForLoop(item_name, list_name, start_pos)) = stack.pop() {
                        let loop_content = output[start_pos..].to_string(); // Копируем в String
                        output.truncate(start_pos);
                        for row in &data {
                            if let Some(list) = row.get(&list_name) {
                                for item in list.split(',') {
                                    let mut temp = loop_content.clone();
                                    temp = temp.replace(&format!("{{ {} }}", item_name), item);
                                    output.push_str(&temp); // Повторяем для каждого элемента!
                                }
                            }
                        }
                    }
                }
                TemplateToken::IfStart(condition) => {
                    stack.push(StackItem::IfCond(condition, output.len())); // Запоминаем начало условия!
                }
                TemplateToken::Else => {
                    if let Some(StackItem::IfCond(condition, start_pos)) = stack.last_mut() {
                        let if_content = output[*start_pos..].to_string(); // Копируем в String
                        output.truncate(*start_pos);
                        if let Some(row) = data.first() {
                            if row.get(condition).map(|v| v == "true").unwrap_or(false) {
                                output.push_str(&if_content); // Условие истинно!
                            }
                        }
                        *start_pos = output.len(); // Перемещаем для else!
                    }
                }
                TemplateToken::IfEnd => {
                    if let Some(StackItem::IfCond(condition, start_pos)) = stack.pop() {
                        let else_content = output[start_pos..].to_string(); // Копируем в String
                        output.truncate(start_pos);
                        if let Some(row) = data.first() {
                            if !row.get(&condition).map(|v| v == "true").unwrap_or(false) {
                                output.push_str(&else_content); // Условие ложно — берём else!
                            }
                        }
                    }
                }
                TemplateToken::Include(file) => {
                    // Проверяем, не включали ли этот файл раньше!
                    if included.contains(&file) {
                        println!("Осторожно! '{}' уже включён — пропускаем, чтобы не зациклиться!", file);
                        continue; // Пропускаем, чтобы избежать бесконечной рекурсии!
                    }
                    included.insert(file.clone()); // Добавляем в список включённых!
                    let include_content = read_to_string(&file)
                        .map_err(|e| RenderError::FileError(format!("Не могу включить '{}': {}", file, e)))?;
                    let rendered_include = self.render_template(&include_content, data.clone(), included)?;
                    output.push_str(&rendered_include);
                }
            }
        }
        Ok(output)
    }

    // Парсим шаблон — разбиваем карту на звёздные куски!
    fn parse_template(&self, template: &str) -> Vec<TemplateToken> {
        let mut tokens = Vec::new();
        let mut remaining = template;
        let re = regex::Regex::new(r"(\{\{.*?\}\}|\{%.*?%\})").unwrap();

        while let Some(mat) = re.find(remaining) {
            let before = &remaining[..mat.start()];
            if !before.is_empty() {
                tokens.push(TemplateToken::Text(before.to_string()));
            }

            let token_str = mat.as_str();
            if token_str.starts_with("{{") && token_str.ends_with("}}") {
                let var = token_str[2..token_str.len() - 2].trim().to_string();
                tokens.push(TemplateToken::Variable(var));
            } else if token_str.starts_with("{%") && token_str.ends_with("%}") {
                let content = token_str[2..token_str.len() - 2].trim();
                if content.starts_with("for ") {
                    let parts: Vec<&str> = content[4..].split(" in ").collect();
                    if parts.len() == 2 {
                        let item_name = parts[0].trim().to_string();
                        let list_name = parts[1].trim().to_string();
                        tokens.push(TemplateToken::ForStart(item_name, list_name));
                    }
                } else if content == "endfor" {
                    tokens.push(TemplateToken::ForEnd);
                } else if content.starts_with("if ") {
                    let condition = content[3..].trim().to_string();
                    tokens.push(TemplateToken::IfStart(condition));
                } else if content == "else" {
                    tokens.push(TemplateToken::Else);
                } else if content == "endif" {
                    tokens.push(TemplateToken::IfEnd);
                } else if content.starts_with("include ") {
                    let file = content[8..].trim().to_string();
                    tokens.push(TemplateToken::Include(file));
                }
            }

            remaining = &remaining[mat.end()..];
        }
        if !remaining.is_empty() {
            tokens.push(TemplateToken::Text(remaining.to_string()));
        }
        tokens
    }
}

// Результат рендера — звёздный груз!
pub enum RenderOutput {
    Rendered(String),                  // Готовая строка — миссия выполнена!
    Raw(Option<Vec<HashMap<String, String>>>), // Сырые данные — для смелых пилотов!
}
