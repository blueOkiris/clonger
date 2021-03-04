/*
 * This is the document view
 * It contains two boxes,
 * one for inputting plain text,
 * one for showing formatted text in real time
 */

using System;
using System.Threading;
using Gtk;
using Pango;
using Clonger.Domain;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class LanguageDocumentationView : VBox {
        private ScrolledWindow formatScroll;
        private TextView inputView;
        
        public LanguageDocumentationView() :
                base(true, (int) AppSettings.Margin) {
            addFormatView();
            addInput();
        }
        
        public string GetText() => inputView.Buffer.Text;
        public void SetText(string text) => inputView.Buffer.Text = text;
        
        private void addFormatView() {
            var formatFrame = new Frame("Format");
            formatScroll = new ScrolledWindow();
            formatScroll.VscrollbarPolicy = PolicyType.Automatic;
            formatScroll.HscrollbarPolicy = PolicyType.Automatic;
            formatFrame.Add(formatScroll);
            PackStart(formatFrame, true, true, AppSettings.Margin);
        }
        
        private void addInput() {
            var inputFrame = new Frame("Input");
            var inputScroll = new ScrolledWindow();
            inputScroll.VscrollbarPolicy = PolicyType.Automatic;
            inputScroll.HscrollbarPolicy = PolicyType.Never;
            
            inputView = new TextView();
            inputView.Buffer.Changed +=
                (object sender, EventArgs args) => {
                    if(formatScroll.Children.Length > 0) {
                        formatScroll.Remove(formatScroll.Child);
                    }
                    
                    var document = TextFormatter.CreateDocument(
                        inputView.Buffer.Text
                    );
                    var newFormatDisplay = documentToVBox(document);
                    
                    formatScroll.Add(newFormatDisplay);
                    AppWindow.Instance.ShowAll();
                };
            
            inputScroll.Add(inputView);
            inputFrame.Add(inputScroll);
            PackStart(inputFrame, true, true, AppSettings.Margin);
        }
        
        private VBox documentToVBox(Document document) {
            var container = new VBox(false, 0);
            
            TextView currTextView = null;
            foreach(var snippet in document.Snippets) {
                if(snippet is TextSnippet) {
                    if(currTextView == null) {
                        currTextView = new TextView();
                        currTextView.Editable = false;
                        container.PackStart(
                            currTextView, false, false, AppSettings.Margin
                        );
                    }
                    
                    var textTag = new TextTag(null);
                    textTag.Weight =
                        ((TextSnippet) snippet).IsBold ?
                            Weight.Bold :
                            Weight.Normal;
                    textTag.Underline =
                        ((TextSnippet) snippet).IsUnderlined ?
                            Underline.Single :
                            Underline.None;
                    textTag.Style =
                        ((TextSnippet) snippet).IsItalicized ?
                            Pango.Style.Italic :
                            Pango.Style.Normal;
                    switch(((TextSnippet) snippet).Style) {
                        case HeaderType.Title:
                            textTag.SizePoints = AppSettings.TitleDocFontSize;
                            break;
                        case HeaderType.Subtitle:
                            textTag.SizePoints = AppSettings.SubtitleDocFontSize;
                            break;
                        case HeaderType.Body:
                            textTag.SizePoints = AppSettings.DefaultDocFontSize;
                            break;
                    }
                    
                    var iter = currTextView.Buffer.EndIter;
                    
                    currTextView.Buffer.TagTable.Add(textTag);
                    currTextView.Buffer.InsertWithTags(
                        ref iter, ((TextSnippet) snippet).RawText, textTag
                    );
                } else if(snippet is ImageSnippet) {
                    currTextView = null;
                    var img = new Image(((ImageSnippet) snippet).ResourcePath);
                    container.PackStart(img, false, false, AppSettings.Margin);
                } else if(snippet is TableSnippet) {
                    currTextView = null;
                    
                    var tableSnip = (TableSnippet) snippet;
                    var numRows = tableSnip.Data.GetLength(0);
                    var numCols = tableSnip.Data.GetLength(1);
                    
                    var table = new Grid();
                    table.ColumnHomogeneous = false;
                    table.RowHomogeneous = false;
                    
                    for(int row = 0; row < numRows; row++) {
                        for(int col = 0; col < numCols; col++) {
                            var datumFrame = new Frame();
                            var datum = new TextView();
                            datum.Editable = false;
                            var bold = new TextTag(null);
                            bold.SizePoints = AppSettings.DefaultDocFontSize;
                            if(row == 0) {
                                // Header
                                bold.Weight = Weight.Bold;
                            }
                            datum.Buffer.TagTable.Add(bold);
                            var iter = datum.Buffer.EndIter;
                            datum.Buffer.InsertWithTags(
                                ref iter, tableSnip.Data[row, col].Trim(), bold
                            );
                            datumFrame.Add(datum);
                            
                            table.Attach(datumFrame, col, row, 1, 1);
                        }
                    }
                    
                    container.PackStart(
                        table, false, false, AppSettings.Margin
                    );
                }
            }
            
            return container;
        }
    }
}