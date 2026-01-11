//! Test fixture generator for PDF Reader tests
//! 
//! This module creates test PDF files for use in unit and property tests.
//! Run with: cargo test --test generate_fixtures -- --ignored

use lopdf::{Document, Object, Dictionary, Stream, StringFormat};
use lopdf::content::{Content, Operation};
use std::path::Path;

const FIXTURES_DIR: &str = "tests/fixtures";

/// Create a simple single-page PDF with known text content
fn create_simple_pdf() -> Document {
    let mut doc = Document::with_version("1.5");
    
    // Create font dictionary
    let font_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Font".to_vec())),
        ("Subtype", Object::Name(b"Type1".to_vec())),
        ("BaseFont", Object::Name(b"Helvetica".to_vec())),
    ]));
    
    // Create resources dictionary
    let resources_id = doc.add_object(Dictionary::from_iter(vec![
        ("Font", Dictionary::from_iter(vec![
            ("F1", Object::Reference(font_id)),
        ]).into()),
    ]));
    
    // Create content stream with text
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Integer(12)]),
            Operation::new("Td", vec![Object::Integer(100), Object::Integer(700)]),
            Operation::new("Tj", vec![Object::String(b"Hello, this is a simple test PDF.".to_vec(), StringFormat::Literal)]),
            Operation::new("ET", vec![]),
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Integer(12)]),
            Operation::new("Td", vec![Object::Integer(100), Object::Integer(680)]),
            Operation::new("Tj", vec![Object::String(b"This PDF contains known text content for testing.".to_vec(), StringFormat::Literal)]),
            Operation::new("ET", vec![]),
        ],
    };
    
    let content_data = content.encode().unwrap();
    let content_id = doc.add_object(Stream::new(Dictionary::new(), content_data));
    
    // Create page dictionary
    let page_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Page".to_vec())),
        ("MediaBox", vec![0.into(), 0.into(), 612.into(), 792.into()].into()),
        ("Resources", Object::Reference(resources_id)),
        ("Contents", Object::Reference(content_id)),
    ]));
    
    // Create pages dictionary
    let pages_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Pages".to_vec())),
        ("Kids", vec![Object::Reference(page_id)].into()),
        ("Count", Object::Integer(1)),
    ]));
    
    // Update page to reference parent
    if let Ok(page) = doc.get_object_mut(page_id) {
        if let Object::Dictionary(dict) = page {
            dict.set("Parent", Object::Reference(pages_id));
        }
    }
    
    // Create catalog
    let catalog_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    
    // Create info dictionary with metadata
    let info_id = doc.add_object(Dictionary::from_iter(vec![
        ("Title", Object::String(b"Simple Test PDF".to_vec(), StringFormat::Literal)),
        ("Author", Object::String(b"Test Author".to_vec(), StringFormat::Literal)),
        ("Subject", Object::String(b"Testing PDF Reader".to_vec(), StringFormat::Literal)),
        ("Creator", Object::String(b"PDF Reader Test Suite".to_vec(), StringFormat::Literal)),
    ]));
    
    // Set trailer
    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.trailer.set("Info", Object::Reference(info_id));
    
    doc
}

/// Create a multi-page PDF with different content on each page
fn create_multi_page_pdf() -> Document {
    let mut doc = Document::with_version("1.5");
    
    // Create font dictionary
    let font_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Font".to_vec())),
        ("Subtype", Object::Name(b"Type1".to_vec())),
        ("BaseFont", Object::Name(b"Helvetica".to_vec())),
    ]));
    
    // Create resources dictionary
    let resources_id = doc.add_object(Dictionary::from_iter(vec![
        ("Font", Dictionary::from_iter(vec![
            ("F1", Object::Reference(font_id)),
        ]).into()),
    ]));
    
    let page_texts = [
        "Page 1: Introduction to the document.",
        "Page 2: Main content section.",
        "Page 3: Conclusion and summary.",
    ];
    
    let mut page_ids = Vec::new();
    
    for (i, text) in page_texts.iter().enumerate() {
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Integer(14)]),
                Operation::new("Td", vec![Object::Integer(100), Object::Integer(700)]),
                Operation::new("Tj", vec![Object::String(text.as_bytes().to_vec(), StringFormat::Literal)]),
                Operation::new("ET", vec![]),
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Integer(10)]),
                Operation::new("Td", vec![Object::Integer(100), Object::Integer(680)]),
                Operation::new("Tj", vec![Object::String(format!("This is page {} of 3.", i + 1).into_bytes(), StringFormat::Literal)]),
                Operation::new("ET", vec![]),
            ],
        };
        
        let content_data = content.encode().unwrap();
        let content_id = doc.add_object(Stream::new(Dictionary::new(), content_data));
        
        let page_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Page".to_vec())),
            ("MediaBox", vec![0.into(), 0.into(), 612.into(), 792.into()].into()),
            ("Resources", Object::Reference(resources_id)),
            ("Contents", Object::Reference(content_id)),
        ]));
        
        page_ids.push(page_id);
    }
    
    // Create pages dictionary
    let pages_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Pages".to_vec())),
        ("Kids", page_ids.iter().map(|id| Object::Reference(*id)).collect::<Vec<_>>().into()),
        ("Count", Object::Integer(page_ids.len() as i64)),
    ]));
    
    // Update pages to reference parent
    for page_id in &page_ids {
        if let Ok(page) = doc.get_object_mut(*page_id) {
            if let Object::Dictionary(dict) = page {
                dict.set("Parent", Object::Reference(pages_id));
            }
        }
    }
    
    // Create catalog
    let catalog_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    
    // Create info dictionary with metadata
    let info_id = doc.add_object(Dictionary::from_iter(vec![
        ("Title", Object::String(b"Multi-Page Test PDF".to_vec(), StringFormat::Literal)),
        ("Author", Object::String(b"Test Author".to_vec(), StringFormat::Literal)),
        ("Subject", Object::String(b"Testing Multi-Page PDF Reader".to_vec(), StringFormat::Literal)),
        ("Creator", Object::String(b"PDF Reader Test Suite".to_vec(), StringFormat::Literal)),
    ]));
    
    // Set trailer
    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.trailer.set("Info", Object::Reference(info_id));
    
    doc
}

/// Generate all test fixtures
pub fn generate_all_fixtures() -> std::io::Result<()> {
    let fixtures_path = Path::new(FIXTURES_DIR);
    std::fs::create_dir_all(fixtures_path)?;
    
    // Generate simple.pdf
    let mut simple_pdf = create_simple_pdf();
    simple_pdf.save(fixtures_path.join("simple.pdf"))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    println!("Created: tests/fixtures/simple.pdf");
    
    // Generate multi-page.pdf
    let mut multi_page_pdf = create_multi_page_pdf();
    multi_page_pdf.save(fixtures_path.join("multi-page.pdf"))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    println!("Created: tests/fixtures/multi-page.pdf");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Run manually with: cargo test --test generate_fixtures -- --ignored
    fn generate_test_fixtures() {
        generate_all_fixtures().expect("Failed to generate test fixtures");
    }
}

fn main() {
    generate_all_fixtures().expect("Failed to generate test fixtures");
}
