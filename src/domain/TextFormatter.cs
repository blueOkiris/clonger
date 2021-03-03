/*
 * Domain code for document view
 * Takes string input and constructs a Document object
 */

using System;
using System.Text;
using System.Collections.Generic;
using Clonger.Data;

namespace Clonger.Domain {
    static class TextFormatter {
        public static Document CreateDocument(string inputText) {
            var doc = new Document { Snippets = new List<DocumentSnippet>() };
            
            /*
             * Split up between regular text and special text
             * There's no recursion, just multiple attributes
             *
             * blah blah blah [bold,italic:blah blah] blah blah
             * [table,5,5:a,b,c,d,e...]
             * [image:/home/...]
             */
            var sections = new List<string>();
            var currStr = new StringBuilder();
            for(int i = 0; i < inputText.Length; i++) {
                switch(inputText[i]) {
                    case '[':
                        if(!currStr.ToString().StartsWith("[")
                                && currStr.ToString() != "") {
                            sections.Add(currStr.ToString());
                            currStr.Clear();
                        }
                        currStr.Append(inputText[i]);
                        break;
                    
                    case ']':
                        currStr.Append(inputText[i]);
                        if(currStr.ToString().StartsWith("[")) {
                            sections.Add(currStr.ToString());
                            currStr.Clear();
                        }
                        break;
                    
                    case '\\':
                        if(i + 1 < inputText.Length) {
                            currStr.Append(inputText[i++]);
                            currStr.Append(inputText[i]);
                        }
                        break;
                        
                    default:
                        currStr.Append(inputText[i]);
                        break;
                }
            }
            
            // Parse the sections
            foreach(var section in sections) {
                doc.Snippets.Add(parseSnippet(section));
            }
            
            return doc;
        }
        
        private static DocumentSnippet parseSnippet(string section) {
            if(!section.StartsWith("[")) {
                return new TextSnippet {
                    IsBold = false, IsItalicized = false, IsUnderlined = false,
                    Style = HeaderType.Body,
                    RawText = section
                };
            }
            
            // Split between formatting and raw text
            var sections = section.Substring(1, section.Length - 2).Split(':');
            if(sections.Length < 2) {
                Console.WriteLine("Format has too few sections!");
                return new TextSnippet {
                    IsBold = false, IsItalicized = false, IsUnderlined = false,
                    Style = HeaderType.Body,
                    RawText = section
                };
            } else if(sections.Length > 2) {
                var newSec1 = new StringBuilder(sections[1]);
                int i = 2;
                while(i < sections.Length) {
                    newSec1.Append(sections[i++]);
                }
                sections[1] = newSec1.ToString();
            }
            
            // Split formatting into options
            var formatting = sections[0].Split(',');
            switch(formatting[0].Trim()) {
                case "table": {
                    if(formatting.Length != 3) {
                        Console.WriteLine(
                            "Wrong number of formatting options in table!"
                        );
                        return new TextSnippet {
                            IsBold = false,
                            IsItalicized = false,
                            IsUnderlined = false,
                            Style = HeaderType.Body,
                            RawText = section
                        };
                    }
                    
                    int numRows = -1;
                    int numCols = -1;
                    if(!int.TryParse(formatting[1], out numRows)) {
                        Console.WriteLine(
                            "Invalid row count in table definition!"
                        );
                        return new TextSnippet {
                            IsBold = false,
                            IsItalicized = false,
                            IsUnderlined = false,
                            Style = HeaderType.Body,
                            RawText = section
                        };
                    }
                    if(!int.TryParse(formatting[2], out numCols)) {
                        Console.WriteLine(
                            "Invalid column count in table definition!"
                        );
                        return new TextSnippet {
                            IsBold = false,
                            IsItalicized = false,
                            IsUnderlined = false,
                            Style = HeaderType.Body,
                            RawText = section
                        };
                    }
                    
                    var data = sections[1].Split(',');
                    var tableData = new string[numRows, numCols];
                    for(int row = 0; row < numRows; row++) {
                        for(int col = 0; col < numCols; col++) {
                            int index = row * numCols + col;
                            if(index < data.Length) {
                                tableData[row, col] = data[index];
                            } else {
                                tableData[row, col] = "";
                            }
                        }
                    }
                    
                    return new TableSnippet {
                        Data = tableData
                    };
                }
                
                case "image":
                    return new ImageSnippet { ResourcePath = sections[1] };
                
                default: {
                    var bold = false;
                    var italic = false;
                    var underline = false;
                    var style = HeaderType.Body;
                    foreach(var formatOption in formatting) {
                        switch(formatOption) {
                            case "bold":
                                bold = true;
                                break;
                            
                            case "italicized":
                                italic = true;
                                break;
                            
                            case "underlined":
                                underline = true;
                                break;
                            
                            case "title":
                                style = HeaderType.Title;
                                break;
                            
                            case "subtitle":
                                style = HeaderType.Subtitle;
                                break;
                            
                            default:
                                Console.WriteLine(
                                    "Unknown option '{0}'. Skipping...",
                                    formatOption
                                );
                                break;
                        }
                    }
                    return new TextSnippet {
                        IsBold = bold,
                        IsItalicized = italic,
                        IsUnderlined = underline,
                        Style = style,
                        RawText = sections[1]
                    };
                }
            }
        }
    }
}
