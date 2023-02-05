use phf::phf_map;

pub static FILE_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
    // Rust
    "rs" => "Rust",

    // Haskell
    "hs" => "Haskell",

    // C
    "c" => "C",

    // Cpp
    "cpp" => "Cpp",
    "cc" => "Cpp",
    "hpp" => "Cpp",

    // Python
    "py" => "Python",
    "pyi" => "Python",

    // Image
    "png" => "Image",
    "jpg" => "Image",
    "gif" => "Image",

    // Text
    "txt" => "Text",

    // Toml
    "toml" => "Toml",
    "lock" => "Toml",

    // Markdown
    "md" => "Markdown",

    // Json
    "json" => "Json",

    // Object
    "o" => "Object",
    "obj" => "Object",
};
