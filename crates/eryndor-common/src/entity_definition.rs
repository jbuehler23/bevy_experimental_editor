use serde::{Deserialize, Serialize};

/// Custom field type for entity definitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType {
    Int { min: Option<i32>, max: Option<i32> },
    Float { min: Option<f32>, max: Option<f32> },
    String { max_length: Option<usize> },
    Bool,
    Enum { enum_id: u32 },
    Color,
    Point,
    Array { element_type: Box<FieldType> },
}

impl FieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldType::Int { .. } => "Int",
            FieldType::Float { .. } => "Float",
            FieldType::String { .. } => "String",
            FieldType::Bool => "Bool",
            FieldType::Enum { .. } => "Enum",
            FieldType::Color => "Color",
            FieldType::Point => "Point",
            FieldType::Array { .. } => "Array",
        }
    }
}

/// Custom field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub name: String,
    pub field_type: FieldType,
    pub default_value: FieldValue,
    pub description: Option<String>,
}

/// Field value - runtime value for a custom field
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldValue {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Enum(String),
    Color(String),  // hex color
    Point { x: f32, y: f32 },
    Array(Vec<FieldValue>),
    Null,
}

impl FieldValue {
    pub fn default_for_type(field_type: &FieldType) -> Self {
        match field_type {
            FieldType::Int { .. } => FieldValue::Int(0),
            FieldType::Float { .. } => FieldValue::Float(0.0),
            FieldType::String { .. } => FieldValue::String(String::new()),
            FieldType::Bool => FieldValue::Bool(false),
            FieldType::Enum { .. } => FieldValue::Null,
            FieldType::Color => FieldValue::Color("#FFFFFF".to_string()),
            FieldType::Point => FieldValue::Point { x: 0.0, y: 0.0 },
            FieldType::Array { .. } => FieldValue::Array(Vec::new()),
        }
    }
}

/// Entity definition - custom entity class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefinitionData {
    pub id: u32,
    pub identifier: String,
    pub width: u32,
    pub height: u32,
    pub color: String,  // hex color for editor visualization
    pub field_definitions: Vec<CustomField>,
}

impl EntityDefinitionData {
    pub fn new(id: u32, identifier: &str) -> Self {
        Self {
            id,
            identifier: identifier.to_string(),
            width: 16,
            height: 16,
            color: "#FF0000".to_string(),
            field_definitions: Vec::new(),
        }
    }

    pub fn with_field(mut self, name: &str, field_type: FieldType) -> Self {
        let default_value = FieldValue::default_for_type(&field_type);
        self.field_definitions.push(CustomField {
            name: name.to_string(),
            field_type,
            default_value,
            description: None,
        });
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Serialize field definitions to JSON string for SpacetimeDB storage
    pub fn field_definitions_json(&self) -> String {
        serde_json::to_string(&self.field_definitions).unwrap_or_else(|_| "[]".to_string())
    }

    /// Deserialize field definitions from JSON string
    pub fn from_json_fields(mut self, json: &str) -> Self {
        if let Ok(fields) = serde_json::from_str(json) {
            self.field_definitions = fields;
        }
        self
    }
}

/// Entity instance - actual entity placement with field values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInstanceData {
    pub level_id: u32,
    pub entity_def_id: u32,
    pub x: f32,
    pub y: f32,
    pub field_values: std::collections::HashMap<String, FieldValue>,
}

impl EntityInstanceData {
    pub fn new(level_id: u32, entity_def_id: u32, x: f32, y: f32) -> Self {
        Self {
            level_id,
            entity_def_id,
            x,
            y,
            field_values: std::collections::HashMap::new(),
        }
    }

    pub fn with_field(mut self, name: &str, value: FieldValue) -> Self {
        self.field_values.insert(name.to_string(), value);
        self
    }

    /// Serialize field values to JSON string for SpacetimeDB storage
    pub fn field_values_json(&self) -> String {
        serde_json::to_string(&self.field_values).unwrap_or_else(|_| "{}".to_string())
    }

    /// Deserialize field values from JSON string
    pub fn from_json_values(mut self, json: &str) -> Self {
        if let Ok(values) = serde_json::from_str(json) {
            self.field_values = values;
        }
        self
    }
}

/// Enum definition - custom enum for entity fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDefinitionData {
    pub id: u32,
    pub identifier: String,
    pub values: Vec<String>,
}

impl EnumDefinitionData {
    pub fn new(id: u32, identifier: &str) -> Self {
        Self {
            id,
            identifier: identifier.to_string(),
            values: Vec::new(),
        }
    }

    pub fn with_values(mut self, values: Vec<&str>) -> Self {
        self.values = values.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Serialize values to JSON string for SpacetimeDB storage
    pub fn values_json(&self) -> String {
        serde_json::to_string(&self.values).unwrap_or_else(|_| "[]".to_string())
    }

    /// Deserialize values from JSON string
    pub fn from_json_values(mut self, json: &str) -> Self {
        if let Ok(values) = serde_json::from_str(json) {
            self.values = values;
        }
        self
    }
}
