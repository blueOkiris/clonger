/*
 * This is the document view
 * It contains two boxes,
 * one for inputting plain text,
 * one for showing formatted text in real time
 */

using System;
using System.Threading;
using Gtk;
using Clonger.Domain;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class LanguageDocumentationView : VBox {
        private VBox formatDisplay;
        private TextView inputView;
        
        public LanguageDocumentationView() :
                base(true, (int) AppSettings.Margin) {
            addFormatView();
            addInput();
        }
        
        private void addFormatView() {
            var formatFrame = new Frame("Format");
            var formatScroll = new ScrolledWindow();
            formatScroll.VscrollbarPolicy = PolicyType.Automatic;
            formatScroll.HscrollbarPolicy = PolicyType.Never;
            formatDisplay = new VBox(true, (int) AppSettings.Margin);
            formatScroll.Add(formatDisplay);
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
                    var formatThread = new Thread(new ThreadStart(
                        () => {
                            foreach(var child in formatDisplay.Children) {
                                formatDisplay.Remove(child);
                                child.Dispose();
                            }
                            
                            var document = TextFormatter.CreateDocument(
                                inputView.Buffer.Text
                            );
                        }
                    ));
                    formatThread.Start();
                };
            
            inputScroll.Add(inputView);
            inputFrame.Add(inputScroll);
            PackStart(inputFrame, true, true, AppSettings.Margin);
        }
    }
}