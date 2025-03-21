use anyhow::{Context, Result};
use clap::Parser;
use pulldown_cmark::{Event, Options, Parser as MarkdownParser, Tag};
use printpdf::*;
use std::fs;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input markdown file
    #[arg(short, long)]
    input: PathBuf,

    /// Output PDF file
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read markdown file
    let markdown_content = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {}", args.input.display()))?;

    // Parse markdown
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    
    let parser = MarkdownParser::new_ext(&markdown_content, options);

    // Create PDF document
    let (doc, page1, layer1) = PdfDocument::new("Markdown Document", 
        Mm(210.0), // A4 width
        Mm(297.0), // A4 height
        "Layer 1"
    );

    // Create a new layer for the content
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Add text to the PDF with proper formatting
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold_font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    
    let mut y_pos = Mm(287.0); // Start from top with margin
    let line_height = Mm(14.0);
    let mut list_indent: u32 = 0;
    let mut current_text = String::new();
    let mut is_bold = false;
    let mut list_counter = 0;
    let mut is_ordered_list = false;
    let mut in_list_item = false;

    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Heading(level, _, _) => {
                        if !current_text.is_empty() {
                            current_layer.use_text(&current_text, 12.0, Mm(10.0), y_pos, &font);
                            current_text.clear();
                            y_pos -= line_height;
                        }
                    }
                    Tag::Strong => {
                        is_bold = true;
                    }
                    Tag::List(Some(_)) => {
                        is_ordered_list = true;
                        list_indent += 1;
                        list_counter = 0;
                    }
                    Tag::List(None) => {
                        is_ordered_list = false;
                        list_indent += 1;
                    }
                    Tag::Item => {
                        in_list_item = true;
                        if !current_text.is_empty() {
                            let indent = Mm(10.0 + (list_indent as f32 * 10.0));
                            let prefix = if is_ordered_list {
                                list_counter += 1;
                                format!("{}. ", list_counter)
                            } else {
                                "• ".to_string()
                            };
                            current_layer.use_text(&format!("{}{}", prefix, current_text), 12.0, indent, y_pos, &font);
                            current_text.clear();
                            y_pos -= line_height;
                        }
                    }
                    _ => {}
                }
            }
            Event::End(tag) => {
                match tag {
                    Tag::Paragraph => {
                        if !current_text.is_empty() && !in_list_item {
                            current_layer.use_text(&current_text, 12.0, Mm(10.0), y_pos, &font);
                            current_text.clear();
                            y_pos -= line_height;
                        }
                    }
                    Tag::Strong => {
                        is_bold = false;
                    }
                    Tag::List(_) => {
                        list_indent = list_indent.saturating_sub(1);
                    }
                    Tag::Item => {
                        in_list_item = false;
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                current_text.push_str(&text);
            }
            Event::SoftBreak => {
                current_text.push(' ');
            }
            Event::HardBreak => {
                if !current_text.is_empty() {
                    current_layer.use_text(&current_text, 12.0, Mm(10.0), y_pos, &font);
                    current_text.clear();
                    y_pos -= line_height;
                }
            }
            Event::TaskListMarker(checked) => {
                let marker = if checked { "☒ " } else { "☐ " };
                current_text.push_str(marker);
            }
            _ => {}
        }
    }

    // Save the PDF
    let file = fs::File::create(&args.output)
        .with_context(|| format!("Failed to create output file: {}", args.output.display()))?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)
        .with_context(|| format!("Failed to save PDF to: {}", args.output.display()))?;

    println!("Successfully converted {} to {}", args.input.display(), args.output.display());
    Ok(())
}
