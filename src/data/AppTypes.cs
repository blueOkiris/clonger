/*
 * Storage for custom types used
 * for transferring large types of data around the app
 * 
 * Includes enums and record types
 */
using System.Collections.Generic;

namespace Clonger.Data {
    enum ViewType { IpaKeyboard, Document, Dictionary, Examples }
    enum HeaderType { Title, Subtitle, Body }
    
    /*
     * The following pieces of data are used to construct a document
     * It isn't necessary to do it this way,
     * as some of this stuff is structs with one piece of data
     *
     * However, I felt this was a pleasant way to internally represent the data
     */
     
    // Snippet of text
    interface DocumentSnippet { }

    // Snippet of document with formatting
    struct TextSnippet : DocumentSnippet {
        public string RawText;
        public bool IsBold, IsItalicized, IsUnderlined;
        public HeaderType Style;
    }
    
    // Snippet of document that forms a table
    struct TableSnippet : DocumentSnippet {
        public string[,] Data;
    }
    
    // Image
    struct ImageSnippet : DocumentSnippet {
        public string ResourcePath;
    }
    
    // Formatted document for showing grammar and stuff
    struct Document {
        public List<DocumentSnippet> Snippets;
    }
    
    // Representation of an example
    struct Example {
        public string Romanization;
        public string Ipa;
        public string Gloss;
        public string Translation;
    }
    
    // Dictionary entry for conlang's word
    struct WordEntry {
        public string Word;
        public string Translation;
        public string PartOfSpeech;
        public string Additional;
    }
}
