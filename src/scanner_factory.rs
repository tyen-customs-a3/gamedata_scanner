use std::sync::Arc;
use models::Scanner as ClassScanner;
use models::FileParser;

/// Returns a scanner implementation based on the scanner type 
pub fn get_scanner(advanced: bool) -> Arc<dyn ClassScanner> {
    if advanced {
        Arc::new(::parser_advanced::scanner::AdvancedScanner::new())
    } else {
        Arc::new(::parser_simple::scanner::SimpleScanner::new())
    }
}

/// Returns a file parser implementation based on the parser type
pub fn get_parser(advanced: bool) -> Arc<dyn FileParser> {
    if advanced {
        Arc::new(::parser_advanced::AdvancedFileParser::new())
    } else {
        Arc::new(::parser_simple::SimpleFileParser::new())
    }
}