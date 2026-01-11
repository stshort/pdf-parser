//! PDF reading and parsing module

use crate::error::PdfError;
use lopdf::Document;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// PDF document metadata and information
#[derive(Debug, Serialize, Deserialize)]
pub struct PdfInfo {
    pub page_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

/// PDF Reader for extracting text and metadata from PDF files
pub struct PdfReader;

impl PdfReader {
    /// Load a PDF document from a file path
    fn load_document(file_path: &str) -> Result<Document, PdfError> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Err(PdfError::FileNotFound(file_path.to_string()));
        }
        
        let doc = Document::load(file_path).map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("encrypted") || err_str.contains("password") {
                PdfError::EncryptedDocument
            } else if err_str.contains("invalid") || err_str.contains("Invalid") {
                PdfError::InvalidFormat(err_str)
            } else {
                PdfError::ParseError(err_str)
            }
        })?;
        
        // Check if document is encrypted and we couldn't decrypt it
        if doc.is_encrypted() && doc.encryption_state.is_none() {
            return Err(PdfError::EncryptedDocument);
        }
        
        Ok(doc)
    }

    /// Extract all text from a PDF file
    /// Extracts text page-by-page, skipping pages that fail to parse.
    /// Returns the concatenated text from all successfully parsed pages.
    pub fn extract_text(file_path: &str) -> Result<String, PdfError> {
        let doc = Self::load_document(file_path)?;
        
        let pages = doc.get_pages();
        if pages.is_empty() {
            return Ok(String::new());
        }
        
        let mut all_text = String::new();
        let mut skipped_pages = Vec::new();
        
        // Extract page-by-page to handle problematic pages gracefully
        for page_num in pages.keys() {
            match doc.extract_text(&[*page_num]) {
                Ok(text) => {
                    if !all_text.is_empty() && !text.is_empty() {
                        all_text.push('\n');
                    }
                    all_text.push_str(&text);
                }
                Err(_) => {
                    skipped_pages.push(*page_num);
                }
            }
        }
        
        // Add a note about skipped pages if any
        if !skipped_pages.is_empty() {
            all_text.push_str(&format!(
                "\n\n[Note: {} page(s) could not be extracted due to font encoding issues: {:?}]",
                skipped_pages.len(),
                skipped_pages
            ));
        }
        
        Ok(all_text)
    }

    /// Extract text from a specific page (1-indexed)
    pub fn extract_page_text(file_path: &str, page: u32) -> Result<String, PdfError> {
        let doc = Self::load_document(file_path)?;
        
        let pages = doc.get_pages();
        let page_count = pages.len();
        
        if page < 1 || page as usize > page_count {
            return Err(PdfError::PageNotFound(page, page_count));
        }
        
        let text = doc.extract_text(&[page]).map_err(|e| {
            PdfError::ParseError(format!("Failed to extract text from page {}: {}", page, e))
        })?;
        
        Ok(text)
    }

    /// Extract text from a range of pages (1-indexed, inclusive)
    pub fn extract_page_range_text(file_path: &str, start_page: u32, end_page: u32) -> Result<String, PdfError> {
        let doc = Self::load_document(file_path)?;
        
        let pages = doc.get_pages();
        let page_count = pages.len();
        
        if start_page < 1 || start_page as usize > page_count {
            return Err(PdfError::PageNotFound(start_page, page_count));
        }
        
        if end_page < 1 || end_page as usize > page_count {
            return Err(PdfError::PageNotFound(end_page, page_count));
        }
        
        if start_page > end_page {
            return Err(PdfError::ParseError(format!(
                "Invalid page range: start_page ({}) must be <= end_page ({})",
                start_page, end_page
            )));
        }
        
        let mut all_text = String::new();
        let mut skipped_pages = Vec::new();
        
        for page_num in start_page..=end_page {
            match doc.extract_text(&[page_num]) {
                Ok(text) => {
                    if !all_text.is_empty() && !text.is_empty() {
                        all_text.push('\n');
                    }
                    all_text.push_str(&text);
                }
                Err(_) => {
                    skipped_pages.push(page_num);
                }
            }
        }
        
        if !skipped_pages.is_empty() {
            all_text.push_str(&format!(
                "\n\n[Note: {} page(s) could not be extracted due to font encoding issues: {:?}]",
                skipped_pages.len(),
                skipped_pages
            ));
        }
        
        Ok(all_text)
    }

    /// Get PDF metadata and page count
    pub fn get_info(file_path: &str) -> Result<PdfInfo, PdfError> {
        let doc = Self::load_document(file_path)?;
        
        let pages = doc.get_pages();
        let page_count = pages.len();
        
        // Try to get the Info dictionary from the trailer
        let (title, author, subject, creator) = Self::extract_metadata(&doc);
        
        Ok(PdfInfo {
            page_count,
            title,
            author,
            subject,
            creator,
        })
    }
    
    /// Extract metadata from the document's Info dictionary
    fn extract_metadata(doc: &Document) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        // Get the Info dictionary reference from the trailer
        let info_ref = match doc.trailer.get(b"Info") {
            Ok(obj) => obj,
            Err(_) => return (None, None, None, None),
        };
        
        // Dereference to get the actual Info dictionary
        let info_dict = match info_ref.as_reference() {
            Ok(ref_id) => {
                match doc.get_dictionary(ref_id) {
                    Ok(dict) => dict,
                    Err(_) => return (None, None, None, None),
                }
            }
            Err(_) => {
                // Maybe it's a direct dictionary
                match info_ref.as_dict() {
                    Ok(dict) => dict,
                    Err(_) => return (None, None, None, None),
                }
            }
        };
        
        let title = Self::get_string_from_dict(info_dict, b"Title");
        let author = Self::get_string_from_dict(info_dict, b"Author");
        let subject = Self::get_string_from_dict(info_dict, b"Subject");
        let creator = Self::get_string_from_dict(info_dict, b"Creator");
        
        (title, author, subject, creator)
    }
    
    /// Helper to extract a string value from a dictionary
    fn get_string_from_dict(dict: &lopdf::Dictionary, key: &[u8]) -> Option<String> {
        dict.get(key).ok().and_then(|obj| {
            // decode_text_string takes an Object reference and returns Result<String>
            lopdf::decode_text_string(obj).ok()
        })
    }
}
