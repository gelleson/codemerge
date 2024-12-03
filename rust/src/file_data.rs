use serde::{Serialize};

/// Represents the data structure that holds metadata about a file.
///
/// This structure is utilized for encapsulating information about a file,
/// which includes its path, content, token count, and any errors.
/// It can be serialized using Serde for ease of transformation into formats like JSON.
///
/// # Fields
/// - `path`: A `String` that specifies the path of the file in the filesystem.
///   This is expected to be an absolute or relative path to where the file is located.
/// - `content`: An `Option<String>` that contains the file's content. This may be `None`
///   if, for example, the file's content is not loaded or an error occurred during reading.
/// - `tokens`: A `usize` representing the number of tokens in the file. This field might be
///   used, for example, in scenarios involving processing or analyzing text, to keep track
///   of its length in terms of lexical tokens.
/// - `error`: An `Option<String>` that contains any error messages encountered while handling the file.
///   If file operations such as reading or parsing fail, this field will include the respective error message.
#[derive(Serialize)]
pub struct FileData {
    /// The location of the file as a string path.
    pub path: String,

    /// Optional content of the file. `None` indicates unread or inaccessible content.
    pub content: Option<String>,

    /// Token count in the content of the file.
    pub tokens: usize,

    /// Optional error message. `None` implies no errors encountered.
    pub error: Option<String>,
}