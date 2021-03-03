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
    
    struct Document {
        public List<DocumentSnippet> Snippets;
    }
}
