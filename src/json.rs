use std::string::ToString;

// false / null / true / object / array / number / string
enum JsonValueType {
    Null(JsonNull),
    Boolean(JsonBoolean),
    Number(JsonNumber),
    String(JsonSting),
    Object(JsonObject),
    Array(JsonArray),
}

impl From<JsonNull> for JsonValueType {
    fn from(null: JsonNull) -> Self {
        JsonValueType::Null(null)
    }
}

impl From<JsonBoolean> for JsonValueType {
    fn from(b: JsonBoolean) -> Self {
        JsonValueType::Boolean(b)
    }
}

impl From<JsonNumber> for JsonValueType {
    fn from(number: JsonNumber) -> Self {
        JsonValueType::Number(number)
    }
}

impl From<JsonSting> for JsonValueType {
    fn from(string: JsonSting) -> Self {
        JsonValueType::String(string)
    }
}

impl From<JsonObject> for JsonValueType {
    fn from(obj: JsonObject) -> Self {
        JsonValueType::Object(obj)
    }
}

impl From<JsonArray> for JsonValueType {
    fn from(arr: JsonArray) -> Self {
        JsonValueType::Array(arr)
    }
}

impl ToString for JsonValueType {
    fn to_string(&self) -> String {
        match self {
            JsonValueType::Null(it) => it.to_string(),
            JsonValueType::Boolean(it) => it.to_string(),
            JsonValueType::Number(it) => it.to_string(),
            JsonValueType::String(it) => it.to_string(),
            JsonValueType::Object(it) => it.to_string(),
            JsonValueType::Array(it) => it.to_string(),
        }
    }
}

struct JsonNull {}

impl ToString for JsonNull {
    fn to_string(&self) -> String {
        "null".to_owned()
    }
}

struct JsonBoolean {
    value: bool
}

impl ToString for JsonBoolean {
    fn to_string(&self) -> String {
        match self.value {
            true => "true",
            false => "false",
        }.to_owned()
    }
}

struct JsonNumber {
    value: i64
}

impl ToString for JsonNumber {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

struct JsonSting {
    value: String
}

impl ToString for JsonSting {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

struct JsonObject {
    name: String,
    value: Box<JsonValueType>,
}

impl ToString for JsonObject {
    fn to_string(&self) -> String {
        format!("{{\"{}\": {}}}", self.name, self.value.to_string())
    }
}

struct JsonArray {}

impl ToString for JsonArray {
    fn to_string(&self) -> String {
        format!("[]")
    }
}

struct Json {
    root: JsonValueType
}

impl Json {
    fn from_string(source: String) {}

    fn to_string(&self) -> String {
        self.root.to_string()
    }
}


#[test]
fn test_to_string() {
    let json = Json {
        root: JsonObject {
            name: "hello".to_owned(),
            value: Box::new(JsonObject {
                name: "world".to_owned(),
                value: JsonValueType::Array(JsonArray {}).into(),
            }.into()),
        }.into()
    };

    println!("{}", json.to_string())
}